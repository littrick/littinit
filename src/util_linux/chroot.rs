use anyhow::{Context, Result, bail, ensure};
use fork::{Fork, fork};
use nix::{sys::wait::waitpid, unistd::Pid};
use std::{
    collections::HashMap,
    env::set_current_dir,
    fmt::Debug,
    os::unix::{fs, process::CommandExt},
    path::Path,
    process::Command,
};

pub fn chroot<P1, P2>(
    new_root: P1,
    cmd: P2,
    args: Vec<String>,
    envs: HashMap<String, String>,
) -> Result<()>
where
    P1: AsRef<Path> + Debug,
    P2: AsRef<Path> + Debug,
{
    match fork() {
        Ok(Fork::Child) => {
            chroot_(new_root, cmd, args, envs)?;
        }
        Ok(Fork::Parent(child)) => {
            waitpid(Pid::from_raw(child), None)?;
        }
        Err(_) => bail!("Fork failed"),
    }
    Ok(())
}

fn chroot_<P1, P2>(
    new_root: P1,
    cmd: P2,
    args: Vec<String>,
    envs: HashMap<String, String>,
) -> Result<()>
where
    P1: AsRef<Path> + Debug,
    P2: AsRef<Path> + Debug,
{
    let new_root = new_root.as_ref();
    let cmd = cmd.as_ref();

    ensure!(new_root.exists(), "{new_root:?} is not exist");

    fs::chroot(new_root).context(format!("failed to chroot to {new_root:?}"))?;
    set_current_dir("/").context("fail to change directry to /")?;

    return Err(Command::new(cmd).args(args).envs(envs).exec().into());
}
