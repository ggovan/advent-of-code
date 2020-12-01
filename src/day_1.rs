use super::files::{read_lines, Res};

pub fn day_1() -> Res<()> {
    println!("Day 1");
    let day_1_in: Vec<_> = read_lines("day_1.in")?
        .map(|l| l.unwrap().trim().parse::<i32>().unwrap())
        .collect();
    println!("  part 1 {}", day_1_part_1(&day_1_in));
    println!("  part 2 {}", day_1_part_2(&day_1_in));
    Ok(())
}

pub fn day_1_part_1(mass: &[i32]) -> i32 {
    mass.iter().map(|m| m / 3 - 2).sum()
}

fn day_1_part_2(mass: &[i32]) -> i32 {
    mass.iter()
        .map(|m| m / 3 - 2)
        .map(|m| m + day_1_part_2_fuel(m))
        .sum()
}

fn day_1_part_2_fuel(mass: i32) -> i32 {
    let more_fuel = mass / 3 - 2;
    if more_fuel <= 0 {
        0
    } else {
        more_fuel + day_1_part_2_fuel(more_fuel)
    }
}
