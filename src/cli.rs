use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Args {
    /// Act as a client, and mount FUSE at given path
    pub mount_point: String,
    /// Automatically unmount on process exit
    #[clap(short = 'u', long, takes_value = false)]
    pub auto_unmount: bool,
    /// Allow root user to access filesystem
    #[clap(short = 'r', long, takes_value = false)]
    pub allow_root: bool,
}
