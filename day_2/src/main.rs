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
    for row in input {
        if check_safety(&row) {
            safe_rows += 1;
        }
    }

    Ok(safe_rows)
}

fn check_safety(row: &Vec<u32>) -> bool {
    let mut maybe_ignored_index: Option<usize> = None;
    let mono_safety = check_safety_monotony(row, &mut maybe_ignored_index);
    let discrepency_safety = check_safety_discrepency(row, &mut maybe_ignored_index);

    mono_safety && discrepency_safety
}

enum Monotony {
    Increasing,
    Decreasing,
}

fn check_safety_monotony(row: &Vec<u32>, maybe_ignored_index: &mut Option<usize>) -> bool {
    let mut maybe_previous: Option<u32> = None;
    let mut maybe_mono_type: Option<Monotony> = None;
    for (index, i) in row.iter().enumerate() {
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
                            if maybe_ignored_index.is_none() {
                                *maybe_ignored_index = Some(index);
                                maybe_previous = Some(*i);
                            } else {
                                return false;
                            }
                        } else if let Some(ignored_index) = maybe_ignored_index {
                            if *ignored_index != index {
                                return false;
                            } else {
                                maybe_previous = Some(*i);
                            }
                        } else {
                            maybe_previous = Some(*i);
                        }
                    }
                    Some(Monotony::Decreasing) => {
                        if i > &previous {
                            if maybe_ignored_index.is_none() {
                                *maybe_ignored_index = Some(index);
                                maybe_previous = Some(*i);
                            } else {
                                return false;
                            }
                        } else if let Some(ignored_index) = maybe_ignored_index {
                            if *ignored_index != index {
                                return false;
                            } else {
                                maybe_previous = Some(*i);
                            }
                        } else {
                            maybe_previous = Some(*i);
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

fn check_safety_discrepency(row: &Vec<u32>, maybe_ignored_index: &mut Option<usize>) -> bool {
    let mut maybe_previous: Option<u32> = None;
    for (index, i) in row.iter().enumerate() {
        match maybe_previous {
            Some(previous) => {
                let abs_diff = previous.abs_diff(*i);
                if !(abs_diff >= 1 && abs_diff <= 3) {
                    if maybe_ignored_index.is_none() {
                        *maybe_ignored_index = Some(index);
                        maybe_previous = Some(*i);
                    } else {
                        return false;
                    }
                } else if let Some(ignored_index) = maybe_ignored_index {
                    if *ignored_index != index {
                        return false;
                    } else {
                        maybe_previous = Some(*i);
                    }
                } else {
                    maybe_previous = Some(*i);
                }
            }
            None => {
                maybe_previous = Some(*i);
            }
        }
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Result: {}", part_1()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::check_safety_monotony;

    #[test]
    fn test_mono_increase() {
        let result = check_safety_monotony(&vec![1, 2, 3, 4, 5, 6], &mut None);
        assert_eq!(result, true)
    }

    #[test]
    fn test_mono_increase_fail() {
        let result = check_safety_monotony(&vec![1, 2, 1, 4, 5, 6], &mut None);
        assert_eq!(result, false)
    }

    #[test]
    fn test_mono_decrease() {
        let result = check_safety_monotony(&vec![6, 5, 4, 3, 2, 1], &mut None);
        assert_eq!(result, true)
    }

    #[test]
    fn test_mono_decrease_fail() {
        let result = check_safety_monotony(&vec![6, 5, 4, 6, 2, 1], &mut None);
        assert_eq!(result, false)
    }
}
