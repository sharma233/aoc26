advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let commands = input.split("\n");
    let mut curr_num = 50;
    let mut zero_count = 0;
    for command in commands  {
        if command.is_empty() {
            continue;
        }
        let command_num:i32 = if command.contains("L") {
            command.replace("L","-").parse::<i32>().unwrap() % 100
        } else {
            command.replace("R","").parse::<i32>().unwrap() % 100
        };

        curr_num = curr_num + command_num;
        if curr_num > 99 {
            curr_num = curr_num - 100
        } else if curr_num < 0 {
            curr_num = curr_num + 100
        }
        if curr_num == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
