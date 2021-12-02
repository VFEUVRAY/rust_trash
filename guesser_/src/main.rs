use std::env;
use env::Args;
use rand::Rng;
use std::cmp::Ordering;
use std::str;

fn get_guess_range() -> u32 {
    let mut answer = String::new();
    let mut check;
    let mut num;
    loop {
        println!("Please choose max number to be chosen from");
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read");
        check = answer.trim_end_matches('\n').parse::<u32>();
        if check.is_ok() {
            num = check.unwrap();
            if num > 1 {
                return num;
            }
        }
        if answer.contains("quit") {
            return 0;
        }
        println!("Please choose a valid number (>1)");
        answer.clear();
    }
}

fn process_guess(buff: &String, correct: &u32, tries: &mut u32) -> bool {
    let guess = buff.trim().parse::<u32>();
    if guess.is_ok() {
        *tries += 1;
        match guess.unwrap().cmp(correct) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => return true,
            Ordering::Greater => println!("Too big!")
        }
    } else {
        println!("Please enter a valid number");
    }
    return false;
}

fn save_game(correct: u32, tries: u32) -> bool{
    let data = format!("CORRECT={}\nTRIES={}\n", correct, tries);
    let check = std::fs::write("save.vs", data).is_ok();
    match check {
        true => println!("Game was successfully saved"),
        false => println!("Could not save game")
    }
    return check;
}

fn split_save_get(lines: &Vec<&str>, arg: &str) -> Option<i32> {
    for line in lines {
        match line.find('=') {
            Some(value) => {
                if line.starts_with(&arg) {
                    match line[value+1..].parse::<i32>() {
                        Ok(res) => return Some(res),
                        Err(_) => return None
                    }
                }
            },
            None => continue
        };
    }
    return None;
}

fn get_data_from_save() -> (i32, i32) {
    let read_ok = std::fs::read_to_string("./save.vs");
    match read_ok.is_ok() {
        true => {
            let buff = read_ok.unwrap();
            let lines: Vec<&str> = buff.split('\n').collect();
            let correct = split_save_get(&lines, "CORRECT");
            let tries = split_save_get(&lines, "TRIES");
            println!("LOADED CORRECT {}", correct.unwrap());
            match tries.is_some() && correct.is_some() {
                true => return (correct.unwrap(), tries.unwrap()),
                false => return (-1, -1)
            }
        },
        false => {
            println!("Could not load save data");
            return (-1, -1);
        }
    };
}

fn load_game() -> (i32, i32) {
    let mut buff = String::new();
    let stdin = std::io::stdin();
    loop {
        println!("Do you wish to load a saved game?");
        stdin.read_line(&mut buff).expect("failed to read");
        if buff.trim().eq("y") {
            return get_data_from_save();
        } else if buff.trim().eq("n") {
            break
        }
        println!("Bad choice");
        buff.clear();
    }
    return (-1, -1);
}

fn guesser(correct: u32, mut tries: u32) -> i32{
    let stdin = std::io::stdin();
    let mut buff = String::new();
    println!("ANSWER IS: {}", correct);
    loop {
        println!("Current tries: {}", tries);
        stdin.read_line(&mut buff).expect("Failed to read line");
        if buff.eq("quit\n") {
            save_game(correct, tries);
            println!("Bye bye");
            return 1;
        }
        match process_guess(&buff, &correct, &mut tries) {
            true => break,
            false => buff.clear()
        }
    }
    println!("GGs");
    return 0;
}

fn print_args() {
    let mut env_args: Args = env::args();
    let mut args: Option<String> = env_args.next();
    let mut current: String;
    let mut num: i32;
    while args != None {
        current = args.unwrap();
        let err = current.parse::<i32>();
        if err.is_ok() {
            num = err.unwrap();
            println!("We have a number {}", num);
        } else {
            println!("{}", current);
        }
        args = env_args.next();
    }
}

fn prepare_game() -> Option<(i32, i32)> {
    let save_pair = load_game();
    if save_pair.0 == (-1) {
        let max = get_guess_range();
        match max == 0 {
            true => return None,
            false => {
                let correct = rand::thread_rng().gen_range(1..max);
                return Some((correct as i32, 0))
            }
        }
    }
    return Some(save_pair);
}

fn real_main() -> i32 {
    print_args();
    let save_pair = prepare_game();
    match save_pair.is_some() {
        true => {
            let save_pair = save_pair.unwrap();
            if guesser(save_pair.0 as u32, save_pair.1 as u32) == 0 {
                std::fs::remove_file("./save.vs").unwrap_or_default();
                return 0;
            }
        }
        false => {
            println!("Bye bye");
            return 1;
        }
    }
    return 1;
}

fn main() {
    std::process::exit(real_main());
}