#!/bin/sh
# qemu-system-x86_64 -m 1024 -vnc 0.0.0.0:1 \
# 	-drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

# config `cargo r` in ./cargo/config.toml
# args behand `--` is qemu args
# see more detail in: https://github.com/rust-osdev/bootimage#running
# vnc port start from 5900, so 0.0.0.0:1 means 0.0.0.0:5901 
cargo r -- -m 1024 -vnc 0.0.0.0:1