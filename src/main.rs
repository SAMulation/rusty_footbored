use fbg_rust::{game::Game, player::Player};

fn main() {
    let ot = false;
    let player1 = Player::new(1, "Roger".to_string(), "Beavs".to_string(), ot);
    let player2 = Player::new(2, "Sam".to_string(), "Rams".to_string(), ot);
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

    let mut game = Game::new(player1, player2);

    for i in 0..100 {
        let p1_card = game.players[0].get_rand_card();
        let p2_card = game.players[1].get_rand_card();

        println!("Play {}:", i + 1);
        println!("Player 1 card: {}", p1_card);
        println!("Player 2 card: {}", p2_card);
        println!("Off num is: {}", game.off_num);

        game.calc_dist(p1_card, p2_card);
        game.end_play();

        println!(
            "{}: {} | {}: {}",
            game.players[0].team_name,
            game.players[0].score,
            game.players[1].team_name,
            game.players[1].score
        );
        println!(
            "{} ball on the {} yard line",
            game.players[game.off_num as usize - 1].team_name,
            if game.spot > 50 {
                100 - game.spot
            } else {
                if game.spot == 50 {
                    50
                } else {
                    game.spot
                }
            }
        );
        println!("----------------");
    }
}
