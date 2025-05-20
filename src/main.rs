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

fn fetch_remotes(repo: &Repository, ssh_key: &Path) -> Result<()> {
    let mut cbs = git2::RemoteCallbacks::new();

    cbs.credentials(|_url, username_from_url, _allowed_types| {
        git2::Cred::ssh_key(username_from_url.unwrap_or("git"), None, ssh_key, None)
    });
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(cbs);

    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[] as &[&str], Some(&mut fetch_options), None)?;
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

fn for_durty_test() {
    println!("Should create a conflict");
}
