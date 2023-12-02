use std::str::FromStr;
use std::cmp::max;
use anyhow::anyhow;

static RED_CUBES: usize = 12;
static GREEN_CUBES: usize = 13;
static BLUE_CUBES: usize = 14;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(anyhow!("Invalid color"))
        }
    }
}

#[derive(Debug)]
struct Draw(usize, Color);

impl FromStr for Draw {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        Ok(Draw(parts.next().unwrap().parse()?, parts.next().unwrap().parse()?))
    }
}

type Set = Vec<Draw>;

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let id: usize = parts.next().unwrap().split(' ').last().unwrap().parse().unwrap();
        let sets: Vec<Set> = parts.next().unwrap().split(';').map(|s| {
            s.split(", ").map(|s2| Ok(s2.trim().parse::<Draw>()?)).collect::<Result<Vec<_>, anyhow::Error>>()
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(Game {
            id,
            sets
        })
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        for set in &self.sets {
            for draw in set {
                let valid = match draw.1 {
                    Color::Red => draw.0 <= RED_CUBES,
                    Color::Blue => draw.0 <= BLUE_CUBES,
                    Color::Green => draw.0 <= GREEN_CUBES,
                };
                if !valid {
                    return false;
                }
            }
        }
        return true;
    }
    fn power(&self) -> usize {
        let mut mins = (0, 0, 0);

        for set in &self.sets {
            for draw in set {
                match draw.1 {
                    Color::Red => mins.0 = max(mins.0, draw.0),
                    Color::Green => mins.1 = max(mins.1, draw.0),
                    Color::Blue => mins.2 = max(mins.2, draw.0),
                }
            }
        }
        return mins.0 * mins.1 * mins.2;
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc2023::read_lines("input/input2.txt").unwrap();
    let games: Vec<Game> = lines.map(|s| Ok(s?.parse()?)).collect::<Result<_, anyhow::Error>>()?;

    let count: usize = games.iter().filter(|g| g.is_possible()).map(|g| g.id).sum();
    println!("Indices sum: {count}");

    let powers: usize = games.iter().map(|g| g.power()).sum();
    println!("Powers sum: {powers}");

    Ok(())
}
