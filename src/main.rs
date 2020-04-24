
#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::{SimpleLogger, LevelFilter, Config};

extern crate structopt;
use structopt::StructOpt;

use cpuset::{CpuSet, Error};
use cpuset::options::*;

#[derive(Clone, PartialEq, StructOpt)]
struct Options {

    /// Path for cpuset file system
    #[structopt(short, long, default_value="/sys/fs/cgroup/cpuset")]
    path: String,

    /// Cpuset subcommand
    #[structopt(subcommand)]
    cmd: Commands,

    /// Log level setting
    #[structopt(long, default_value="debug")]
    log_level: LevelFilter,
}

#[derive(Clone, PartialEq, StructOpt)]
enum Commands {
    /// Create a new cpuset file system
    Init,

    /// List existing cpusets
    List(ListOptions),

    /// Create a new cpuset
    Create(CreateOptions),

    /// Remove a new cpuset
    Remove(RemoveOptions),
}

fn do_command(cpu_set: &mut CpuSet, cmd: &Commands) -> Result<(), Error> {
    match cmd {
        Commands::Init => {
            cpu_set.init()?;
        },
        Commands::List(o) => {
            let r = cpu_set.list(o)?;
            println!("{:?}", r);
        },
        Commands::Create(o) => {
            let r = cpu_set.create(o)?;
            println!("{:?}", r);
        },
        Commands::Remove(o) => {
            cpu_set.remove(o)?;
        }
    };

    Ok(())
}

pub fn main() {

    // Load options
    let opts = Options::from_args();

    // Setup logging
    let _ = SimpleLogger::init(opts.log_level, Config::default());

    // Create CPUSet
    let mut cpu_set = CpuSet::new(&opts.path);

    if let Err(e) = do_command(&mut cpu_set, &opts.cmd) {
        error!("{:?}", e);
    }
}
