use std::env;
use std::fmt::{Error};
use std::fs;

fn read_input(filename: String) -> std::io::Result<String> {
    let path = env::current_dir()?.join(filename);

   fs::read_to_string(path)
}

#[derive(Debug)]
struct Item<'a> {
    min: i32,
    max: i32,
    x: char,
    password: &'a str
}

fn parse(contents: &String) -> Result<Vec<Item>, Error>{
    let lines = contents.lines();

    let items: Vec<Item> = lines.into_iter().map(|x|parse_item(x).unwrap()).collect();

    Ok(items)
}

fn parse_item(line: &str) ->  Result<Item, Error> {
    let parts: Vec<&str> = line.split(": ").collect();

    let password = parts[1];
    let parts2:  Vec<&str> = parts[0].split(" ").collect();
    let x = parts2[1].chars().last().unwrap();
    let minmax_parts: Vec<&str> = parts2[0].split("-").collect();
    let min = minmax_parts[0].parse::<i32>().unwrap();
    let max = minmax_parts[1].parse::<i32>().unwrap();

    Ok(Item {
        min,
        max,
        x,
        password
    })
}

fn is_valid(item: &Item) -> bool {
    let letter_count = item.password.chars().filter(|c| c == &item.x).count();

    letter_count >= item.min as usize && letter_count <= item.max as usize
}

fn main() {
    let input = read_input(String::from("input.txt")).unwrap();
    let items = parse(&input).unwrap();

    let valid_items = items.iter().filter(|i| is_valid(i)).count();
    println!("{}", valid_items);
}
