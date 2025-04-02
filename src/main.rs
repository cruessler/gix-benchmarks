use crate::git_backend::GitLogWalker;
use clap::{Parser, ValueEnum};
use git2::Repository;

mod git_backend;

#[derive(Clone, Debug, ValueEnum)]
enum Library {
    /// Use `libgit2` for running the benchmark.
    Git,
    /// Use `gitoxide` for running the benchmark.
    Gix,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommands {
    /// Benchmark traversing a repository’s full history.
    Log {
        #[arg(value_enum)]
        library: Library,
    },
}

#[derive(Debug, clap::Parser)]
#[clap(name = "gix-benchmarks")]
struct Args {
    #[clap(subcommand)]
    pub cmd: Subcommands,
}

fn main() {
    let args: Args = Args::parse_from(std::env::args_os());

    match args.cmd {
        Subcommands::Log { library } => match library {
            Library::Git => {
                let repo = Repository::open_from_env().unwrap();

                println!(
                    "walking the history of {workdir:?}, counting the number of cs in commit ids",
                    workdir = repo.workdir().unwrap()
                );

                let mut walker = GitLogWalker::new(&repo);
                let mut commits = Vec::new();
                let mut number_of_commits = 0;
                let mut number_of_cs = 0;

                while walker.read(&mut commits) > 0 {
                    number_of_commits += commits.iter().count();
                    number_of_cs += commits.iter().fold(0, |acc, commit| {
                        acc + commit
                            .to_string()
                            .chars()
                            .filter(|char| *char == 'c')
                            .count()
                    });

                    commits.clear();
                }

                println!("number of commits traversed: {number_of_commits}");
                println!("number of cs in all commit ids traversed: {number_of_cs}");
            }
            Library::Gix => todo!(),
        },
    }
}
