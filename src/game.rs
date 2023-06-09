use crate::{play::Play, player::Player};
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

enum Score {
    Touchdown,
    Safety,
    _FieldGoal,
    _ExtraPoint,
    _TwoPoint,
}

pub struct Game {
    pub players: [Player; 2],
    pub game_over: bool,
    pub spot: u8,
    pub first_down: u8,
    mult_cards: Vec<u8>,
    yard_cards: Vec<u8>,
    pub off_num: u8,
    pub down: u8,
    recent_score: bool,
}

impl Game {
    pub fn new(p1: Player, p2: Player) -> Self {
        Self {
            players: [p1, p2],
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

    pub fn change_poss(&mut self) {
        if self.off_num == 1 {
            self.off_num = 2;
        } else {
            self.off_num = 1;
        }

        // Reset spot
        self.spot = 25;
        self.first_down = 35;
    }

    pub fn calc_dist(&mut self, p1: Play, p2: Play) {
        let mut score: Option<Score> = None;
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
        let mut distance = (yard_card as f32 * times).ceil() as i8;

        // Check for touchdown
        if distance + (self.spot as i8) > 100 {
            distance = 100 - (self.spot as i8);
            score = Some(Score::Touchdown); //self.score_touchdown(); // Maybe this part later?
                                            // Check for safety
        } else if distance + (self.spot as i8) < 0 {
            distance = -(self.spot as i8);
            score = Some(Score::Safety); //self.score_safety();
        }

        println!(
            "This play {}",
            match distance {
                d if d > 0 => format!("gained {} yards!", d),
                d if d < 0 => format!("lost {} yards!", -d),
                _ => "resulted in no gain.".to_string(),
            }
        );

        self.spot = (self.spot as i8 + distance) as u8;

        if score.is_some() {
            match score {
                Some(Score::Touchdown) => self.score_touchdown(),
                Some(Score::Safety) => self.score_safety(),
                _ => unimplemented!(
                    "How'd you get here? You scored something you shouldn't be able to!"
                ),
            }
        }
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

    pub fn score_touchdown(&mut self) {
        self.players[Game::get_index(self.off_num)].score += 6;
        println!(
            "{} scored a touchdown!",
            self.players[Game::get_index(self.off_num)].name
        );
        self.change_poss();
        self.recent_score = true;
    }

    pub fn score_safety(&mut self) {
        self.players[self.get_def_index()].score += 2;
        println!(
            "{} forced a safety!",
            self.players[self.get_def_index()].name
        );
        self.change_poss();
        self.recent_score = true;
    }

    fn get_index(num: u8) -> usize {
        (num - 1) as usize
    }

    fn get_def_index(&self) -> usize {
        let def = match self.off_num {
            1 => 2,
            _ => 1,
        };
        Game::get_index(def)
    }

    fn coin_flip() -> u8 {
        rand::thread_rng().gen_range(0..1)
    }

    pub fn end_play(&mut self) {
        let mut coin: u8 = 0;

        // Avoid reporting and down changing for recent scores
        if !self.recent_score {
            // Sticks
            if self.spot == self.first_down {
                println!("Sticks...");
                coin = Game::coin_flip();

                if coin == 0 {
                    println!("Almost!");
                }
            }

            if self.spot > self.first_down || coin == 1 {
                println!("First down!");
                self.down = 1;

                if self.spot > 90 {
                    self.first_down = 100;
                } else {
                    self.first_down = self.spot + 10;
                }

                coin = 1;
            }

            if coin == 0 {
                // && game.change_time !== Time::PEN_DOWN)
                self.down += 1
            }

            if self.down > 4 {
                // && !recentScore)
                println!("Turnover on downs!!!");
                self.change_poss();
                self.down = 1
            }

            self.print_down();
        } else {
            self.recent_score = false;
        }
    }

    fn print_down(&self) {
        println!(
            "{}{} & {}",
            self.down,
            Game::ending(self.down),
            self.down_dist()
        );
    }

    fn ending(down: u8) -> String {
        match down {
            1 => "st".to_string(),
            2 => "nd".to_string(),
            3 => "rd".to_string(),
            _ => "th".to_string(),
        }
    }

    fn down_dist(&self) -> String {
        let difference = self.first_down - self.spot;

        match difference {
            100 => "G".to_string(),
            _ if difference == 0 => "IN".to_string(),
            _ => format!("{}", difference),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            players: [Player::default(), Player::default()],
            game_over: false,
            spot: 25,
            first_down: 35,
            mult_cards: vec![3, 4, 4, 4],
            yard_cards: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            off_num: 1,
            down: 1,
            recent_score: false,
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
