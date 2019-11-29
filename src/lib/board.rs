use itertools::Itertools;
use std::collections::HashMap;

use crate::lib::coordinates::Coordinates;
use crate::lib::game::Game;
use crate::lib::player::Player;

#[derive(PartialEq, Eq, Clone)]
pub struct Board {
    hash: HashMap<Coordinates, Player>,

    min_x: i8,
    max_x: i8,
    min_y: i8,
    max_y: i8,
}

impl Board {
    fn on_board(&self, coordinates: &Coordinates) -> bool {
        coordinates.x >= self.min_x
            && coordinates.x <= self.max_x
            && coordinates.y >= self.min_y
            && coordinates.y <= self.max_y
    }

    pub fn new(game: &Game) -> Board {
        let hash = HashMap::new();
        Board {
            hash,
            min_x: game.min_x,
            max_x: game.max_x,
            min_y: game.min_y,
            max_y: game.max_y,
        }
    }

    pub fn insert(&self, coordinates: &Coordinates, player: &Player) -> Result<Board, String> {
        if !self.on_board(coordinates) {
            Err("OutOfBounds".to_string())
        } else if self.hash.contains_key(coordinates) {
            Err("AlreadyDefined".to_string())
        } else {
            let mut hash = self.hash.clone();
            hash.insert(coordinates.clone(), player.clone());

            Ok(Board {
                hash,
                ..self.clone()
            })
        }
    }

    pub fn is_draw(&self) -> bool {
        let cell_amount = (self.min_x..=self.max_x).len() * (self.min_y..=self.max_y).len();
        self.hash.len() >= cell_amount
    }

    fn affected_rows(&self, coordinates: &Coordinates) -> Vec<Vec<Coordinates>> {
        let x_size = self.max_x - self.min_x;
        let xs = -x_size..=x_size;
        let y_size = self.max_y - self.min_y;
        let ys = -y_size..=y_size;

        vec![
            xs.clone()
                .map(|x| (x + coordinates.x, coordinates.y))
                .collect::<Vec<(i8, i8)>>(),
            ys.clone()
                .map(|y| (coordinates.x, y + coordinates.y))
                .collect::<Vec<(i8, i8)>>(),
            xs.clone().zip_eq(ys.clone()).collect::<Vec<(i8, i8)>>(),
            xs.clone()
                .zip_eq(ys.clone().rev())
                .collect::<Vec<(i8, i8)>>(),
        ]
        .iter()
        .map(|row| {
            row.iter()
                .map(|&(x, y)| Coordinates { x, y })
                .filter(|coordinates| self.on_board(coordinates))
                .collect::<Vec<Coordinates>>()
        })
        .filter(|row| row.contains(coordinates))
        .unique()
        .collect()
    }

    pub fn is_winning_move(&self, coordinates: &Coordinates, goal: i8) -> bool {
        self.affected_rows(coordinates)
            .iter()
            .flat_map(|row| {
                row.windows(goal as usize)
                    .map(|window| window.to_vec())
                    .filter(|window| window.contains(coordinates))
                    .collect::<Vec<Vec<Coordinates>>>()
            })
            .map(|row| {
                row.iter()
                    .map(|coordinates| self.hash.get(coordinates))
                    .collect::<Vec<Option<&Player>>>()
            })
            .any(|sequence| sequence.iter().all_equal() && !sequence.contains(&None))
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let cell_size = vec![self.min_x, self.max_x, self.min_y, self.max_y]
            .iter()
            .map(|s| s.to_string().len())
            .max()
            .unwrap()
            * 2
            + 3;
        let line_split = vec!["-".repeat(cell_size); (self.min_x..=self.max_x).len()];

        (self.min_y..=self.max_y)
            .map(|y| {
                (self.min_x..=self.max_x)
                    .map(move |x| {
                        let cell_value = self
                            .hash
                            .get(&Coordinates { x, y })
                            .map_or(format!("{},{}", x, y), |player| format!("{:?}", player));

                        format!("{: ^1$}", cell_value, cell_size)
                    })
                    .collect()
            })
            .intersperse(line_split)
            .map(|row| row.join("|"))
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tic_tac_toe() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);

