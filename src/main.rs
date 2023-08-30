use {anyhow::Result, seraphite::parser::Args};

fn main() -> Result<()> {
    Args::build().handle()?;
    Ok(())
}
