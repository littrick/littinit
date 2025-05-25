```sh
dd if=/dev/zero bs=1M count=1 of=target/mount.ext4 && \
mkfs.ext4 target/mount.ext4
```
```sh
cargo b --example mount
```
```sh
mkdir -p target/mnt && \
sudo target/debug/examples/mount target/mount.ext4 target/mnt && \
umount target/mnt
```