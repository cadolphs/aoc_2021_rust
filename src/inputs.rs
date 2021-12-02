use std::error::Error;
use std::fs;
use std::str::FromStr;

pub fn read_file_to_list<T: FromStr>(filename: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    <T as FromStr>::Err: Error + 'static,
{
    let file_as_string = fs::read_to_string(filename)?;
    file_as_string
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect()
}
