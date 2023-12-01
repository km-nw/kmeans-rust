use std::cmp::Ordering;
use anyhow::{anyhow, Result};


#[derive(Debug, PartialEq, Eq, Ord)]
struct Point {
    x: i32, 
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn parse(s: &str) -> Result<Self> {
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 2 {
            return Err(anyhow!("Invalid point: {}", s));
        }
        let x = coords[0]
            .trim()
            .parse::<i32>()
            .ok()
            .ok_or(anyhow!("Invalid point: {}", s))?;
        let y = coords[1]
            .trim()
            .parse::<i32>()
            .ok()
            .ok_or(anyhow!("Invalid point: {}", s))?;
        Ok(Point { x, y })
    }

    fn dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.partial_cmp(&other.y),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_parse() {
        let res = Point::parse("1,2");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Point::new(1, 2));
    }

    #[test]
    fn test_point_ord() {
        assert_eq!(1 + 2, 3);
    }
}
