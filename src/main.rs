use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use git2::Repository;

fn main() -> Result<()> {
    // Some conflicting change
    // LOL another conflicting change
    // Temp change
    let repo = Repository::open(".").context("Current working directory is not Git Directory")?;

    println!("{:?}", repo.path());
    let head = repo.head()?.peel_to_commit()?.id();
    println!("{head}");
    Ok(())
}

/// Just to fetch the remotes
/// Ftches all remotes to the local but do not update the local branch
fn fetch_remotes(repo: &Repository, ssh_key: Option<&Path>) -> Result<()> {
    let mut fetch_options = git2::FetchOptions::new();
    let mut cbs = git2::RemoteCallbacks::new();
    cbs.credentials(move |_url, username, _at| {
        git2::Cred::ssh_key(username.unwrap(), None, &Path::new(""), None)
    });
    fetch_options.remote_callbacks(cbs);
    Ok(())
}
