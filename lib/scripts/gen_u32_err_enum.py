import glob
import os
import re
import stringcase
import sys

KEYS = {
    "create_transfer": ("TB_CREATE_TRANSFER_RESULT", "TB_CREATE_TRANSFER", "CreateTransferErr"),
    "create_account": ("TB_CREATE_ACCOUNT_RESULT", "TB_CREATE_ACCOUNT", "CreateAccountErr"),
    "packet": ("TB_PACKET_STATUS", "TB_PACKET", "TbPacketErr"),
    "status": ("TB_STATUS", "TB_STATUS", "TbStatusErr"),
}

def usage():
    print("Usage: gen_u32_err_enum.py <key>")
    print("Assumes a debug build of tigerbeetle-unoff-sys has been built already")
    exit(0)

def script_dir():
    return os.path.dirname(os.path.realpath(__file__))

def lib_base_dir():
    return os.path.join(script_dir(), "../")

def workspace_dir():
    return os.path.join(lib_base_dir(), "../")

def bindings_rs():
    all_sys = glob.glob(os.path.join(workspace_dir(), "target", "debug", "build", "tigerbeetle-unoff-sys-*/"))
    latest = max(all_sys, key=os.path.getmtime)
    return os.path.join(latest, "out", "bindings.rs")

def enum_variant_name(shouty_snake):
    # stringcase.pascalcase doesnt really handle `_<number>` suffixes
    return stringcase.pascalcase(shouty_snake.lower()).replace("_", "")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        usage()
    key = sys.argv[1]
    entry = KEYS.get(key)
    if not entry:
        print(f"Invalid key '{key}'. Expected one of: {list(KEYS.keys())}")
        exit(1)
    (ty, prefix, enum_name) = entry
    regex = f"pub const {ty}_{prefix}_([^\s]+)\s*:"
    bindings = bindings_rs()
    print(f"Using {bindings}")
    
    with open(bindings, "r") as bindings:
        bindings = bindings.read()
        consts = re.findall(regex, bindings)
        with open(os.path.join(lib_base_dir(), "src", "err", f"{key}.rs"), "w") as dst:
            # double space is deleted by cargo fmt
            dst.write(f"/// Generated with scripts/gen_u32_err_enum.py {key}. DO NOT MODIFY MANUALLY.\n\n")

            dst.write("use num_derive::{FromPrimitive, ToPrimitive};\n")
            dst.write("use tigerbeetle_unoff_sys::*;\n\n")

            dst.write("#[repr(u32)]\n")
            dst.write("#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]\n")
            dst.write(f"pub enum {enum_name} {{\n")
            for c in consts:
                if c != "OK" and c != "SUCCESS":
                    dst.write(f"    {enum_variant_name(c)} = {ty}_{prefix}_{c},\n")
            dst.write("}\n")
