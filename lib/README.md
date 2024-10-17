# tigerbeetle-unoff

Unofficial tigerbeetle rust library

## Testing

### `live-test` feature

Setup a tigerbeetle cluster locally at port 3000 with the state initialized to what the tests expect before running `cargo test --features live-test`.

## Troubleshooting

There's always a whole bunch of sporadic linker errors for some reason such as

- `/usr/bin/ld: skipping incompatible /path/to/tigerbeetle-unoff-rs/sys/tigerbeetle/zig-out/lib/libtb_client.a when searching for -ltb_client`
- 'some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified'

For some reason it seems like the order of building the libraries seem to matter. For a clean build:

- make sure to turn off rust-analyzer so that it doesnt start the debug build immediately - it always seems to trigger using `ld` instead of `zcc` as linker for some reason.
- `cd sys && cargo test` - the sys crate must be built first
- now you can build normally and start rust-analyzer
