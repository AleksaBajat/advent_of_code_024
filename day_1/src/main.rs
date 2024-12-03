use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::iter::zip;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let answ_1 = part_1()?;
    let answ_2 = part_2()?;

    println!("Answer 1: {:?}\nAnswer 2: {:?}", answ_1, answ_2);
    Ok(())
}

fn get_lists<P>(input_file: P) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let input: String = fs::read_to_string(input_file)?;
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut left_right = line.split_whitespace();
        let left: u32 = left_right.next().expect("").parse()?;
        let right: u32 = left_right.next().expect("").parse()?;
        first_list.push(left);
        second_list.push(right);
    }

    Ok((first_list, second_list))
}

fn part_1() -> Result<u32, Box<dyn Error>> {
    let (mut first_list, mut second_list) = get_lists("input_1.txt")?;
    first_list.sort();
    second_list.sort();

    let zipper = zip(first_list, second_list);
    let mut result: u32 = 0;

    for zz in zipper {
        result += zz.0.abs_diff(zz.1);
    }

    Ok(result)
}

fn part_2() -> Result<u32, Box<dyn Error>> {
    let (first_list, second_list) = get_lists("input_2.txt")?;
    let hist = histogram(second_list)?;
    let mut result: u32 = 0;
    for i in first_list {
        if let Some(freq) = hist.get(&i) {
            result += i * freq
        }
    }

    Ok(result)
}

fn histogram(input: Vec<u32>) -> Result<HashMap<u32, u32>, Box<dyn Error>> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for i in input {
        if let Some(stat) = map.get_mut(&i) {
            *stat += 1;
        } else {
            map.insert(i, 1);
        }
    }

    Ok(map)
}
