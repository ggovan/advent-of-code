use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub type Res<T> = Result<T, Box<dyn Error>>;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_better<P, R, F>(
    filename: P,
    item_parser: &'static F,
) -> io::Result<impl Iterator<Item = Vec<R>>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> R,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(move |l| l.unwrap().split(',').map(item_parser).collect::<Vec<R>>()))
}
