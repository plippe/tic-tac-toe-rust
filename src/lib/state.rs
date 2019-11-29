use crate::lib::board::Board;
use crate::lib::player::Player;

#[derive(PartialEq, Eq, Clone)]
pub enum State {
    StartGame,
    NextTurn(Player, Board),
    Won(Player),
    Draw,
    EndGame,
}
