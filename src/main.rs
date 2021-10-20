mod game;
mod occupied_error;

use game::Game;
use occupied_error::OccupiedError;
use std::io::Write;
use std::io::stdout;
use std::io::stdin;


fn main() {
    let mut game: Game = Game::new(5, 3);

    loop {
        let mut pos: [usize; 2] = [0 as usize; 2];
        loop {
            println!("");
            println!("now playing: {}", game.to_str(game.current_player()));
            game.show_current();

            print!("player {}, enter column,row: ", game.to_str(game.current_player()));
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();

            let split: Vec<&str> = input.split(",").collect();
            match split[0].trim().parse::<usize>() {
                Ok(i) => {
                    if i > game.length() {
                        println!("invalid input for x");
                        continue;
                    }
                    pos[0] = i - 1;
                },
                Err(_) => {println!("invalid input for x"); continue},
            };

            match split[1].trim().parse::<usize>() {
                Ok(i) => {
                    if i > game.length() {
                        println!("invalid input for y");
                        continue;
                    }

                    pos[1] = i - 1;
                },
                Err(_) => {println!("invalid input for y"); continue},
            };

            match game.place(pos[0], pos[1]) {
                Ok(()) => {break;},
                Err(_) => println!("that position is occupied"),
            };
        }

        let out = game.check();
        match out {
            -1 => {},
            -2 => {
                game.show("tying");
                println!("game was a tie.");
                break;
            },
            _ => {
                game.show("winning");
                println!("player {} wins!", game.to_str(out as u8));
                break;
            }
        }
    }
}
