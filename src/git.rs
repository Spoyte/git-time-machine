use anyhow::Result;
use git2::Repository;
use std::path::Path;

pub struct GitExplorer {
    repo: Repository,
}

impl GitExplorer {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path)?;
        Ok(Self { repo })
    }

    /// Get all commits in chronological order
    pub fn get_commit_history(&self) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        
        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            commits.push(CommitInfo::from_commit(&commit)?);
        }
        
        // Reverse to get chronological order (oldest first)
        commits.reverse();
        Ok(commits)
    }
}

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub id: String,
    pub short_id: String,
    pub message: String,
    pub author: String,
    pub time: i64,
    pub parent_count: usize,
}

impl CommitInfo {
    fn from_commit(commit: &git2::Commit) -> Result<Self> {
        Ok(Self {
            id: commit.id().to_string(),
            short_id: commit.id().to_string()[..8].to_string(),
            message: commit.message().unwrap_or("").lines().next().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("Unknown").to_string(),
            time: commit.time().seconds(),
            parent_count: commit.parent_count(),
        })
    }
}
