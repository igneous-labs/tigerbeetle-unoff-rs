//! TODO: automatic splitting of batches for requests that exceed MAX_MSG_SIZE
//! TODO: query_accounts, query_transfers, get_account_balances

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use std::mem::MaybeUninit;

use num_traits::FromPrimitive;
use pin_project::pin_project;
use tigerbeetle_unoff_sys::{
    tb_client_submit, tb_client_t, tb_packet_t, TB_PACKET_STATUS_TB_PACKET_OK,
};

use crate::err::TbPacketErr;
use crate::resp::RespBuf;
use crate::Client;

pub mod create_accounts;
pub mod create_transfers;
pub mod get_account_transfers;
pub mod lookup_accounts;
pub mod lookup_transfers;

impl Client {
    /// Caveats:
    /// - the C client currently does nothing and never returns if the DB is unreachable (e.g. port closed).
    ///   This method should be at least raced with a `timeout()` fn to ensure recoverability from such a state.
    #[inline]
    pub async fn request(&self, packet: tb_packet_t) -> Result<RespBuf, TbPacketErr> {
        Req::new(self.ptr, packet).await
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum ReqState {
    /// Safety: need to ensure that client is not Dropped (deinit)
    /// before the first poll(). Therefore this enum should never leave scopes
    /// of [`crate::Client`] methods that take &self or &mut self
    Init(tb_client_t),
    AwaitingResp(MaybeUninit<Waker>),
    Completed(MaybeUninit<RespBuf>),
}

impl ReqState {
    /// State transition Init -> AwaitingResp.
    ///
    /// Saves waker to use on_completion()
    fn init_to_awaiting_resp(&mut self, waker: Waker) {
        match self {
            Self::Init(_) => *self = Self::AwaitingResp(MaybeUninit::new(waker)),
            _ => panic!("Expected to be in Init state, got {self:?}"),
        }
    }

    /// State transition AwaitingResp -> Completed
    ///
    /// Consumes previously saved Waker to continue with Future
    fn awaiting_resp_to_completed(&mut self, data: *const u8, data_size: u32) {
        match self {
            Self::AwaitingResp(waker) => {
                let waker = unsafe { waker.assume_init_read() };
                // TODO: maybe can optimize here to not allocate if response failed.
                // This will also make it so we don't need to rmb to assume_init_drop() the RespBuf.
                // Need to ensure that failure responses will not set any data
                *self = Self::Completed(MaybeUninit::new(RespBuf::from_raw(data, data_size)));
                waker.wake();
                // waker dropped here, MaybeUninit does not double-drop
            }
            _ => panic!("Expected to be in AwaitingResp state, got {self:?}"),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
#[pin_project]
struct Req {
    packet: tb_packet_t,
    req_state: ReqState,
}

impl Req {
    fn new(client: tb_client_t, packet: tb_packet_t) -> Self {
        Self {
            packet,
            req_state: ReqState::Init(client),
        }
    }
}

impl Future for Req {
    type Output = Result<RespBuf, TbPacketErr>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this_addr: *const Self = self.as_ref().get_ref();
        let this = self.project();
        match this.req_state {
            ReqState::Init(client) => {
                let client = *client; // copy out the ptr
                this.req_state.init_to_awaiting_resp(cx.waker().clone());
                // TODO: this looks like UB-inducing since on_completion()
                // mutates self via this ptr even tho this is a
                // const ptr but im not sure how else to do it.
                //
                // Given that this fn exits almost immediately after calling tb_client_submit(),
                // on_completion() should have exclusive access to Req by the time its called
                // so maybe its ok?
                this.packet.user_data = this_addr.cast_mut().cast();
                unsafe {
                    tb_client_submit(client, this.packet);
                }
                Poll::Pending
            }
            // this branch should never be taken,
            // on_completion() should always change ReqState to Completed
            // before calling wake()
            ReqState::AwaitingResp(_) => Poll::Pending,
            ReqState::Completed(data) => Poll::Ready(
                if this.packet.status != TB_PACKET_STATUS_TB_PACKET_OK as u8 {
                    unsafe {
                        data.assume_init_drop();
                    }
                    // unwrap-safety: conversion fails only if tb server is buggy
                    Err(TbPacketErr::from_u8(this.packet.status).unwrap())
                } else {
                    // box moved out here, MaybeUninit does not double-drop
                    Ok(unsafe { data.assume_init_read() })
                },
            ),
        }
    }
}

/// Notes:
/// - this is also called when the client prematurely stops the request - e.g. on timeout. In this case, data = NULL, data_size = 0
pub(crate) extern "C" fn on_completion(
    _global_ctx: usize,
    _client: tb_client_t,
    packet: *mut tb_packet_t, // packet's user_data is a ptr to [`crate::Req`]
    data: *const u8,
    data_size: u32,
) {
    // race-condition safety: ensure that
    // nothing else on other threads can mutate Req while this fn is running
    let req: *mut Req = unsafe { *packet }.user_data.cast();
    unsafe { &mut *req }
        .req_state
        .awaiting_resp_to_completed(data, data_size);
}
