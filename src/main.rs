use nix::pty;
use std::fs::{read_link, File};
use std::io::{Read, Write};
use std::os::unix::fs::symlink;
use std::os::unix::io::FromRawFd;
use std::path::{Path, PathBuf};

fn main() {
    let mut pty = create_pty().unwrap();
    let symlink_path = Path::new("/tmp/ttyuartmock");
    if symlink_path.exists() {
        std::fs::remove_file(symlink_path).unwrap();
    }
    symlink(&pty.slave_path, symlink_path).unwrap();

    println!("Master: {:?}, Slave: {:?}", pty.master_path, pty.slave_path);

    let mut buffer = [0 as u8; 1000];
    loop {
        if let Ok(bytes_read) = pty.master_file.read(&mut buffer) {
            println!("Received Data, {} bytes", bytes_read);
            pty.master_file.write(&buffer[0..bytes_read]).unwrap();
        }
    }
}

struct PseudoTerminal {
    pub master_file: File,
    pub master_path: PathBuf,
    pub slave_file: File,
    pub slave_path: PathBuf,
}

fn create_pty() -> std::io::Result<PseudoTerminal> {
    let pty_pair = pty::openpty(None, None).unwrap();
    std::io::Result::Ok(PseudoTerminal {
        master_file: unsafe { File::from_raw_fd(pty_pair.master) },
        master_path: read_link(format!("/proc/self/fd/{}", pty_pair.master))?,
        slave_file: unsafe { File::from_raw_fd(pty_pair.slave) },
        slave_path: read_link(format!("/proc/self/fd/{}", pty_pair.slave))?,
    })
}
