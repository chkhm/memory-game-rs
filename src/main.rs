
use std::io;

use memory_game::{Game, Coord};

fn main() {
    println!("Hello, world!");
    let height = 8;
    let width = 8;
    let mut game = Game::new(height, width);

    println!("Game width {} and height {}", game.field.width, game.field.height);
    let player_hugo = "Hugo".to_string();
    game.add_player(player_hugo);
    println!("Added Player: {}", game.players[0].name);
    println!("Deck has {} cards.", game.deck.len());
    println!("First card of deck is: {:#?}", & game.deck[0]);
    println!("Setting cards...");
    game.reset();
    println!("First Card is: {:#?}", & game.deck[0]);
    let card_id = game.field.field[0][0].unwrap();
    println!("Card on field[0][0] is: {}", card_id);
    println!("Card on {} is  {:#?}", card_id, game.deck[card_id]);

    while ! game.check_game_over() {
        let mut coord1_str = String::new();
        let mut coord2_str = String::new();

        game.print_field();

        println!("\nMake a guess, for coord 1 like so \"3, 5\":");
        io::stdin()
            .read_line(&mut coord1_str)
            .expect("Failed to read");

        println!("\nMake a guess, for coord 2 like so \"3, 5\":");
        io::stdin()
            .read_line(&mut coord2_str)
            .expect("Failed to read");

        let splitted: Vec<&str>= coord1_str.split(",").collect();
        let y1 :usize = splitted[0].trim().parse().unwrap();
        let x1 :usize = splitted[1].trim().parse().unwrap();

        let splitted: Vec<&str>= coord2_str.split(",").collect();
        let y2 :usize = splitted[0].trim().parse().unwrap();
        let x2 :usize = splitted[1].trim().parse().unwrap();

        println!("\nPlayer 0 trying his luck at [{}, {}] and [{}, {}]", y1, x1, y2, x2);
        let player_id = 0;
        let coord1 = Coord(y1, x1);
        let coord2 = Coord(y2, x2);
        let found_pair = game.check_guess(player_id, &coord1, &coord2);
        if found_pair {
            println!("You are in luck!");
            game.print_cards_of_player(player_id);
        } else {
            println!("Sorry, that did not work out");
        }
        println!("\nlocation[{}, {}]: ", y1, x1);
        game.print_card_at(&coord1);
        println!("location[{}, {}]: ", y2, x2);
        game.print_card_at(&coord2);
    }
}
