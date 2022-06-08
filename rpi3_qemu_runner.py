#!/usr/bin/env python3

import sys
import os

QEMU = 'qemu-system-aarch64'

def main():
    machines = os.popen(f"{QEMU} -machine help | sed '1d' | awk -F' ' '{{ print $1; }}'").read()
    machines = machines.splitlines()

    machine = 'raspi3' if 'raspi3' in machines else 'raspi3b'

    kernel = sys.argv[1]
    kernel_bin = f'{kernel}.bin'

    objcopy = f'rust-objcopy --strip-all -Obinary {kernel} {kernel_bin}'
    print(objcopy)
    os.system(objcopy)

    args = ' '.join(sys.argv[2:])
    qemu = f'{QEMU} -M {machine} -smp 4 -m 1G -cpu cortex-a72 -serial stdio -monitor telnet:localhost:4444,server,nowait -display none -gdb tcp::3333 -S -kernel'
    qemu = f'{qemu} {kernel_bin} {args}'
    print(qemu)
    os.system(qemu)

    pass

if __name__ == '__main__':
    main()
