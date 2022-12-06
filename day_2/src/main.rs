use std::io::{BufReader, BufRead};
use std::fs::File;


#[derive(Copy, Clone, PartialEq)]
enum RPS
{
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3
}
use RPS::*;


fn move_score(x: RPS) -> i32
{
    return match x {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
    };
}

fn their_movetype(x: &str) -> RPS
{
    return match x {
        "A" => ROCK,
        "B" => PAPER,
        "C" => SCISSORS,
        _ => panic!("bad option: {}", x)
    }
}

fn my_movetype(x: &str) -> RPS
{
    return match x {
        "X" => ROCK,
        "Y" => PAPER,
        "Z" => SCISSORS,
        _ => panic!("bad option: {}", x)
    }
}

#[derive(PartialEq)]
enum WinState
{
    WIN,
    DRAW,
    LOSE
}
use WinState::*;


fn win_state(a: RPS, b: RPS) -> WinState
{
    match a 
    {
        ROCK => return match b
        {
            ROCK => DRAW,
            PAPER => LOSE,
            SCISSORS => WIN,
        },
        PAPER => return match b
        {
            ROCK => WIN,
            PAPER => DRAW,
            SCISSORS => LOSE,
        },
        SCISSORS => return match b
        {
            ROCK => LOSE,
            PAPER => WIN,
            SCISSORS => DRAW,
        }
    }
}

fn choose_move(their_move: RPS, state: &str) -> RPS
{
    let request_state = match state
    {
        "X" => LOSE,
        "Y" => DRAW,
        "Z" => WIN,
        _ => panic!("bad option: {}", state)
    };

    match their_move
    {
        ROCK => return match request_state
        {
            WIN => PAPER,
            DRAW => ROCK,
            LOSE => SCISSORS,
        },
        PAPER => return match request_state
        {
            WIN => SCISSORS,
            DRAW => PAPER,
            LOSE => ROCK,
        },
        SCISSORS => return match request_state
        {
            WIN => ROCK,
            DRAW => SCISSORS,
            LOSE => PAPER,
        }
    }
}


#[derive(PartialEq)]
enum GameType
{
    IsMove,
    IsWinState,
}
use GameType::*;


fn run_game(game_type: GameType) {
    let path = "data/input.txt";
    let file = match File::open(&path)
    {
        Err(why) => panic!("can't open {}: {}", path, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut their_score = 0;
    let mut my_score = 0;

    for line in reader.lines()
    {
        let unwrapped_line = line.unwrap();
        let (their_movestr, my_movestr) = unwrapped_line.split_once(" ").unwrap();

        let their_move = their_movetype(their_movestr);
        let my_move = if game_type == IsWinState { choose_move(their_move, my_movestr) } else { my_movetype(my_movestr) };

        their_score += move_score(their_move);
        my_score += move_score(my_move);

        let state = win_state(my_move, their_move);
        if state == WIN
        {
            my_score += 6;
        }
        else if state == LOSE
        {
            their_score += 6;
        }
        else
        {
            their_score += 3;
            my_score += 3;
        }
    }
    println!("My score: {}, Their score: {}", my_score, their_score);
}


fn main()
{
    run_game(IsMove);
    run_game(IsWinState);
}