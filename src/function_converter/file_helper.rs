extern crate rand;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn print_random_alphanum() -> String {
    let outputString = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>();
    outputString
}