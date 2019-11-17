use crate::board::Board;
use crate::player::Player;

#[derive(PartialEq, Eq, Clone)]
pub enum State {
    StartGame,
    NextTurn(Player, Board),
    Won(Player),
    Draw,
    EndGame,
}
