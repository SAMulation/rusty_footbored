use fbg_rust::{game::Game, player::Player};

fn main() {
    let ot = false;
    let mut player1 = Player::default();
    let mut player2 = Player::new(2, "Sam".to_string(), "Rams".to_string(), ot);
    println!("Welcome to FootBored: Rust Edition!!");
    println!(
        "Player {} is {} and their team is the {}!",
        player1.num, player1.name, player1.team_name
    );
    println!(
        "Player {} is {} and their team is the {}!",
        player2.num, player2.name, player2.team_name
    );

    println!("Let's check out all the play types!");

    for play in &player2.play_cards {
        println!("Play: {}, Count: {}", play.play, play.count);
    }

    let mut game = Game::new();

    for i in 0..20 {
        let p1_card = player1.get_rand_card();
        let p2_card = player2.get_rand_card();

        println!("Play {}:", i + 1);
        println!("Player 1 card: {}", p1_card);
        println!("Player 2 card: {}", p2_card);
        game.calc_dist(p1_card, p2_card);
        println!("----------------");
    }
}
