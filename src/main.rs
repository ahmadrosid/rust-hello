mod file;

use std::{env::args, path::Path};

use anyhow::Ok;
use file::read_string;

fn main() -> anyhow::Result<()> {
    if let Some(value) = take_arg() {
        let path = Path::new(&value);
        let content = read_string(path)?;//.expect("Can not find the file path.");
        println!("{}", content);
    }
    Ok(())
}

fn take_arg() -> Option<String> {
    let arg = args().nth(1);
    let value = arg?;
    Some(value)
}
