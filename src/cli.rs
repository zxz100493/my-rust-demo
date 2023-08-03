// mod cli;
// use std::path::PathBuf;

// pub enum Action {
//     Add { task: String },
//     Done { position: usize },
//     List,
// }

// pub struct CommandLineArgs {
//     pub action: Action,
//     pub journal_file: Option<PathBuf>,
// }
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rust Journal", about = "测试rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// 在哪个目录中找.
    #[structopt(parse(from_os_str), short, long)]
    pub dir: Option<PathBuf>,

    /// 你想找到哪个文件.
    #[structopt(parse(from_os_str), short, long)]
    pub target_file: Option<PathBuf>,
}
