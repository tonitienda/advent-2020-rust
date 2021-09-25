use core::panic;
use std::env;
use std::fs;

fn part_2(filename: String) -> std::io::Result<i32> {
   let path = env::current_dir()?.join(filename);

   let contents = fs::read_to_string(path)?;

   let array: Vec<i32> = contents.split("\r\n")
.map(|x| x.parse::<i32>().unwrap() ).collect();

  for (i, x) in array.iter().enumerate() {
    for (j, y) in array.iter().enumerate() {
        for (k, z) in array.iter().enumerate() {
            if i != j && j != k && i != k && x + y + z == 2020 {
                return Ok(x * y * z)
            }
        }
    }
  }

  panic!("Noo!")
}

#[allow(dead_code)]
fn part_1(filename: String) -> std::io::Result<i32> {
    let path = env::current_dir()?.join(filename);

    println!("{:?}", path);
    // Read file
    let contents = fs::read_to_string(path)?;

    // Parse to int[]
    let array: Vec<i32> = contents
        .split("\r\n")
        .map(|x| {
            println!("{}", x);
            x.parse::<i32>().unwrap()
        })
        .collect();

    // Find x + y = 2020
    for (i, x) in array.iter().enumerate() {
        for (j, y) in array.iter().enumerate() {
            if i != j && x + y == 2020 {
                return Ok(x * y);
            }
        }
    }

    panic!()
}

fn main() {
    let result = part_2(String::from("input.txt")).unwrap();

    println!("{}", result);
}
