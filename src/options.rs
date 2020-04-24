use structopt::StructOpt;

#[derive(Clone, PartialEq, StructOpt)]
pub struct OpenOptions {
   
}

#[derive(Clone, PartialEq, StructOpt)]
pub struct ListOptions {
   
}

#[derive(Clone, PartialEq, StructOpt)]
pub struct CreateOptions {
    /// name for new cpuset
    #[structopt(long)]
    pub name: String
}

#[derive(Clone, PartialEq, StructOpt)]
pub struct RemoveOptions {
    /// Name of cpuset to be removed
    #[structopt(long)]
    pub name: String
}
