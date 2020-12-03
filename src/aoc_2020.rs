pub mod day_02;
pub mod day_03;
pub mod day_1;
use crate::files::Res;

pub fn run_all(day: Option<usize>) -> Res<()> {
    if Some(1) == day || day.is_none() {
        day_1::day_1()?;
    }

    if Some(2) == day || day.is_none() {
        day_02::day_02()?;
    }

    if Some(3) == day || day.is_none() {
        day_03::day_03()?;
    }

    Ok(())
}
