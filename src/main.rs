use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[allow(unused_must_use)]
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn main() {
    let path = Path::new("foo.txt");
    echo("Hello World", path);

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open because {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
