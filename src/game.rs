pub struct Game {
    pub min_x: i8,
    pub max_x: i8,
    pub min_y: i8,
    pub max_y: i8,

    pub goal: i8,
}

impl Game {
    pub const TIC_TAC_TOE: Game = Game {
        min_x: -1,
        max_x: 1,
        min_y: -1,
        max_y: 1,
        goal: 3,
    };

    pub const GOMOKU: Game = Game {
        min_x: -7,
        max_x: 7,
        min_y: -7,
        max_y: 7,
        goal: 5,
    };
}
