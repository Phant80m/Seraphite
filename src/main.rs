use anyhow::Result;
use seraphite::parser::Args;
fn main() -> Result<()> {
    let args = Args::build();
    args.handle(false)?;
    Ok(())
}
