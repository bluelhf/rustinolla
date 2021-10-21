mod game;
mod occupied_error;

use game::Game;
use occupied_error::OccupiedError;
use std::io::Write;
use std::io::stdout;
use std::io::stdin;

use indoc::indoc;

fn clear() {
    /* stupid windows stdout doesn't support ANSI
     * rust might add support for ANSI on windows later,
     * but for now, we're stuck with not clearing on windows
     */
    if !cfg!(windows) {
        print!("{}[2J", 27 as char);
        flush();
    }
}

fn flush() {
    stdout().flush().expect("cannot flush stdout, what's going on?");
}

fn main() {
    let mut game: Game = Game::new(5, 3);

    clear();
    println!( indoc! {"
        Welcome to Rustinolla!
        You are playing with {a} players on a {n}x{n} board
        To take a slot, simply enter its coordinates separated
        by commas when requested (e.g. {m},{m} for the middle)
        
        Have fun!
    "}, a = game.players, n = game.length, m = game.length / 2 + 1);

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
                    /* i know that this and the corresponding line for the y-input
                     * break if the user enters a 0.. i'll fix that later
                     *
                     * this part of the code was kind of shoved in last-minute
                     * anyways. hoping noone notices !
                     */
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
