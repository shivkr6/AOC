fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn str_to_num(s: &str) -> Option<u8> {
    if s.contains("one") {
        Some(1)
    } else if s.contains("two") {
        Some(2)
    } else if s.contains("three") {
        Some(3)
    } else if s.contains("four") {
        Some(4)
    } else if s.contains("five") {
        Some(5)
    } else if s.contains("six") {
        Some(6)
    } else if s.contains("seven") {
        Some(7)
    } else if s.contains("eight") {
        Some(8)
    } else if s.contains("nine") {
        Some(9)
    } else {
        None
    }
}

fn first_str_num(input: &str) -> Option<u8> {
    let mut sliced_input = String::new();
    for c in input.chars() {
        sliced_input.push(c);
        if c.is_numeric() {
            return Some(c.to_digit(10).unwrap() as u8);
        }
        let string_to_num = str_to_num(&sliced_input);
        if string_to_num != None {
            return Some(string_to_num.unwrap());
        }
    }
    None
}

fn second_str_num(input: &str) -> Option<u8> {
    let reversed_input = reverse(input);
    let mut sliced_input = String::new();
    for c in reversed_input.chars() {
        sliced_input.insert(0, c);
        if c.is_numeric() {
            return Some(c.to_digit(10).unwrap() as u8);
        }
        let string_to_num = str_to_num(&sliced_input);
        if string_to_num != None {
            return Some(string_to_num.unwrap());
        }
    }
    None
}

fn line_ans(s: &str) -> u128 {
    let first_numb = first_str_num(s).unwrap();
    let second_numb = second_str_num(s).unwrap();
    let both_nums_in_str = format!("{}{}", first_numb, second_numb);
    let both_nums: u128 = both_nums_in_str.parse().unwrap();
    return both_nums;
}

fn part2(input: &str) -> u128 {
    let lines = input.lines().collect::<Vec<&str>>();
    let numbers = lines.into_iter().map(|s| line_ans(s)).collect::<Vec<u128>>();
    let final_ans: u128 = numbers.iter().sum();
    return final_ans;
}



fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    println!("{}", output)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_first_str_num() {
        let result = first_str_num(&String::from("3two1nine")).unwrap();
        assert_eq!(result, 3);
    }
    #[test]
    fn testing_final_ans() {
        let input = String::from("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
");
        let result = part2(&input);
        assert_eq!(result, 281);
    }
}
