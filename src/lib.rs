use rand::Rng;

pub mod game;
pub mod gui;
pub mod tiles;

pub fn get_random_number(start_range: i32, end_range: i32) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(start_range..end_range)
        .try_into()
        .expect("Cannot convert to usize")
}
