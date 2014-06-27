use std::io;
use std::rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = std::rand::task_rng().gen_range(1i, 101);
    println!("Secret number is {}", secret_number);

    let mut guesses: int = 0;
    let mut reader = io::stdin();

    loop {
        println!("Please input guess number {}", guesses + 1);

        let input = reader.read_line().ok().expect("Failed to read line");
        let input_num = from_str::<int>(input.as_slice().trim());

        let num = match input_num  {
            Some(num) => num,
            None      => {
                println!("Please input a number.");
                continue;
            }
        };

        println!("You guessed: {}", num);
        guesses += 1;

        match num.cmp(&secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => {
                println!("You win!");
                break;
            },
        }
    }

    println!("You took {} guesses!", guesses);
}