        assert_eq!(board.hash.len(), 0);
        assert_eq!(board.min_x, game.min_x);
        assert_eq!(board.max_x, game.max_x);
        assert_eq!(board.min_y, game.min_y);
        assert_eq!(board.max_y, game.max_y);
    }

    #[test]
    fn test_new_gomoku() {
        let game = Game::GOMOKU;
        let board = Board::new(&game);

        assert_eq!(board.hash.len(), 0);
        assert_eq!(board.min_x, game.min_x);
        assert_eq!(board.max_x, game.max_x);
        assert_eq!(board.min_y, game.min_y);
        assert_eq!(board.max_y, game.max_y);
    }

    #[test]
    fn test_insert_smaller_min_x() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let coordinates = Coordinates {
            x: board.min_x - 1,
            y: 0,
        };
        let player = Player::X;
        let board = board.insert(&coordinates, &player);

        assert_eq!(board.is_err(), true);
    }

    #[test]
    fn test_insert_smaller_min_y() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let coordinates = Coordinates {
            x: 0,
            y: board.min_y - 1,
        };
        let player = Player::X;
        let board = board.insert(&coordinates, &player);

        assert_eq!(board.is_err(), true);
    }

    #[test]
    fn test_insert_bigger_max_x() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let coordinates = Coordinates {
            x: board.max_x + 1,
            y: 0,
        };
        let player = Player::X;
        let board = board.insert(&coordinates, &player);

        assert_eq!(board.is_err(), true);
    }

    #[test]
    fn test_insert_bigger_max_y() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let coordinates = Coordinates {
            x: 0,
            y: board.max_y + 1,
        };
        let player = Player::X;
        let board = board.insert(&coordinates, &player);

        assert_eq!(board.is_err(), true);
    }

    #[test]
    fn test_insert() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let coordinates = Coordinates { x: 0, y: 0 };
        let player = Player::X;
        let board = board.insert(&coordinates, &player);

        assert_eq!(board.unwrap().hash.get(&coordinates), Some(&player));
    }

    #[test]
    fn test_insert_already_taken() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let coordinates = Coordinates { x: 0, y: 0 };
        let player = Player::X;
        let board = board
            .insert(&coordinates, &player)
            .and_then(|board| board.insert(&coordinates, &player));

        assert_eq!(board.is_err(), true);
    }

    #[test]
    fn test_is_draw_empty() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);

        assert_eq!(board.is_draw(), false);
    }

    #[test]
    fn test_is_draw_minus_1() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game)
            .insert(&Coordinates { x: -1, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 1, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: -1, y: 0 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: 0 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 1, y: 0 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: -1, y: 1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: 1 }, &Player::X)
            .unwrap();

        assert_eq!(board.is_draw(), false);
    }

    #[test]
    fn test_is_draw_full() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game)
            .insert(&Coordinates { x: -1, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 1, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: -1, y: 0 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: 0 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 1, y: 0 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: -1, y: 1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: 1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 1, y: 1 }, &Player::X)
            .unwrap();

        assert_eq!(board.is_draw(), true);
    }

    #[test]
    fn test_affected_rows_tic_tac_toe_center() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let affected_rows = board.affected_rows(&Coordinates { x: 0, y: 0 });

        assert_eq!(affected_rows.len(), 4);
        assert!(affected_rows.contains(&vec![
            Coordinates { x: -1, y: 0 },
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 1, y: 0 }
        ]));
        assert!(affected_rows.contains(&vec![
            Coordinates { x: 0, y: -1 },
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 0, y: 1 }
        ]));
        assert!(affected_rows.contains(&vec![
            Coordinates { x: -1, y: -1 },
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 1, y: 1 }
        ]));
        assert!(affected_rows.contains(&vec![
            Coordinates { x: -1, y: 1 },
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 1, y: -1 }
        ]));
    }

    #[test]
    fn test_affected_rows_tic_tac_toe_corner() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let affected_rows = board.affected_rows(&Coordinates { x: -1, y: -1 });

        assert_eq!(affected_rows.len(), 3);
        assert!(affected_rows.contains(&vec![
            Coordinates { x: -1, y: -1 },
            Coordinates { x: 0, y: -1 },
            Coordinates { x: 1, y: -1 }
        ]));
        assert!(affected_rows.contains(&vec![
            Coordinates { x: -1, y: -1 },
            Coordinates { x: -1, y: 0 },
            Coordinates { x: -1, y: 1 }
        ]));
        assert!(affected_rows.contains(&vec![
            Coordinates { x: -1, y: -1 },
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 1, y: 1 }
        ]));
    }

    #[test]
    fn test_affected_rows_tic_tac_toe_middle() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let affected_rows = board.affected_rows(&Coordinates { x: -1, y: 0 });

        assert_eq!(affected_rows.len(), 2);
        assert!(affected_rows.contains(&vec![
            Coordinates { x: -1, y: -1 },
            Coordinates { x: -1, y: 0 },
            Coordinates { x: -1, y: 1 }
        ]));
        assert!(affected_rows.contains(&vec![
            Coordinates { x: -1, y: 0 },
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 1, y: 0 }
        ]));
    }

    #[test]
    fn test_is_winning_move_missing() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game)
            .insert(&Coordinates { x: -1, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: -1 }, &Player::X)
            .unwrap();

        assert_eq!(
            board.is_winning_move(&Coordinates { x: 1, y: -1 }, 3),
            false
        );
    }

    #[test]
    fn test_is_winning_move_blocked() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game)
            .insert(&Coordinates { x: -1, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 1, y: -1 }, &Player::O)
            .unwrap();

        assert_eq!(
            board.is_winning_move(&Coordinates { x: 1, y: -1 }, 3),
            false
        );
    }

    #[test]
    fn test_is_winning_move() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game)
            .insert(&Coordinates { x: -1, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 0, y: -1 }, &Player::X)
            .unwrap()
            .insert(&Coordinates { x: 1, y: -1 }, &Player::X)
            .unwrap();

        assert_eq!(board.is_winning_move(&Coordinates { x: 1, y: -1 }, 3), true);
    }

    #[test]
    fn test_to_string_tic_tac_toe() {
        let game = Game::TIC_TAC_TOE;
        let board = Board::new(&game);
        let expected = vec![
            " -1,-1 | 0,-1  | 1,-1  ",
            "-------|-------|-------",
            " -1,0  |  0,0  |  1,0  ",
            "-------|-------|-------",
            " -1,1  |  0,1  |  1,1  ",
        ]
        .join("\n");

        assert_eq!(board.to_string(), expected);
    }

    #[test]
    fn test_to_string_gomoku() {
        let game = Game::GOMOKU;
        let board = Board::new(&game);
        let expected = vec!(
            " -7,-7 | -6,-7 | -5,-7 | -4,-7 | -3,-7 | -2,-7 | -1,-7 | 0,-7  | 1,-7  | 2,-7  | 3,-7  | 4,-7  | 5,-7  | 6,-7  | 7,-7  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,-6 | -6,-6 | -5,-6 | -4,-6 | -3,-6 | -2,-6 | -1,-6 | 0,-6  | 1,-6  | 2,-6  | 3,-6  | 4,-6  | 5,-6  | 6,-6  | 7,-6  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,-5 | -6,-5 | -5,-5 | -4,-5 | -3,-5 | -2,-5 | -1,-5 | 0,-5  | 1,-5  | 2,-5  | 3,-5  | 4,-5  | 5,-5  | 6,-5  | 7,-5  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,-4 | -6,-4 | -5,-4 | -4,-4 | -3,-4 | -2,-4 | -1,-4 | 0,-4  | 1,-4  | 2,-4  | 3,-4  | 4,-4  | 5,-4  | 6,-4  | 7,-4  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,-3 | -6,-3 | -5,-3 | -4,-3 | -3,-3 | -2,-3 | -1,-3 | 0,-3  | 1,-3  | 2,-3  | 3,-3  | 4,-3  | 5,-3  | 6,-3  | 7,-3  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,-2 | -6,-2 | -5,-2 | -4,-2 | -3,-2 | -2,-2 | -1,-2 | 0,-2  | 1,-2  | 2,-2  | 3,-2  | 4,-2  | 5,-2  | 6,-2  | 7,-2  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,-1 | -6,-1 | -5,-1 | -4,-1 | -3,-1 | -2,-1 | -1,-1 | 0,-1  | 1,-1  | 2,-1  | 3,-1  | 4,-1  | 5,-1  | 6,-1  | 7,-1  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,0  | -6,0  | -5,0  | -4,0  | -3,0  | -2,0  | -1,0  |  0,0  |  1,0  |  2,0  |  3,0  |  4,0  |  5,0  |  6,0  |  7,0  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,1  | -6,1  | -5,1  | -4,1  | -3,1  | -2,1  | -1,1  |  0,1  |  1,1  |  2,1  |  3,1  |  4,1  |  5,1  |  6,1  |  7,1  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,2  | -6,2  | -5,2  | -4,2  | -3,2  | -2,2  | -1,2  |  0,2  |  1,2  |  2,2  |  3,2  |  4,2  |  5,2  |  6,2  |  7,2  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,3  | -6,3  | -5,3  | -4,3  | -3,3  | -2,3  | -1,3  |  0,3  |  1,3  |  2,3  |  3,3  |  4,3  |  5,3  |  6,3  |  7,3  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,4  | -6,4  | -5,4  | -4,4  | -3,4  | -2,4  | -1,4  |  0,4  |  1,4  |  2,4  |  3,4  |  4,4  |  5,4  |  6,4  |  7,4  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,5  | -6,5  | -5,5  | -4,5  | -3,5  | -2,5  | -1,5  |  0,5  |  1,5  |  2,5  |  3,5  |  4,5  |  5,5  |  6,5  |  7,5  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,6  | -6,6  | -5,6  | -4,6  | -3,6  | -2,6  | -1,6  |  0,6  |  1,6  |  2,6  |  3,6  |  4,6  |  5,6  |  6,6  |  7,6  ",
            "-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------",
            " -7,7  | -6,7  | -5,7  | -4,7  | -3,7  | -2,7  | -1,7  |  0,7  |  1,7  |  2,7  |  3,7  |  4,7  |  5,7  |  6,7  |  7,7  ",
        ).join("\n");

        assert_eq!(board.to_string(), expected);
    }
}
