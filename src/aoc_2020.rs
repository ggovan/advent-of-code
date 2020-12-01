mod day_1;
use crate::files::Res;

pub fn run_all() -> Res<()> {
    day_1::day_1()?;

    Ok(())
}
