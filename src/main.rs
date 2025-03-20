use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use git2::Repository;

fn main() -> Result<()> {
    let repo = Repository::open(".").context("Current working directory is not Git Directory")?;

    println!("{:?}", repo.path());
    let head = repo.head()?.peel_to_commit()?.id();
    println!("{head}");
    Ok(())
}

fn fetch_remotes(repo: &Repository, ssh_key: Option<&Path>) -> Result<()> {
    Ok(())
}

fn fetch_current_branch(repo: &Repository) -> Result<String> {
    let branches = repo
        .branches(Some(git2::BranchType::Local))?
        .filter_map(Result::ok)
        .filter(|(_branch, branch_type)| branch_type == &git2::BranchType::Local)
        .map(|(branch, _bt)| branch)
        .collect::<Vec<git2::Branch>>();

    Ok(String::default())
}
