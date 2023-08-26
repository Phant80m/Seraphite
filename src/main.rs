use seraphite::parser::Args;
use std::io;
fn main() -> io::Result<()> {
    let args = Args::build();
    args.handle(false)?;
    Ok(())
}
