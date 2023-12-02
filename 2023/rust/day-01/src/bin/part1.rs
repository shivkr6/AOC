fn first_num_in_line(input: &str) -> char {
    for c in input.chars() {
        if c.is_numeric() {
            return c;
        }
    }
    return ' ';
}
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn line_ans(s: &str) -> u128 {
    let first_numb = first_num_in_line(s);
    let s1 = reverse(s);
    let second_numb = first_num_in_line(&s1);
    let both_nums_in_str = format!("{}{}", first_numb, second_numb);
    let both_nums: u128 = both_nums_in_str.parse().unwrap();
    return both_nums;
}

fn part1(input: &str) -> u128 {
    let lines = input.lines().collect::<Vec<&str>>();
    let numbers = lines.into_iter().map(|s| line_ans(s)).collect::<Vec<u128>>();
    let final_ans: u128 = numbers.iter().sum();
    return final_ans;
}


fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{}", output)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        let result = part1(String::from("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"));
        assert_eq!(result, 142);
    }
}
