use crate::play::{Play, PlayCard};

pub struct Player {
    pub num: u8,
    pub name: String,
    pub team_name: String,
    pub score: u32,
    pub play_cards: Vec<PlayCard>,
    pub current_play: Option<Play>,
}

impl Player {
    pub fn new(num: u8, name: String, team_name: String, ot: bool) -> Self {
        Self {
            num: if num == 1 { 1 } else { 2 },
            name,
            team_name,
            play_cards: init_play_cards(ot),
            ..Default::default()
        }
    }

    pub fn get_rand_card(&mut self) -> Play {
        loop {
            let temp_play = Play::get_rand_play();
            if self.is_valid_play(temp_play) {
                return temp_play;
            }
        }
    }

    // Attempt to draw a play card - this is NOT for Hail Mary
    fn is_valid_play(&mut self, play: Play) -> bool {
        if let Some(card) = self
            .play_cards
            .iter_mut()
            .find(|card| card.play == play && card.count > 0)
        {
            card.count -= 1;

            // Check to see if play cards are empty
            if self.play_cards.iter().take(5).all(|card| card.count == 0) {
                self.init_play_cards_not_hm();
            }

            true
        } else {
            false
        }
    }

    fn init_play_cards_not_hm(&mut self) {
        println!("Dealing play cards - not touching HMs, though");
        self.play_cards[0] = PlayCard::new(Play::ShortRun, false);
        self.play_cards[1] = PlayCard::new(Play::LongRun, false);
        self.play_cards[2] = PlayCard::new(Play::ShortPass, false);
        self.play_cards[3] = PlayCard::new(Play::LongPass, false);
        self.play_cards[4] = PlayCard::new(Play::TrickPlay, false);
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            num: 1,
            name: String::from("Player"),
            team_name: String::from("Teams"),
            score: 0,
            play_cards: init_play_cards(false),
            current_play: None,
        }
    }
}

fn init_play_cards(ot: bool) -> Vec<PlayCard> {
    println!("Dealing play cards - including HMs");
    vec![
        PlayCard::new(Play::ShortRun, ot),
        PlayCard::new(Play::LongRun, ot),
        PlayCard::new(Play::ShortPass, ot),
        PlayCard::new(Play::LongPass, ot),
        PlayCard::new(Play::TrickPlay, ot),
        PlayCard::new(Play::HailMary, ot),
    ]
}
