use std::{error::Error, fs::read_to_string, path::Path};

fn read_input<P>(path: P) -> Result<Vec<Vec<u32>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let input = read_to_string(path)?;
    let mut result: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let fragments = line.split_whitespace();
        let mut res = Vec::new();

        for fragment in fragments {
            let fragment: u32 = fragment.parse().expect("");
            res.push(fragment);
        }

        result.push(res);
    }

    Ok(result)
}

fn part_1() -> Result<u32, Box<dyn Error>> {
    let input: Vec<Vec<u32>> = read_input("input_1.txt")?;
    let mut safe_rows: u32 = 0;
}

fn check_safety(row: &Vec<u32>) {
    check_safety_monotony(row) && check_safety_discrepency(row);
}

enum Monotony {
    Increasing,
    Decreasing,
}

fn check_safety_monotony(row: &Vec<u32>) -> bool {
    let mut maybe_previous: Option<u32> = None;
    let mut maybe_mono_type: Option<Monotony> = None;
    for i in row {
        match maybe_previous {
            Some(previous) => {
                if maybe_mono_type.is_none() {
                    if i > &previous {
                        maybe_mono_type = Some(Monotony::Increasing)
                    } else {
                        maybe_mono_type = Some(Monotony::Decreasing)
                    }
                }

                match maybe_mono_type {
                    Some(Monotony::Increasing) => {
                        if i < &previous {
                            return false;
                        }
                    }
                    Some(Monotony::Decreasing) => {
                        if i > &previous {
                            return false;
                        }
                    }
                    None => panic!("Shouldn't happen."),
                }
            }
            None => {
                maybe_previous = Some(*i);
            }
        }
    }

    true
}

fn check_safety_discrepency(row: &Vec<u32>) -> bool {}

fn main() {}
