#!/usr/bin/env python3

import sys
import os


def main():
    print(sys.argv)
    kernel = sys.argv[1]
    kernel_bin = f'{kernel}.bin'

    objcopy = f'rust-objcopy --strip-all -Obinary {kernel} {kernel_bin}'
    print(objcopy)
    os.system(objcopy)

    args = ' '.join(sys.argv[2:])
    qemu = 'qemu-system-aarch64 -M raspi3b -smp 4 -m 1G -cpu cortex-a72 -serial stdio -monitor telnet:localhost:4444,server,nowait -gdb tcp::3333 -S -kernel'
    qemu = f'{qemu} {kernel_bin} {args}'
    print(qemu)
    os.system(qemu)

    pass

if __name__ == '__main__':
    main()
