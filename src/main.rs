use std::io;
use std::str::FromStr;

mod lib;

use crate::lib::board::Board;
use crate::lib::coordinates::Coordinates;
use crate::lib::game::Game;
use crate::lib::player::Player;
use crate::lib::state::State;

fn start_game(game: &Game) -> State {
    State::NextTurn(Player::first(), Board::new(game))
}

fn next_turn(game: &Game, player: &Player, board: &Board) -> State {
    println!("Player {:?}'s turn", player);
    println!("{}", board.to_string());
    println!("");
    println!("Where would you like to play ?");
    read_input::<Coordinates>()
        .and_then(|coordinates| {
            board
                .insert(&coordinates, &player)
                .map(|board| (board, coordinates))
        })
        .map(|(new_board, coordinates)| {
            if new_board.is_winning_move(&coordinates, game.goal) {
                State::Won(player.clone())
            } else if new_board.is_draw() {
                State::Draw
            } else {
                State::NextTurn(player.next(), new_board)
            }
        })
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            println!("Try again ?");
            match read_input::<bool>().unwrap_or(false) {
                true => State::NextTurn(player.clone(), board.clone()),
                false => State::EndGame,
            }
        })
}

fn draw() -> State {
    println!("Game finished with a draw");
    State::EndGame
}

fn won(player: &Player) -> State {
    println!("Game finished and {:?} won", player);
    State::EndGame
}

fn end_game() -> State {
    println!("Game finished");
    State::EndGame
}

fn turn(game: &Game, state: &State) -> State {
    match state {
        State::StartGame => start_game(game),
        State::NextTurn(player, board) => next_turn(game, player, board),
        State::Draw => draw(),
        State::Won(player) => won(player),
        State::EndGame => end_game(),
    }
}

fn read_input<A: FromStr>() -> Result<A, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Input can't be read: {}", e))
        .and_then(|_| {
            input
                .trim()
                .parse::<A>()
                .map_err(|_| "Input can't be parsed".to_string())
        })
}

fn main() {
    let game = Game::TIC_TAC_TOE;
    let mut state = State::StartGame;

    while state != State::EndGame {
        state = turn(&game, &state);
    }
}
