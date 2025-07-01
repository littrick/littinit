# Littinit: little init programs

一个简单的init进程, 负责在linux内核启动后, 在console的tty上拉起shell或者其他进程, 相对于用sh作为init进程, littinit会在启动时自动挂载`proc`,`sys`,`dev`等目录, 并配置好tty设备, 避免出现`no job control in this shell`的警告

## 构建

```sh
cargo b --release
```

默认target为`x86_64-unknown-linux-musl`, 你也可以添加`--target`选项指定其他target, 但建议选择musl的c库, 能直接得到静态链接littinit

```sh
cargo b --release --target aarch64-unknown-linux-musl
```


然后你就得到了一个没有依赖的程序:
```sh
ldd target/x86_64-unknown-linux-musl/release/littinit
```
-> statically linked


## 在qemu测试

尝试使用qemu运行ubuntu-base的rootfs, 提前准备好以下文件

- linux kernel: bzImage
- ubuntu-base: [ubuntu-base-22.04-base-amd64.tar.gz](https://cdimage.ubuntu.com/ubuntu-base/releases/22.04/release/)
- qemu


### 安装依赖

```sh
# for ubuntu-base
sudo apt install -y qemu-system-x86 cpio wget tar gzip

# for linux kernel
sudo apt install -y make clang llvm lld flex bison bc automake libelf-dev libssl-dev
```

### 下载ubuntu-base

```sh
mkdir ubuntu-rootfs
```
```sh
wget -qO - https://cdimage.ubuntu.com/ubuntu-base/releases/22.04/release/ubuntu-base-22.04-base-amd64.tar.gz | tar -C ubuntu-rootfs -xvzf -
```

完成后目录结构如下

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

### 复制littinit为init

```sh
cp target/x86_64-unknown-linux-musl/release/littinit ubuntu-rootfs/init
```

### 打包initrd

```sh
find ubuntu-rootfs -printf "%P\n" | cpio -D ubuntu-rootfs -R root:root -H newc -o -F ubuntu-rootfs.cpio
```

### 编译linux kernel

1. 下载源码

```sh
git clone https://github.com/torvalds/linux.git -b v6.14 --depth 1
```

2. 编译kernel

```sh
export LLVM=1
make -C linux O=../linux-build defconfig
make -C linux O=../linux-build bzImage -j$(nproc)
```

输出kernel文件在`linux-buildarch/x86/boot/bzImage`


### 使用qemu运行kernel和littinit


```sh
qemu-system-x86_64 --nographic \
-smp $(nproc) -m 2G \
-kernel linux-build/arch/x86/boot/bzImage \
-initrd ubuntu-rootfs.cpio \
-append "console=ttyS0"
```

输出如下, littinit已经正常拉起bash

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