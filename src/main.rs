mod cli;
use std::path::PathBuf;

use anyhow::Ok;
use structopt::StructOpt;
mod find;
mod tasks;

use cli::CommandLineArgs;
// use tasks::Task;
use find::Config;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        dir,
        target_file,
    } = CommandLineArgs::from_args();

    let dir = dir.or_else(find_default_dir).expect("请输入目标目录");

    let target_file = target_file
        .or_else(find_default_target_file)
        .expect("请输入目标文件");

    let c = Config::new(target_file, dir.clone());
    find::read_dir(&dir, &c)?;
    /* match res {
        Ok(()) => {
            println!("it is ok");
        },
        Err(e) => {
            println!("{}", e);
        }
    } */
    /* match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?; */
    Ok(())
}

fn find_default_target_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn find_default_dir() -> Option<PathBuf> {
    home::home_dir()
}
