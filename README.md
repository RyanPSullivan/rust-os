# rust-os
An Operating System written in rust. Inspired by http://os.phil-opp.com/

## pre-requisites
- nasm
- ld
- binutils
- grub
- qemu

## build steps
nasm -f elf64 multiboot_header.asm
nasm -f elf64 boot.asm
ld -n -o bin/isofiles/kernel.bin -T src/linker.ld src/multiboot_header.o src/boot.o
grub-mkrescue -o os.iso bin/isofiles

