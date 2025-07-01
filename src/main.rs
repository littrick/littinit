use anyhow::Result;
use nix::{
    ioctl_write_int_bad,
    mount::{MsFlags, mount},
    unistd::{getsid, setsid},
};
use std::{
    env::args,
    ffi::OsStr,
    fs,
    io::stdin,
    os::{fd::AsRawFd, unix::process::CommandExt},
    path::Path,
    process::Command,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
ioctl_write_int_bad!(tiocsctty, TIOCSCTTY);

fn main() {
    let args: Vec<_> = args().collect();
    // println!("args: {args:?}");

    setup_tmpfs();
    setup_tty();

    if args.len() > 1 {
        try_run_cmd(&args[1], &args[2..]).unwrap();
    } else {
        try_run_shell().unwrap();
    }
}

fn setup_tmpfs() {
    let mount_list: [(&'static str, &'static str); 4] = [
        ("/proc", "proc"),
        ("/sys", "sysfs"),
        ("/dev", "devtmpfs"),
        ("/run", "tmpfs"),
    ];

    for (point, fstype) in mount_list {
        let mount_point = Path::new(point);
        if !mount_point.exists() {
            if let Err(err) = fs::create_dir_all(mount_point) {
                println!("Fail to create dir {point}: {err}")
            }
        }

        if let Err(err) =
            mount::<str, _, _, str>(None, mount_point, Some(fstype), MsFlags::empty(), None)
        {
            println!("Warning: Fail to mount {point} as {fstype}: {err}")
        };
    }
}

fn setup_tty() {
    if let Err(err) = getsid(None).or(setsid()) {
        println!("Warning: Fail to creates a session: {err}");
    };

    if let Err(err) = unsafe {
        tiocsctty(stdin().as_raw_fd(), 0 /* dont steal */)
    } {
        println!("Warning: Fail to set tty: {err}")
    }
}

fn try_run_cmd<S1, I, S2>(cmd: S1, args: I) -> Result<()>
where
    S1: AsRef<OsStr>,
    I: IntoIterator<Item = S2>,
    S2: AsRef<OsStr>,
{
    Err(Command::new(cmd).args(args).exec().into())
}

fn try_run_shell() -> Result<()> {
    let shell_list = ["/bin/bash", "/bin/sh"];
    for shell in shell_list {
        println!("Trying run {shell}...");
        if fs::exists(shell)? {
            return Err(Command::new(shell).exec().into());
        } else {
            println!("Warning: {shell} is not exist");
        }
    }
    Ok(())
}
