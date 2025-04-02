use gix::{ObjectId, Repository, revision::Walk};

use crate::LogWalker;

/// This is a simplified version of:
/// https://github.com/gitui-org/gitui/blob/156381155e12d89b39e657f07c8dac557271d24f/asyncgit/src/sync/logwalker.rs

pub struct GixLogWalker<'a> {
    walk: Walk<'a>,
}

impl<'a> GixLogWalker<'a> {
    pub fn new(repo: &'a mut Repository) -> Self {
        // This seems to be an object cache size that yields optimal performance. There’s no
        // specific reason this is 2^14, so benchmarking might reveal that there’s better values.
        repo.object_cache_size_if_unset(2_usize.pow(14));

        let commit = repo.head().unwrap().peel_to_commit_in_place().unwrap();

        let tips = [commit.id];

        let platform = repo
            .rev_walk(tips)
            .sorting(gix::revision::walk::Sorting::ByCommitTime(
                gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
            ))
            .use_commit_graph(false);

        let walk = platform.all().unwrap();

        Self { walk }
    }
}

impl LogWalker<ObjectId> for GixLogWalker<'_> {
    fn read(&mut self, out: &mut Vec<ObjectId>) -> usize {
        let mut count = 0_usize;

        while let Some(Ok(info)) = self.walk.next() {
            out.push(info.id);

            count += 1;
        }

        count
    }
}
