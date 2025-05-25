use anyhow::{Context, Ok, Result, ensure};
use nix::{ioctl_none, ioctl_read, ioctl_write_buf, ioctl_write_ptr};
use std::{
    fs::File,
    os::{fd::AsRawFd, unix::fs::FileTypeExt},
    path::{Path, PathBuf},
    ptr,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const LOOP_CTRL_DEV: &str = "/dev/loop-control";

ioctl_none!(loop_ctl_get_free, LOOPIO_MAGIC, LOOPCTL_GET_FREE);
ioctl_read!(loop_get_status, LOOPIO_MAGIC, LOOPIO_GET_STATUS, loop_info);
ioctl_read!(loop_get_status64, LOOPIO_MAGIC, LOOPIO_GET_STATUS, loop_info64);
ioctl_write_ptr!(loop_set_status, LOOPIO_MAGIC, LOOPIO_SET_STATUS, loop_info);

pub fn set_loop<P>(file: P) -> Result<PathBuf>
where
    P: AsRef<Path>,
{
    let file = file.as_ref();
    let loop_ctl_dev = File::open(LOOP_CTRL_DEV)?;

    /* 获取空闲的loop device */
    let free = unsafe { loop_ctl_get_free(loop_ctl_dev.as_raw_fd()) }
        .context("Get free loop device fail")?;

    println!("free loop device: {free}");

    /* 检查loop device */
    let loop_dev_path = PathBuf::from(format!("/dev/loop{free}"));
    let loop_dev = File::open(loop_dev_path.as_path())
        .context(format!("Fail to open device {loop_dev_path:?}"))?;
    ensure!(
        loop_dev.metadata()?.file_type().is_block_device(),
        "device {loop_dev_path:?} is not a block device"
    );

    /* 关联file与loop device*/
    let mut loopdev_info = unsafe { std::mem::zeroed() };
    unsafe { loop_get_status64(loop_dev.as_raw_fd(), &mut loopdev_info) }
        .context(format!("Fail to get loop device({loop_dev:?}) info"))?;

    println!("loopdev_info: {loopdev_info:?}");

    let file = File::open(file).context(format!("Fail to open file {file:?}"))?;

    Ok(Path::new("/dev/loop0").to_path_buf())
}
