use crate::files::{read_lines, Res};

pub fn day_1() -> Res<()> {
    println!("Day 1");

    let input: Vec<_> = load()?;

    println!("  part 1: {}", part_1(&input, 2020).expect("Impossible"));
    println!("  part 2: {}", part_2(&input));

    Ok(())
}

pub fn load() -> Res<Vec<i32>> {
    let mut input: Vec<_> = read_lines("data/2020/day_01.in")?
        .map(|l| l.unwrap().trim().parse::<i32>().unwrap())
        .collect();
    input.sort();
    Ok(input)
}

pub fn part_1(expenses: &[i32], goal: i32) -> Option<i32> {
    let mut bottom = 0;
    let mut top = expenses.len() - 1;
    while top != bottom {
        let res = expenses[bottom] + expenses[top];
        match res {
            x if x == goal => return Some(expenses[bottom] * expenses[top]),
            x if x > goal => top -= 1,
            _ => bottom += 1,
        }
    }
    None
}

pub fn part_2(expenses: &[i32]) -> i32 {
    for i in 0..expenses.len() - 2 {
        if let Some(res) = part_1(&expenses[i + 1..], 2020 - expenses[i]) {
            return res * expenses[i];
        }
    }
    panic!("Impossible");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let res = part_1(&input, 2020).unwrap();
        assert_eq!(res, 514579);
    }

    #[test]
    fn test_part_2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let res = part_2(&input);
        assert_eq!(res, 241861950);
    }
}
