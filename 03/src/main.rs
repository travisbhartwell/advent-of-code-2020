use std::{fmt::Display, path::Path};
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Debug)]
struct TreeInLocation(bool);
#[derive(Debug)]
struct Error {
    string: String,
}

impl TreeInLocation {
    fn is_tree_in_location(&self) -> bool {
        self.0
    }
}

impl FromStr for TreeInLocation {
    type Err = Error;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match input.chars().nth(0) {
            Some('#') => Ok(TreeInLocation(true)),
            Some('.') => Ok(TreeInLocation(false)),
            _ => Err(Error {
                string: format!("Expecting '#' or '.', got '{}'.", input),
            }),
        }
    }
}

impl Display for TreeInLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.0 == true {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

#[derive(Debug)]
struct TreePlacementMap {
    map_data: Vec<Vec<TreeInLocation>>,
    width: usize,
    height: usize
}

impl TreePlacementMap {
    fn new(map_data: Vec<Vec<TreeInLocation>>) -> TreePlacementMap {
        let row_widths= map_data.iter().map(Vec::len).collect::<Vec<usize>>();
        // Borrowed from https://sts10.github.io/2019/06/06/is-all-equal-function.html
        let all_same = row_widths.get(0).map(|first| row_widths.iter().all(|x| x == first)).unwrap_or(true);

        if ! all_same {
            panic!("Rows are not all the same.");
        } 

        let width = *row_widths.get(0).unwrap();
        let height = map_data.len();

        TreePlacementMap { map_data, width, height }
    }

    fn tree_in_location(&self, x: usize, y: usize) -> Result<bool, &str> {
        // println!("tree_in_location: x: {}, y: {}, width: {}, height: {}", x, y, self.width, self.height);

        if y >= self.height {
            // println!("\tReached the bottom of the map.");
            return Err("Reached the bottom of the map.");
        }

        let actual_x = x % self.width;

        // println!("\tactual x: {}", actual_x);
        Ok(self.map_data[y][actual_x].is_tree_in_location())
    }
}

impl Display for TreePlacementMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut sbuffer = String::new();

        for row in self.map_data.iter() {
            for space in row.iter() {
                sbuffer.push_str(space.to_string().as_str());
            }
            sbuffer.push_str("\n");
        }

        write!(f, "{}", sbuffer)
    }
}

impl FromStr for TreePlacementMap {
    type Err = Error;

    fn from_str(map_str: &str) -> Result<Self, Self::Err> {
        let map_data: Vec<Vec<TreeInLocation>> = map_str
            .lines()
            .map(|s| {
                s.trim()
                    .chars()
                    .map(|c| String::from(c).parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(TreePlacementMap::new(map_data))
    }
}

fn get_contents(input_filename: &Path) -> Result<TreePlacementMap, &str> {
    let contents: String = std::fs::read_to_string(input_filename).unwrap_or_else(|_| {
        panic!(
            "Problems reading from file: {}",
            input_filename.to_str().unwrap()
        )
    });

    let map_data: TreePlacementMap = contents.parse().unwrap();
    Ok(map_data) 
}

fn count_trees_in_path(tree_map: &TreePlacementMap, step_x: usize, step_y: usize) -> i32 {
    let mut count: i32 = 0;

    let mut x: usize = 0; 
    let mut y: usize = 0;

    loop {
        match tree_map.tree_in_location(x, y) {
            Ok(true) => count += 1,
            Ok(false) => (),
            Err(_) => return count
        }

        x += step_x;
        y += step_y;
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
    let map_data = get_contents(&input_filename).unwrap();

    let step_x: usize = 3;
    let step_y: usize = 1;

    let tree_count = count_trees_in_path(&map_data, step_x, step_y);
    println!("PART 1: Found {} trees in the path.", tree_count);

    let slopes: Vec<(usize, usize)> = vec!{(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)};

    let count_product: i32 = slopes.iter().map(|(step_x, step_y)| count_trees_in_path(&map_data, *step_x, *step_y)).product();
    println!("PART 2: Product of trees found: {}", count_product);
}
