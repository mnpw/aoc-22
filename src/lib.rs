use std::io::Error;

pub fn read_input(path: &str) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(|e| {
        eprintln!("Could not load file: {e}");
        e
    })
}
