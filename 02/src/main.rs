use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
struct PasswordData {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordData {
    fn new(min: usize, max: usize, letter: char, password: &str) -> PasswordData {
        PasswordData {
            min,
            max,
            letter,
            password: String::from(password),
        }
    }

    fn is_valid_part1(&self) -> bool {
        let pred = |c| c == self.letter;

        let letter_count = self.password.matches(pred).count();

        self.min <= letter_count && letter_count <= self.max
    }

    fn is_valid_part2(&self) -> bool {
        let password_chars = self.password.chars().collect::<Vec<char>>();
        // Values in the file are 1 based.
        let position1 = self.min - 1;
        let position2 = self.max - 1;

        let char_in_pos1 = password_chars[position1] == self.letter;
        let char_in_pos2 = password_chars[position2] == self.letter;

        char_in_pos1 ^ char_in_pos2
    }
}

#[derive(Debug)]
struct Error {
    string: String,
}

impl Display for PasswordData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "(min: {}, max: {}, letter: '{}', password:'{}')",
            self.min, self.max, self.letter, self.password
        )
    }
}

impl FromStr for PasswordData {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // 16-18 h: hhhhhhhhhhhhhhhhhh
        let parts: Vec<_> = line.split(' ').collect();

        if let [positions, letter_part, password] = parts.as_slice() {
            let parts: Vec<usize> = positions
                .split('-')
                .map(|part| part.trim().parse().unwrap())
                .collect();
            let min: usize = parts[0];
            let max: usize = parts[1];
            let letter: char = letter_part.chars().next().unwrap();
            let pd = PasswordData::new(min, max, letter, password);
            return Ok(pd);
        }

        Err(Error {
            string: format!("Problems reading line: {}", line),
        })
    }
}

fn get_contents(input_filename: &Path) -> Result<Vec<PasswordData>, &str> {
    let contents: String = std::fs::read_to_string(input_filename).unwrap_or_else(|_| {
        panic!(
            "Problems reading from file: {}",
            input_filename.to_str().unwrap()
        )
    });
    let password_data: Vec<PasswordData> = contents
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    Ok(password_data)
}

fn main() {
    let input_filename: PathBuf;

    if std::env::args().len() > 1 {
        input_filename = PathBuf::from(std::env::args().nth(1).unwrap());
    } else {
        input_filename = PathBuf::from(r"input.txt");
    }

    println!("Using filename {}.", input_filename.to_str().unwrap());

    let password_data: Vec<PasswordData> = get_contents(&input_filename).unwrap();

    let part1_valid_passports: Vec<&PasswordData> = password_data
        .iter()
        .filter(|pd| pd.is_valid_part1())
        .collect();

    println!(
        "Part 1: Found {} valid passports.",
        part1_valid_passports.len()
    );

    let part2_valid_passports: Vec<&PasswordData> = password_data
        .iter()
        .filter(|pd| pd.is_valid_part2())
        .collect();

    println!(
        "Part 2: Found {} valid passports.",
        part2_valid_passports.len()
    );
}
