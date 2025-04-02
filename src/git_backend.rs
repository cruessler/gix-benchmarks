use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use git2::{Commit, Oid, Repository};

use crate::LogWalker;

/// This is a simplified version of:
/// https://github.com/gitui-org/gitui/blob/156381155e12d89b39e657f07c8dac557271d24f/asyncgit/src/sync/logwalker.rs

struct TimeOrderedCommit<'a>(Commit<'a>);

impl Eq for TimeOrderedCommit<'_> {}

impl PartialEq for TimeOrderedCommit<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.time().eq(&other.0.time())
    }
}

impl PartialOrd for TimeOrderedCommit<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeOrderedCommit<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.time().cmp(&other.0.time())
    }
}

pub struct GitLogWalker<'a> {
    commits: BinaryHeap<TimeOrderedCommit<'a>>,
    visited: HashSet<Oid>,
}

impl<'a> GitLogWalker<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        let commit = repo.head().unwrap().peel_to_commit().unwrap();

        let mut commits = BinaryHeap::with_capacity(10);
        commits.push(TimeOrderedCommit(commit));

        Self {
            commits,
            visited: HashSet::with_capacity(1000),
        }
    }

    fn visit(&mut self, commit: Commit<'a>) {
        if self.visited.insert(commit.id()) {
            self.commits.push(TimeOrderedCommit(commit));
        }
    }
}

impl LogWalker<Oid> for GitLogWalker<'_> {
    fn read(&mut self, out: &mut Vec<Oid>) -> usize {
        let mut count = 0_usize;

        while let Some(commit) = self.commits.pop() {
            for parent in commit.0.parents() {
                self.visit(parent);
            }

            out.push(commit.0.id());

            count += 1;
        }

        count
    }
}
