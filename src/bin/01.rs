use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for (_index,line) in input.lines().enumerate(){
        let digits = find_digits(line);
        let first_digit = digits.first();
        let last_digit = digits.last();
        let number = match first_digit {
            Some(first) => {
                let last = last_digit.unwrap_or(first);
                Some(first * 10 + last)
            }
            _ => None,
        };
        sum = sum + match number {
            None => {0}
            Some(number) => {number}
        }
    }
    return Option::from(sum);
}

fn find_digits(input: &str) -> Vec<u32> {
    input.chars()
        .filter_map(|ch| ch.to_digit(10))
        .collect()
}

fn find_digits_with_index(input: &str) -> Vec<(usize, u32)> {
    input.char_indices()
        .filter_map(|(idx, ch)| ch.to_digit(10).map(|num| (idx, num)))
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    let words: Vec<_> = map.keys().collect();

    for (_index,line) in input.lines().enumerate(){
        let mut digits = find_digits_with_index(line);
        for  word in words.clone() {
            let mut last_pos = 0;
            while let Some(pos) = line[last_pos..].find(word) {
                digits.push((last_pos + pos,*map.get(word).unwrap()));
                last_pos += pos + 1;
            }
        }
        digits.sort_by(|a, b| a.0.cmp(&b.0));
        // for digit in &digits {
        //     println!("{:?}", digit);
        // }
        // Get and print the first and last values.
        let first = digits.first();

        let last= digits.last();

        let number = match first {
            Some(&(_, first_num)) => {
                let last_num = match last {
                    Some(&(_, num)) => num,
                    _ => first_num,
                };
                Some(first_num * 10 + last_num)
            }
            _ => None,
        };
        println!("{}",number.unwrap());
        sum = sum + match number {
            None => {0}
            Some(number) => {number}
        }
    }
    return Option::from(sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_mine() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part_one(input), Some(142));
    }

    #[test]
    fn test_part_one_mine_1() {
        let test_input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part_two(test_input), Some(281));
    }
}
