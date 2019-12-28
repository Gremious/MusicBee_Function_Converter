extern crate rand;
use rand::Rng;
use rand::distributions::Alphanumeric;

fn printRandomAlphanum() {
    let s = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>();

    println!("random string: {}", s);
}