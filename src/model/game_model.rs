
//use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt;

pub type CardId = usize;

pub struct Player {
    pub name : String,
    pub collected_cards : Vec<CardId>,
}

#[derive(Debug)]
pub struct Card {
    pub id : CardId,
    pub card_type : usize,
    pub title : String,
}

pub type Deck = Vec<Card>;
pub type Shuffle = Vec<CardId>;

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

pub fn shuffle_deck(deck : &Deck) -> Shuffle {
    let mut deck_shuffle : Vec<usize> = (0..deck.len()).collect();
    deck_shuffle.shuffle(&mut thread_rng());
    deck_shuffle
}

pub type FieldSlot = Option<CardId>;
pub type FieldRow  = Vec<FieldSlot>;

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

pub struct Game {
    pub field : Field,
    pub players : Vec<Player>,
    pub deck : Deck,
    pub current_player_id : usize,
    pub clicked_card : Option<Coord>,
}

#[derive(Clone)]
pub struct Coord (pub usize, pub usize);


impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Game {
    pub fn new(height : usize, width : usize) -> Self {
        Self {
            field : Field::new(height, width),
            players : Vec::new(),
            deck : create_deck((height * width) / 2),
            current_player_id : 0,
            clicked_card : None,
        }
    }
    pub fn add_player(&mut self, name : String) {
        let p = Player {
            name,
            collected_cards : Vec::new(),
        };
        self.players.push(p);
    }
    pub fn reset(&mut self) {
        self.field.clear_field();
        self.field.place_deck(&self.deck);
        for player in &mut self.players {
            player.collected_cards.clear();
        }
        self.current_player_id = 0;
    }
    pub fn open_card(&mut self, coord : &Coord) {
        self.clicked_card = Some(coord.clone());
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
        let card_id1 = self.field.field[coord1.0][coord1.1].unwrap();
        let card_id2 = self.field.field[coord2.0][coord2.1].unwrap();
        let col_cards = &mut self.players[player].collected_cards;

        if self.deck[card_id1].card_type == self.deck[card_id2].card_type {
            col_cards.push(card_id1);
            col_cards.push(card_id2);

            self.field.field[coord1.0][coord1.1] = None;
            self.field.field[coord2.0][coord2.1] = None;
           return true;
        }
        false
    }
    pub fn check_game_over(& self) -> bool {
        for row in 0..self.field.height {
            for col in 0..self.field.width {
                if self.field.field[row][col] != None {
                    return false;
                }
            }
        }
        true
    }
    pub fn next_player(&mut self) {
        self.current_player_id += 1;
        if self.current_player_id >= self.players.len() {
            self.current_player_id = 0;
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
    pub fn print_cards_of_player(& self, player_id : usize) {
        println!("Player {} has {} cards:", self.players[player_id].name, self.players[player_id].collected_cards.len());
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
