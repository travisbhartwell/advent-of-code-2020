use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
enum HeightUnits {
    Centimeters,
    Inches,
}

impl FromStr for HeightUnits {
    type Err = &'static str;

    fn from_str(unit_str: &str) -> Result<Self, Self::Err> {
        match unit_str {
            "cm" => Ok(HeightUnits::Centimeters),
            "in" => Ok(HeightUnits::Inches),
            _ => Err("Unknown unit."),
        }
    }
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = &'static str;

    fn from_str(color_str: &str) -> Result<Self, Self::Err> {
        match color_str {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Grey),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            _ => Err("Unknown eye color"),
        }
    }
}

#[derive(Debug)]
enum PassportField {
    BirthYear(i32),
    IssueYear(i32),
    ExpirationYear(i32),
    Height { value: i32, units: HeightUnits },
    HairColor(String),
    EyeColor(EyeColor),
    PassportId(String),
    CountryId(String),
}

impl From<i32> for PassportField {
    fn from(v: i32) -> Self {
        PassportField::BirthYear(v)
    }
}

impl FromStr for PassportField {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split(":").collect();

        if parts.len() != 2 {
            return Err("Invalid input!");
        }

        match parts.as_slice() {
            ["byr", year] => {
                let year_num: i32 = year.parse().unwrap();
                Ok(PassportField::BirthYear(year_num))
            }
            ["iyr", year] => {
                let year_num: i32 = year.parse().unwrap();
                Ok(PassportField::IssueYear(year_num))
            }
            ["eyr", year] => {
                let year_num: i32 = year.parse().unwrap();
                Ok(PassportField::ExpirationYear(year_num))
            }
            ["hgt", height_str] => {
                let len = height_str.len();
                let value: i32 = height_str[0..(len - 2)].parse().unwrap();
                let units: HeightUnits = height_str[(len - 2)..].parse().unwrap();
                Ok(PassportField::Height { value, units })
            }
            ["hcl", hair_color_str] => Ok(PassportField::HairColor(String::from(*hair_color_str))),
            ["ecl", eye_color_str] => {
                let eye_color: EyeColor = eye_color_str.parse().unwrap();
                Ok(PassportField::EyeColor(eye_color))
            }
            ["pid", passport_id] => Ok(PassportField::PassportId(String::from(*passport_id))),
            ["cid", country_id] => Ok(PassportField::CountryId(String::from(*country_id))),
            _ => Err("Invalid field!"),
        }
    }
}

fn main() {
    let input_filename: PathBuf;

    if std::env::args().len() > 1 {
        input_filename = PathBuf::from(std::env::args().nth(1).unwrap());
    } else {
        input_filename = PathBuf::from(r"input.txt");
    }

    println!("Using filename {}.", input_filename.to_str().unwrap());

    let test_str = String::from(
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
    );

    let test: Vec<PassportField> = test_str
        .split_whitespace()
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect();
    println!("Result: {:#?}", test);
}
