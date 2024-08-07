
//use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt;

pub type CardId = usize;

/// Info about the player.
/// 
#[derive(Clone)]
pub struct Player {
    pub name : String,
    pub collected_cards : Vec<CardId>,
}

impl Player {
    pub fn new(name : String) -> Self {
        Player {
            name,
            collected_cards : Vec::new(),
        }
    }
}

/// Ifo about a card.
/// Each card has a unique id which is a number from 0 to length of card deck. 
/// The card_type should exist as a pair on the card deck. Same with the title.
/// There should be pairs.
/// 
#[derive(Debug)]
pub struct Card {
    pub id : CardId,
    pub card_type : usize,
    pub title : String,
}

/// Represents a deck of cards
pub type Deck = Vec<Card>;

/// Represents a random shuffle of the card IDs of the deck
pub type Shuffle = Vec<CardId>;

/// This creates a sorted deck of cards with the given number of pairs.
pub fn create_deck(num_pairs : usize) -> Deck {
    let mut deck = Vec::with_capacity(num_pairs*2);

    for i in 0..num_pairs {
        deck.push( Card {
            id : 2*i,
            card_type : i,
            title : i.to_string(),
        });
        deck.push(Card {
            id : 2*i+1,
            card_type : i,
            title : i.to_string(),
        });
    }
    deck
}

/// This creates a shuffle of the card deck (i.e. a vector witht he ids of the card deck in random order)
pub fn shuffle_deck(deck : &Deck) -> Shuffle {
    let mut deck_shuffle : Vec<usize> = (0..deck.len()).collect();
    deck_shuffle.shuffle(&mut thread_rng());
    deck_shuffle
}

/// This is the information stored in a field. It is an Option of CardId.
pub type FieldSlot = Option<CardId>;
/// This is a row in the field
pub type FieldRow  = Vec<FieldSlot>;

/// The field represents a deck of cards layed out in a rectangular shape. The cards are layed out according
/// to the shuffle. Note that each position in the field holds the index of a card in the deck. We don't use
/// pointers because this requires either smart pointers or very complicated timeline annotations.
/// Not sure if this is the best way to do it (the relationship between card and field is not obvious anymore),
/// but it allows to avoid smart pointers or timelines.
pub struct Field {
    pub height : usize,
    pub width : usize,
    pub field : Vec<FieldRow>,
}

impl Field {
    pub fn new(height : usize, width : usize) -> Self {
        let mut field : Vec<FieldRow> = Vec::with_capacity(height);
        for _ in 0..height {
            field.push(vec![None; width]);
        }

        Field {
            height,
            width,
            field
        }
    }
    pub fn card_idx(& self, y : usize, x : usize) -> Option<usize> {
        if y >= self.height || x >= self.width {
            panic!("illegal coordinates ({}, {})", y, x);
        } 
        self.field[y][x]
    }
    pub fn clear_field(&mut self, ) {
        for row in 0..self.height {
            for col in 0..self.width {
               self.field[row][col] = None;
            }
        }
    }
    pub fn place_deck(&mut self, deck : & Deck) {
        let total_fields= self.width * self.height;
        let total_cards = deck.len();
        if total_fields != total_cards {
            panic!("Card number does not fit to field size");
        }
        let shuffle = shuffle_deck(deck);
        let mut row : usize = 0;
        let mut col : usize = 0;
        for card_id in shuffle {
            self.field[row][col] = Some(card_id);
            col += 1;
            if col >= self.width {
                col = 0;
                row += 1;
            }
        }
    }
}

/// Holds the core data of the game.
/// 
/// This is the expected sequence of function calls to make the Game iterate through it's states:
/// 
/// new() --> state = GameOver
/// add_player()
/// add_player()
/// ...
/// reset() --> state = StartGame
/// open_card() --> state = FirstCard
/// open_card() --> state = SecondCard
/// check_guess_current_player() --> state = ViewResult
/// check_gameover() --> state = NextUser
/// next_player() --> state = StartSelectCaards
/// open_card() -- state = FirstCard
/// ...
/// check_gameover() -- state = GameOver
/// reset() --> state = StartGame
pub struct Game {
    pub state : GameState,
    pub last_guess_success : bool,
    pub rounds_counter : u32,
    pub field : Field,
    pub players : Vec<Player>,
    pub deck : Deck,
    pub current_player_id : usize,
    pub num_clicked : usize,
    pub clicked_card1 : Coord,
    pub clicked_card2 : Coord,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Coord (pub usize, pub usize);


impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(PartialEq, Clone)]
pub enum GameState {
    GameSetup,
    StartGame,
    StartSelectCards,
    FirstCard,
    SecondCard,
    // ViewResult,
    NextUser,
    GameOver,
}

impl Game {
    pub fn new(height : usize, width : usize) -> Self {
        Self {
            state : GameState::GameOver,
            last_guess_success : false,
            rounds_counter : 0,
            field : Field::new(height, width),
            players : Vec::new(),
            deck : create_deck((height * width) / 2),
            current_player_id : 0,
            num_clicked : 0,
            clicked_card1 : Coord(0, 0),
            clicked_card2 : Coord(0, 0),
        }
    }

    pub fn game_state(& self) -> GameState {
        self.state.clone()
    }

    pub fn last_guess_success(& self) -> bool {
        self.last_guess_success
    }

    pub fn round(& self) -> u32 {
        self.rounds_counter
    }

    pub fn add_player(&mut self, name : String) {
        let p = Player::new(name);
        self.players.push(p);
    }

    pub fn current_player(& self) -> Player {
        self.players[self.current_player_id].clone()
    }

