use regex::Regex;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Coordinates {
    pub x: i8,
    pub y: i8,
}

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::new(r"^(-?[0-9]+),(-?[0-9]+)$")
            .unwrap()
            .captures(s)
            .and_then(|cap| {
                let x = cap.get(1).and_then(|m| m.as_str().parse().ok());
                let y = cap.get(2).and_then(|m| m.as_str().parse().ok());

                match (x, y) {
                    (Some(x), Some(y)) => Some((x, y)),
                    _ => None,
                }
            })
            .map(|(x, y)| Coordinates { x, y })
            .ok_or_else(|| "Coordinates can't be parsed".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            Coordinates::from_str("-1,-1").unwrap(),
            Coordinates { x: -1, y: -1 }
        );
        assert_eq!(
            Coordinates::from_str("-1,0").unwrap(),
            Coordinates { x: -1, y: 0 }
        );
        assert_eq!(
            Coordinates::from_str("-1,1").unwrap(),
            Coordinates { x: -1, y: 1 }
        );
    }
}
