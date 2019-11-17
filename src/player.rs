#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn first() -> Player {
        Player::X
    }

    pub fn next(&self) -> Player {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(Player::X.next().next(), Player::X);
        assert_eq!(Player::O.next().next(), Player::O);
    }
}
