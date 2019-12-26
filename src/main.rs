extern crate nom_tutorial;

use structopt::StructOpt;
use std::path::PathBuf;
use nom_tutorial::Mount;

#[derive(StructOpt)]
#[structopt(name = "mount parser")]
struct Opts {
    #[structopt(name = "MOUNT_FILE")]
    file: PathBuf,
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
    if let Ok(mounts) = nom_tutorial::mounts(&opts.file) {
        for mount in mounts {
            println!("The device \"{}\" is mounted at \"{}\".", mount.device, mount.mount_point);
        }
    }
    Ok(())
}
