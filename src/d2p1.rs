use crate::util::read_input;

const RED_COUNT: usize = 12;
const GREEN_COUNT: usize = 13;
const BLUE_COUNT: usize = 14;

#[derive(Debug, Default)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl Hand {
    pub fn read(line: &str) -> Option<Self> {
        let colors = line.split(",");
        let mut hand = Hand::default();
        for color in colors {
            let mut sp = color.trim().split(" ");
            let count = sp.next()?.parse::<usize>().expect("invalid color count");
            match sp.next()? {
                "red" => hand.red = count,
                "green" => hand.green = count,
                "blue" => hand.blue = count,
                other => {
                    println!("Can't read color {}", other);
                    return None;
                }
            }
        }
        Some(hand)
    }

    pub fn possible(&self) -> bool {
        return self.red <= RED_COUNT && self.green <= GREEN_COUNT && self.blue <= BLUE_COUNT;
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    hands: Vec<Hand>,
}

impl Game {
    pub fn read(mut line: &str) -> Option<Self> {
        line = line.strip_prefix("Game")?;
        let id = line
            .chars()
            .skip(1)
            .take_while(|it| it.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .expect("invalid game id");
        line = line.split(":").skip(1).next()?;

        let mut hands = Vec::with_capacity(4);
        for h in line.split(";") {
            hands.push(Hand::read(h)?);
        }

        Some(Game { id, hands })
    }

    pub fn possible(&self) -> bool {
        self.hands.iter().all(Hand::possible)
    }
}

pub fn main() {
    let input = read_input(2);
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::read(line).expect("can't read game");

        if game.possible() {
            sum += game.id;
        }
    }
    println!("Total: {}", sum);
}
