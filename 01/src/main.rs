use std::path::Path;
use std::path::PathBuf;

fn get_contents(input_filename: &Path) -> Result<Vec<i32>, &str> {
    let contents: String = std::fs::read_to_string(input_filename).unwrap_or_else(|_| {
        panic!(
            "Problems reading from file: {}",
            input_filename.to_str().unwrap()
        )
    });
    let numbers: Vec<i32> = contents
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    Ok(numbers)
}

fn find_sum(target_sum: i32, operand_count: i32, numbers: &[i32]) -> Option<Vec<i32>> {
    let mut operands: Vec<i32> = Vec::new();
    let number_count = numbers.len();

    for (index, &val) in numbers.iter().enumerate() {
        if operand_count == 1 && val == target_sum {
            operands.push(val);
            break;
        }

        if operand_count > 1 && index < (number_count - 1) {
            if let Some(mut result) =
                find_sum(target_sum - val, operand_count - 1, &numbers[index..])
            {
                operands.push(val);
                operands.append(&mut result);
                break;
            }
        }
    }

    if operands.is_empty() {
        None
    } else {
        Some(operands)
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
    let numbers = get_contents(&input_filename).unwrap();

    if let Some(operands) = find_sum(2020, 2, numbers.as_slice()) {
        let product: i32 = operands.iter().product();

        println!(
            "Part 1:\nFound operands adding up to 2020: {:#?} with product {}",
            operands, product
        );
    } else {
        println!("Part 1:\nNo operands found.");
    }

    if let Some(operands) = find_sum(2020, 3, numbers.as_slice()) {
        let product: i32 = operands.iter().product();

        println!(
            "Part 2:\nFound operands adding up to 2020: {:#?} with product {}",
            operands, product
        );
    } else {
        println!("Part 2:\nNo operands found.");
    }
}
