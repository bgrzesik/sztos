#!/bin/bash
lldb ./target/aarch64-unknown-none/debug/sztos -o 'gdb-remote 127.0.0.1:3333'
