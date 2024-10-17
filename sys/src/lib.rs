#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // TODO: u128s are currently not FFI-safe, we need to include tests to make sure no weird behaviour happens

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::tb_account_balance_t;

    #[test]
    fn for_linker_detection() {
        let _a = tb_account_balance_t {
            debits_pending: 0,
            debits_posted: 0,
            credits_pending: 0,
            credits_posted: 0,
            timestamp: 0,
            reserved: [0; 56],
        };
    }
}
