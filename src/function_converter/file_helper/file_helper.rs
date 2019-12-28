extern crate rand;
use rand::distributions::Alphanumeric;
use rand::Rng;

fn print_random_alphanum() {
    let s = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>();

    println!("random string: {}", s);
}