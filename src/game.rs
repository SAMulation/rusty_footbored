use crate::play::Play;
use rand::Rng;
use std::{fmt::Display, vec};

const MATCHUP: [[u8; 4]; 4] = [[5, 3, 3, 2], [2, 4, 1, 2], [3, 2, 5, 3], [1, 2, 2, 4]];

const MULTI: [[f32; 4]; 5] = [
    [0.0, 2.0, 3.0, 4.0],
    [0.0, 1.0, 2.0, 3.0],
    [0.0, 0.5, 1.0, 2.0],
    [-1.0, 0.0, 1.0, 1.5],
    [-1.0, 0.0, 0.5, 1.0],
];

pub struct Game {
    pub game_over: bool,
    pub spot: u8,
    pub first_down: u8,
    mult_cards: Vec<u8>,
    yard_cards: Vec<u8>,
    pub off_num: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    // Loop through yard cards until valid card found, decrement, return +1 of it
    // Check if the deck is now empty
    pub fn draw_yard_card(&mut self) -> u8 {
        let mut card: u8 = 0;
        let mut index: usize = 11;

        while card == 0 {
            index = rand::thread_rng().gen_range(0..self.yard_cards.len());
            card = self.yard_cards[index];

            if card > 0 {
                self.yard_cards[index] -= 1;
            }
        }

        if self.yard_cards.iter().all(|&count| count == 0) {
            self.init_yard_cards();
        }

        index as u8 + 1
    }

    fn init_yard_cards(&mut self) {
        println!("Dealing yard cards");
        self.yard_cards = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    }

    // Loop through multiplier cards until valid card found, decrement, return MultCard
    // Check if the deck is now empty
    pub fn draw_mult_card(&mut self) -> MultCard {
        let mut card: u8 = 0;
        let mut index: usize = 4;

        while card == 0 {
            index = rand::thread_rng().gen_range(0..self.mult_cards.len());
            card = self.mult_cards[index];

            if card > 0 {
                self.mult_cards[index] -= 1;
            }
        }

        if self.mult_cards.iter().all(|&count| count == 0) {
            self.init_mult_cards();
        }

        Self::get_mult_card(index)
    }

    fn get_mult_card(index: usize) -> MultCard {
        match index {
            0 => MultCard::Ten,
            1 => MultCard::Jack,
            2 => MultCard::Queen,
            3 => MultCard::King,
            _ => unreachable!("Invalid MultCard: {}! How'd you get here?", index),
        }
    }

    fn init_mult_cards(&mut self) {
        println!("Dealing multiplier cards");
        self.mult_cards = vec![3, 4, 4, 4];
    }

    pub fn calc_dist(&mut self, p1: Play, p2: Play) {
        let mult_card = self.draw_mult_card();
        println!("Multipler Card: {}", mult_card);

        let times = self.calc_times(p1 as u8, p2 as u8, mult_card as u8);

        let yard_card = if times != 0.0 {
            self.draw_yard_card()
        } else {
            0
        };

        println!("Yard Card: {}", yard_card);
        println!("Multiplier: {}", times);
        let distance = (yard_card as f32 * times).ceil() as i8;

        // Check for touchdowns
        if self.spot as i8 + distance >= 100 {
            println!("This would be a touchdown!");
        }

        // Check for safeties
        if self.spot as i8 + distance <= 0 {
            println!("This would be a safety!");
        }

        println!(
            "This play {}",
            match distance {
                d if d > 0 => format!("gained {} yards!", d),
                d if d < 0 => format!("lost {} yards!", -d),
                _ => "resulted in no gain.".to_string(),
            }
        );

        // self.spot = (self.spot as i8 + distance) as u8;
    }

    fn calc_times(&self, p1: u8, p2: u8, mult_idx: u8) -> f32 {
        let mut _match_value = 0;

        if p1 == 4 || p2 == 4 {
            _match_value = 1;
            // game.this_play.quality = Some('/');
        } else {
            _match_value = MATCHUP[if self.off_num == 1 {
                p1 as usize
            } else {
                p2 as usize
            }][if self.off_num == 1 {
                p2 as usize
            } else {
                p1 as usize
            }];
        }

        // Handle Same Play here

        MULTI[_match_value as usize - 1][mult_idx as usize]
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            game_over: false,
            spot: 65,
            first_down: 75,
            mult_cards: vec![3, 4, 4, 4],
            yard_cards: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            off_num: 1,
        }
    }
}

pub enum MultCard {
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for MultCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_text = match *self {
            MultCard::Ten => "10",
            MultCard::Jack => "Jack",
            MultCard::Queen => "Queen",
            MultCard::King => "King",
        };

        write!(f, "{}", display_text)
    }
}
