use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Let's play a guessing game!");
    let mut playing = true;
    while playing {
        play_one_round();
        playing = ask_to_play_again();
        if playing {
            println!("\nGreat!\n");
        }
    }
}

fn ask_to_play_again() -> bool {
    let answer = prompt("Play again? (Y/n)");
    return answer.len() == 0 ||
        answer.to_lowercase().starts_with("y")
}

fn play_one_round() {
    let expected = rand::thread_rng().gen_range(1, 101);
    println!("I'm thinking of a number... can you guess what it is?");
    let max_guesses = 10;
    for i in 0..max_guesses {
        let attempts_remaining = max_guesses - i - 1;
        let guess = read_guess(attempts_remaining);
        println!("You guessed: {}", guess);

        match guess.cmp(&expected) {
            Ordering::Less => {
                perhaps_try_again(
                    attempts_remaining,
                    "try a little higher!",
                    expected,
                );
            }
            Ordering::Equal => {
                println!("winner, winner, chicken dinner");
                return;
            }
            Ordering::Greater => {
                perhaps_try_again(
                    attempts_remaining,
                    "try a little lower!",
                    expected,
                );
            }
        }
    }
}

fn perhaps_try_again(
    attempts_remaining: u8,
    when_have_attempts: &str,
    answer: i32) {
    println!("{}",
             determine_message_for(
                 attempts_remaining,
                 when_have_attempts,
                 answer,
             )
    );
}

fn determine_message_for(
    attempts_remaining: u8,
    when_have_attempts: &str,
    answer: i32) -> String {
    return match attempts_remaining {
        0 => format!("You're outta luck\nThe answer was: {}", answer),
        _ => {
            let mut result = String::new();
            result.push_str(when_have_attempts);
            return result;
        }
    };
}

fn prompt(message: &str) -> String {
    print!("{} > ", message);
    io::stdout().flush()
        .expect("Unable to flush stdout");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Unable to read from stdin");
    return answer.trim().to_string();
}

fn read_guess(attempts_remaining: u8) -> i32 {
    let s = match attempts_remaining {
        1 => "",
        _ => "s"
    };

    let attempts = match attempts_remaining {
        0 => "last chance".to_string(),
        _ => format!("{} attempt{} remaining", attempts_remaining, s)
    };

    loop {
        let guess = prompt(
            format!("Enter a number (1-100) [{}]", attempts).as_str()
        );

        if guess.as_str() == "quit" {
            std::process::exit(1);
        }

        let guess = match guess.trim().parse::<i32>() {
            Ok(number) => number,
            Err(_) => continue
        };
        return guess;
    }
}
