use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    let random_num = rand::thread_rng().gen_range(0, 100);

    println!("random number:{}" , &random_num);


    loop {
        println!("guess a number:");

        let mut guess_number = String::new();

        io::stdin().read_line(&mut guess_number).expect("connt read");

        let guess_number:i32 = match guess_number.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
                println!("解析错误");
                continue;
            }
        };
        // let guess_number:i32 = guess_number.trim().parse().expect("解析错误");

        match guess_number.cmp(&random_num) {
            Ordering::Equal => {println!("Equal")},
            Ordering::Greater => {println!("Greater")},
            Ordering::Less => {
                println!("Less");
                break;
            }
        }
    }
    

    // print!("guess_number: {}", &guess_number)
}
