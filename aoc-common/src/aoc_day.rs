use crate::files::Res;
use crate::time;
use std::fmt::Display;

pub trait AocDay {
    type Input;
    type Result1: Display;
    type Result2: Display;

    fn day() -> usize;
    fn load() -> Res<Self::Input>;
    fn part_1(input: &Self::Input) -> Self::Result1;
    fn part_2(input: &Self::Input) -> Self::Result2;

    fn run() -> Res<Vec<String>> {
        let mut output = vec![];
        let (input, t_l) = time(Self::load);
        let input = input?;
        let (res_1, t_1) = time(|| Self::part_1(&input));
        let (res_2, t_2) = time(|| Self::part_2(&input));

        output.push(format!("Day {}", Self::day()));
        output.push(format!("  input loaded in {:?}", t_l));
        output.push(format!("  part 1: {} in {:?}", res_1, t_1));
        output.push(format!("  part 2: {} in {:?}", res_2, t_2));
        output.push("".to_string());

        Ok(output)
    }

    fn run_me_maybe(day: Option<usize>) -> Res<Vec<String>> {
        if Some(Self::day()) == day || day.is_none() {
            Self::run()
        } else {
            Ok(vec![])
        }
    }
}
