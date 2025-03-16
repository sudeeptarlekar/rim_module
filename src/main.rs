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
