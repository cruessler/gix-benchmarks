use backend::LogWalker;
use clap::{Parser, ValueEnum};
use git_backend::GitLogWalker;
use git2::opts::{strict_hash_verification, strict_object_creation};
use gix_backend::GixLogWalker;

mod backend;
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
    /// Benchmark blaming a single file.
    Blame {
        #[arg(value_enum)]
        library: Library,

        #[arg(long)]
        path: String,
    },
}

#[derive(Debug, clap::Parser)]
#[clap(name = "gix-benchmarks")]
struct Args {
    #[clap(subcommand)]
    pub cmd: Subcommands,
}

fn walk_log_and_count_letters<Id: ToString>(mut walker: impl LogWalker<Id>) {
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

fn main() {
    let args: Args = Args::parse_from(std::env::args_os());

    match args.cmd {
        Subcommands::Log { library } => match library {
            Library::Git => {
                // This makes `git2` as fast as possible.
                strict_object_creation(false);
                strict_hash_verification(false);

                let repo = git2::Repository::open_from_env().unwrap();

                println!(
                    "using `git2` to walk the history of {workdir:?}, counting the number of cs in commit ids",
                    workdir = repo.workdir().unwrap()
                );

                let walker = GitLogWalker::new(&repo);

                walk_log_and_count_letters(walker);
            }
            Library::Gix => {
                let mut repo: gix::Repository =
                    gix::ThreadSafeRepository::discover_with_environment_overrides(".")
                        .map(Into::into)
                        .unwrap();

                println!(
                    "using `gix` to walk the history of {workdir:?}, counting the number of cs in commit ids",
                    workdir = repo.workdir().unwrap()
                );

                let walker = GixLogWalker::new(&mut repo);

                walk_log_and_count_letters(walker);
            }
        },
        Subcommands::Blame { library, path } => match library {
            Library::Git => todo!(),
            Library::Gix => {
                let repo: gix::Repository =
                    gix::ThreadSafeRepository::discover_with_environment_overrides(".")
                        .map(Into::into)
                        .unwrap();

                println!(
                    "using `gix` to get blame for {path:?} (in {workdir:?})",
                    workdir = repo.workdir().unwrap()
                );

                let head_id: gix::ObjectId =
                    repo.head().unwrap().peel_to_commit_in_place().unwrap().id;

                let cache: Option<gix::commitgraph::Graph> =
                    repo.commit_graph_if_enabled().unwrap();
                let mut resource_cache = repo.diff_resource_cache_for_tree_diff().unwrap();

                let diff_algorithm = repo.diff_algorithm().unwrap();

                let options = gix_blame::Options {
                    diff_algorithm,
                    range: None,
                    since: None,
                };

                let outcome = gix_blame::file(
                    &repo.objects,
                    head_id.into(),
                    cache,
                    &mut resource_cache,
                    (&path as &str).into(),
                    options,
                )
                .unwrap();

                println!("got {len} blame entries", len = outcome.entries.len());
            }
        },
    }
}
