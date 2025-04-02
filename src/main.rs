use clap::{Parser, ValueEnum};
use git_backend::GitLogWalker;
use gix_backend::GixLogWalker;

mod git_backend;
mod gix_backend;

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
                let repo = git2::Repository::open_from_env().unwrap();

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
            Library::Gix => {
                let mut repo: gix::Repository =
                    gix::ThreadSafeRepository::discover_with_environment_overrides(".")
                        .map(Into::into)
                        .unwrap();

                println!(
                    "walking the history of {workdir:?}, counting the number of cs in commit ids",
                    workdir = repo.work_dir().unwrap()
                );

                let mut walker = GixLogWalker::new(&mut repo);
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
        },
    }
}
