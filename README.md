# Littinit: little init programs

A simple init process designed to launch a shell or other processes on the console tty after the Linux kernel boots. Compared to using `sh` as the init process, Littinit automatically mounts directories like `proc`, `sys`, and `dev` during startup and configures the tty device, avoiding the `no job control in this shell` warning.

## Build

```sh
cargo b --release
```

The default target is `x86_64-unknown-linux-musl`. You can also specify other targets using the `--target` option, but it is recommended to use a musl-based libc to obtain a statically linked Littinit directly.

```sh
cargo b --release --target aarch64-unknown-linux-musl
```


Then you will have a dependency-free executable:
```sh
ldd target/x86_64-unknown-linux-musl/release/littinit
```
-> statically linked


## Testing in QEMU

To test using QEMU with an Ubuntu Base rootfs, prepare the following files in advance:

- Linux kernel: bzImage
- Ubuntu Base: [ubuntu-base-22.04-base-amd64.tar.gz](https://cdimage.ubuntu.com/ubuntu-base/releases/22.04/release/)
- QEMU


### Installing Dependencies

```sh
# for ubuntu-base
sudo apt install -y qemu-system-x86 cpio wget tar gzip

# for linux kernel
sudo apt install -y make clang llvm lld flex bison bc automake libelf-dev libssl-dev
```

### Downloading Ubuntu Base

```sh
mkdir ubuntu-rootfs
```
```sh
wget -qO - https://cdimage.ubuntu.com/ubuntu-base/releases/22.04/release/ubuntu-base-22.04-base-amd64.tar.gz | tar -C ubuntu-rootfs -xvzf -
```

After completion, the directory structure will look like this:

```sh
tree -L 1 ubuntu-rootfs/
```
```log
ubuntu-rootfs/
├── bin -> usr/bin
├── boot
├── dev
├── etc
├── home
├── lib -> usr/lib
├── lib32 -> usr/lib32
├── lib64 -> usr/lib64
├── libx32 -> usr/libx32
├── media
├── mnt
├── opt
├── proc
├── root
├── run
├── sbin -> usr/sbin
├── srv
├── sys
├── tmp
├── usr
└── var

21 directories, 0 files
```

### Copying Littinit as the Init Process

```sh
cp target/x86_64-unknown-linux-musl/release/littinit ubuntu-rootfs/init
```

### Packing the Initrd

```sh
find ubuntu-rootfs -printf "%P\n" | cpio -D ubuntu-rootfs -R root:root -H newc -o -F ubuntu-rootfs.cpio
```

### Compiling the Linux Kernel

1. Download the source code

```sh
git clone https://github.com/torvalds/linux.git -b v6.14 --depth 1
```

2. Compile the kernel

```sh
export LLVM=1
make -C linux O=../linux-build defconfig
make -C linux O=../linux-build bzImage -j$(nproc)
```

he compiled kernel file will be located at `linux-buildarch/x86/boot/bzImage`


### Running the Kernel and Littinit with QEMU


```sh
qemu-system-x86_64 --nographic \
-smp $(nproc) -m 2G \
-kernel linux-build/arch/x86/boot/bzImage \
-initrd ubuntu-rootfs.cpio \
-append "console=ttyS0"
```

The output will show that Littinit has successfully launched Bash

```log
......
Trying run /bin/bash...
......
root@(none):/# ps
  PID TTY          TIME CMD
    1 ttyS0    00:00:01 bash
  131 ttyS0    00:00:00 ps
root@(none):/#
```