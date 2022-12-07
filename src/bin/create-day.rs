use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! create_and_write {
    ($path:literal, $text:literal) => {
        create_and_write_impl(format!($path), format!($text))
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read command line argument for the day and parse it into a number
    let day = std::env::args()
        .nth(1)
        .ok_or("Missing arguments")?
        .parse::<u8>()?;
    // Create the file if it doesn't exist
    create_and_write!(
        "src/bin/day{day}.rs",
        "use std::fs;

fn main() {{
    let input = fs::read_to_string(\"inputs/day{day}.txt\").unwrap();
}}
"
    )?;
    create_and_write!("inputs/day{day}.txt", "")?;
    Ok(())
}

fn create_and_write_impl<P: AsRef<Path>>(path: P, content: String) -> Result<(), std::io::Error> {
    if path.as_ref().exists() {
        eprintln!("File already exists: {}", path.as_ref().display());
        return Ok(());
    }

    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    eprintln!("Created file: {}", path.as_ref().display());
    Ok(())
}
