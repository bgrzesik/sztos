#!/bin/bash
lldb ./target/aarch64-unknown-none-softfloat/debug/sztos -o 'gdb-remote 127.0.0.1:3333' -o 'target modules load --file sztos --slide 0x0000000000000000'
