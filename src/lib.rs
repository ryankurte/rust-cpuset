//! cpuset library
//! Uses the cpuset sysfs to apply cpuset rules, see
//! https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v1/cpusets.html for information


#[macro_use]
extern crate log;

#[macro_use]
extern crate failure;

extern crate structopt;

extern crate regex;

use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

pub mod options;
use options::*;

pub mod error;
pub use error::Error;

pub const CPUSET_PATH: &str = "/sys/fs/cgroup/cpuset";

pub struct CpuSet {
    path: String,
}

impl CpuSet {
    // Open existing cpuset directory    
    pub fn new(path: &str) -> Self {
        Self{ path: path.to_string() }
    }

    /// Initialise and mount a new cpuset directory
    pub fn init(&mut self) -> Result<(), io::Error> {
      // Create directory if required
      if !Path::new(&self.path).exists() {
        fs::create_dir(&self.path)?;
      }

      // Execute mount command
      Command::new("mount")
        .args(&["-t", "cgroup", "-ocpuset", "cpuset", &self.path])
        .output()
        .expect("Mount error");

      Ok(())
    }

    /// List existing cpusets
    pub fn list(&self, opts: &ListOptions) -> Result<Vec<Set>, Error> {
      let mut sets = vec![];

      // Find sets (directories under the base path)
      for d in fs::read_dir(&self.path)? {
        let d = d?;

        if d.path().is_dir() {
          if let Some(name) = d.file_name().to_str() {
            let set = Set::load(&d.path())?;
            sets.push(set);
          }
        }
      }

      println!("Found sets: {:?}", sets);
      
      Ok(sets)
    }

    /// Create a new cpuset
    pub fn create(&self, opts: &CreateOptions) -> Result<(), Error> {
      let p = Path::new(&self.path).join(&opts.name);

      println!("Creating set: {:?}", p);

      fs::create_dir(&p)?;

      if !p.exists() {
        error!("cpuset {:?} creation failed", opts.name);
        return Err(Error::CreationFailed)
      }

      Ok(())
    }

    pub fn remove(&self, opts: &RemoveOptions) -> Result<(), Error> {
      let p = Path::new(&self.path).join(&opts.name);

      fs::remove_dir(&p)?;

      Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Set {
  pub name: String,

  pub cpus: CpuRange,

  pub cpu_exclusive: bool,
}

pub const CPUSET_CPUS: &str = "cpuset.cpus";
pub const CPUSET_CPU_EXCLUSIVE: &str = "cpuset.cpu_exclusive";

impl Set {
  pub fn load(path: &Path) -> Result<Self, Error> {
    let name = path.file_name().unwrap().to_str().unwrap();

    let cpus_str = fs::read_to_string(path.join(CPUSET_CPUS))?;

    let cpus = CpuRange::from_str(&cpus_str).unwrap();

    let cpu_exclusive_str = fs::read_to_string(path.join(CPUSET_CPU_EXCLUSIVE))?;

    let cpu_exclusive: usize = trim_line_endings(&cpu_exclusive_str).parse()?;

    let cpu_exclusive = match cpu_exclusive {
      0 => false,
      1 => true,
      _ => unimplemented!(),
    }

    Ok(Self{ name: name.to_string(), cpus, cpu_exclusive })
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CpuRange {
  None,
  List(Vec<usize>),
  Range{
    start: usize,
    end: usize,
  },
}

fn trim_line_endings(s: &str) -> &str {
  s.trim_end_matches(|c| {
    c == '\r' || c == '\n'
  })
}


impl FromStr for CpuRange {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = trim_line_endings(s);

    if s.len() == 0 {
      return Ok(Self::None)
    }

    debug!("Parsing cpurange: '{}'", s);
    
    let r = if s.contains(",") {
      let v = s.split(",").map(|i| i.parse() ).collect::<Result<Vec<usize>, _>>()?;

      Self::List(v)
    
    } else if s.contains("-") {
      let v = s.split("-").map(|i| {
        debug!("Split: '{}'", i);
        i.parse()
      } ).collect::<Result<Vec<usize>, _>>()?;

      if v.len() != 2 {
        return Err(Error::InvalidFormat(s.to_string(), format!("0-1 or 0,1,2")));
      }

      Self::Range{start: v[0], end: v[1]}

    } else {
      let n: usize = s.parse()?;
      Self::List(vec![n])
    };

    Ok(r)
  }
}

impl ToString for CpuRange {
  fn to_string(&self) -> String {
    match self {
      CpuRange::List(l) => {
        let v: Vec<String> = l.iter().map(|i| format!("{}", i)).collect();

        v.join(",")
      },
      CpuRange::Range{start, end} => format!("{}-{}", start, end),
      CpuRange::None => format!("")
    }
  }
}

