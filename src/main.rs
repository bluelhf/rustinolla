mod game;

use game::Game;
use game::State;

use std::cmp::*;
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
    let mut game: Game = Game::new(3, 2);
    let auto: bool = true;

    clear();
    println!( indoc! {"
        Welcome to Rustinolla!
        You are playing with {a} players on a {n}x{n} board
        To take a slot, simply enter its coordinates separated
        by commas when requested (e.g. {m},{m} for the middle)
        
        Have fun!
    "}, a = game.players, n = game.length, m = game.length / 2 + 1);

    loop {
        loop {
            println!("now playing: {}", game.current_symbol());
            game.show("current");

            if game.current_player != 1 && auto {
                let mut best = game.minimax(-1);
                game.place(best.0, best.1);
            } else {
                let mut input = String::new();
                print!("player {}, enter column,row: ", game.current_symbol());
                
                flush();
                stdin().read_line(&mut input).expect("cannot read stdin, what's going on?");
    
                // rust doesn't let us convert this iterator to a tuple...
                let mut it = input.splitn(2, ",")
                    .map(|s| s.trim().parse::<usize>());
                match (it.next(), it.next()) {
                    (Some(Ok(x)), Some(Ok(y))) => {
                        if max(x, y) > game.length || min(x, y) < 1 {
                            println!("invalid input");
                            continue;
                        }
    
                        match game.place(x - 1, y - 1) {
                            Ok(()) => break,
                            Err(_) => println!("that position is occupied"),
                        };
                    },
                    _ => println!("invalid input"),
                };
            }
        }

        match game.check() {
            State::TIE => {
                game.show("tying");
                println!("game was a tie.");
                break;
            },
            State::WINNER(winner) => {
                game.show("winning");
                println!("player {} wins!", game.to_str(winner));
                break;
            },
            _ => {},
        };

        clear();
    }
}