    pub fn reset(&mut self) {
        self.rounds_counter = 1;
        self.state = GameState::StartGame;
        self.field.clear_field();
        self.field.place_deck(&self.deck);
        for player in &mut self.players {
            player.collected_cards.clear();
        }
        self.current_player_id = 0;
    }

    pub fn card_at(&self, y : usize, x : usize) -> Option<&Card> {
        match self.field.card_idx(y, x) {
            Some(card_idx) => { Some(&self.deck[card_idx]) },
            None => { None },
        }
    }

    pub fn is_clicked(&self, coord : & Coord) -> bool {
        if self.num_clicked == 0 {
            return false;
        }
        if *coord == self.clicked_card1 {
            return true;
        }
        if self.num_clicked > 1 && self.clicked_card2 == *coord {
            return true;
        }
        false
    }

    pub fn coord_has_card(&self, coord : &Coord) -> bool {
        let result = match self.card_at(coord.0, coord.1) {
            Some(_) => { true },
            None => { false },
        };
        result
    }

    pub fn close_selected_cards(&mut self) {
        self.num_clicked = 0;
    }

    pub fn open_card(&mut self, coord : &Coord) -> bool { 
        if ! self.coord_has_card(coord) {
            return false;
        }
        if self.num_clicked == 0 {
            self.clicked_card1 = coord.clone();
            self.num_clicked = 1;
            self.state = GameState::FirstCard;
            return true;
        } else if self.num_clicked == 1 {
            if *coord != self.clicked_card1 {
                self.clicked_card2 = coord.clone();
                self.num_clicked = 2;
                self.state = GameState::SecondCard;
                return true;
            }
        }
        false
    }

    pub fn check_guess_current_player(&mut self) -> bool {
        // copying the parameters is just borrow checker bs
        // could probably be avoided with some lifetime crap ...
        let p = self.current_player_id;
        let c1 = self.clicked_card1.clone();
        let c2 = self.clicked_card2.clone();
        self.check_guess(p, &c1, &c2)
    }

    pub fn check_guess(& mut self, player : usize, coord1 : &Coord, coord2 : &Coord) -> bool {
        if self.field.field[coord1.0][coord1.1] == None {
            println!("coord1 card already taken");
            return false;
        }
        if self.field.field[coord2.0][coord2.1] == None {
            println!("coord2 card already taken");
            return false;
        }

        // self.state = GameState::ViewResult;
        self.state = GameState::NextUser;
        // self.state = GameState::StartSelectCards;

        let card_id1 = self.field.field[coord1.0][coord1.1].unwrap();
        let card_id2 = self.field.field[coord2.0][coord2.1].unwrap();
        let col_cards = &mut self.players[player].collected_cards;

        if self.deck[card_id1].card_type == self.deck[card_id2].card_type {
            col_cards.push(card_id1);
            col_cards.push(card_id2);

            self.field.field[coord1.0][coord1.1] = None;
            self.field.field[coord2.0][coord2.1] = None;
            self.last_guess_success = true;
            return true;
        }
        self.last_guess_success = false;
        false
    }

    pub fn check_game_over(&mut self) -> bool {
        if self.state == GameState::GameOver {
            return true;
        }
        for row in 0..self.field.height {
            for col in 0..self.field.width {
                if self.field.field[row][col] != None {
                    self.state = GameState::NextUser;
                    // self.state = GameState::StartSelectCards;
                    return false;
                }
            }
        }
        self.state = GameState::GameOver;
        true
    }

    pub fn next_player(&mut self) {
        self.state = GameState::StartSelectCards;
        self.current_player_id += 1;
        if self.current_player_id >= self.players.len() {
            self.current_player_id = 0;
            self.rounds_counter += 1;
        }
    }

    pub fn print_card_at(& self, coord : & Coord) {
        if self.field.field[coord.0][coord.1] == None {
            println!("<<no card>>");
        } else {
            let card_id = self.field.field[coord.0][coord.1].unwrap();
            println!("{:#?}", self.deck[card_id]);
        }
    }

    pub fn print_field(& self) {
        for row in 0..self.field.height {
            for col in 0..self.field.width {
                if self.field.field[row][col] == None {
                    print!(" xx ");
                } else {
                    let card_id = self.field.field[row][col].unwrap();
                    let card_type = self.deck[card_id].card_type;
                    print!(" {:0>2} ", card_type);
                }
            }
            println!("");
        }
    }

    pub fn print_cards_of_current_player(& self) {
        let p = self.current_player_id;
        self.print_cards_of_player(p);
    }

    pub fn print_cards_of_player(& self, player_id : usize) {
        println!("Round {} Player {} has {} cards:", 
            self.rounds_counter , self.players[player_id].name, self.players[player_id].collected_cards.len());
        let col_cards = &self.players[player_id].collected_cards;
        for card_id in col_cards {
            let card = &self.deck[*card_id];
            print!("(id: {} type: {} str: {}), ", card.id, card.card_type, card.title);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::{create_deck, shuffle_deck};
    #[test]
    fn test_create_deck() {
        let deck = create_deck(32);
        assert_eq!(deck.len(), 64);
        for i in 0..32  {
            assert_eq!(deck[2*i].id, 2*i);
            assert_eq!(deck[2*i].card_type, i);
            assert_eq!(deck[2*i].title, i.to_string());
            assert_eq!(deck[2*i+1].id, 2*i+1);
            assert_eq!(deck[2*i+1].card_type, i);
            assert_eq!(deck[2*i+1].title, i.to_string());
        }
    }
    #[test]
    fn test_shuffle_deck() {
        let deck = create_deck(32);
        let shuffle = shuffle_deck(&deck);
        assert_eq!(shuffle.len(), 64);
        // we should now test that all numbers between 0 and 31 are in it twice, maybe also check randomness in some way
        // but too much work
    }
}
