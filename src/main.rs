use anyhow::Result;
use git2::Repository;

fn main() -> Result<()> {
    let repo = Repository::open(".")?;

    println!("{:?}", repo.path());
    Ok(())
}
