extern crate rand;

use rand::Rng;
use rand::distributions::{Distribution, Uniform, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random i32: {}", rng.gen::<i32>());

    println!("Float: {}", rng.gen_range(0.0, 10.0));

    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);

    const CHARSET: &[u8] = b"ABCDEFGhijklmn0123456789)(+-*$%^";
    const PASSWORD_LEN: usize = 15;
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("{:?}", password);
}
