use anyhow::{Context, Result, bail, ensure};
use nix::mount::{MsFlags, mount};
use std::{
    env::set_current_dir,
    fmt::Debug,
    os::unix::{fs::chroot, process::CommandExt},
    path::Path,
    process,
    process::Command,
};

pub fn switch_root<P1, P2>(new_root: P1, init: P2, args: Vec<String>) -> Result<()>
where
    P1: AsRef<Path> + Debug,
    P2: AsRef<Path> + Debug,
{
    let new_root = new_root.as_ref();
    let old_root = Path::new("/");
    let init = init.as_ref();

    ensure!(
        process::id() == 1,
        "switch_root must be executed by process 1 (the init process)"
    );
    ensure!(new_root.exists(), "{new_root:?} is not exist");
    ensure!(old_root.exists(), "{old_root:?} is not exist");

    set_current_dir(new_root).context(format!("failed to change directory to {new_root:?}"))?;

    mount(
        Some(new_root.as_os_str()),
        "/",
        None::<&str>,
        MsFlags::MS_MOVE,
        None::<&str>,
    )
    .context(format!("failed to mount moving {new_root:?} to /"))?;

    chroot(".").context("failed to chroot to .")?;

    set_current_dir("/").context(format!("failed to change directory to {new_root:?}"))?;

    let err = Command::new(init).env_clear().args(args).exec();

    bail!("init result: {err:?}");
}
