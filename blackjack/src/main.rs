use std::{io, usize};
use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;


#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        return self.cards.is_empty();
    }

    pub fn check_bust(&self) -> bool {
        return self.points() > 21;
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        return &self.cards;
    }

    pub fn add_card(&mut self, new_card: Card) {
        self.cards.push(new_card);
    }

    pub fn points(&self) -> u32 {
        let mut points = self.cards.iter().map(|card| card.get_value() ).sum::<u8>() as u32;
        if self.ace_cards() > 0 && points > 21 {
            let mut temp_ace = self.ace_cards();
            while temp_ace > 0 && points > 21 {
                points -= 10;
                temp_ace -= 1;
            }
        }
        return points;
    }

    pub fn display_cards(&self) {
        let mut top = String::new();
        let mut next_1 = String::new();
        let mut next_2 = String::new();
        let mut next_3 = String::new();
        let mut next_4 = String::new();
        let mut next_5 = String::new();
        let mut next_6 = String::new();
        let mut next_7 = String::new();
        let mut next_8 = String::new();
        for card in self.cards.iter() {
            let number = card.get_number_symbol();
            let symbol = card.get_suit_symbol();
            if number.len() > 1 {
                top.push_str("┌────────────┐");
                next_1 += &format!("│{}          │", number);
                next_2.push_str("│            │");
                next_3.push_str("│            │");
                next_4 += &format!("│     {}      │", symbol);
                next_5.push_str("│            │");
                next_6.push_str("│            │");
                next_7 += &format!("│          {}│", number);
                next_8.push_str("└────────────┘");
            } else {
                top.push_str("┌───────────┐");
                next_1 += &format!("│{}          │", number);
                next_2.push_str("│           │");
                next_3.push_str("│           │");
                next_4 += &format!("│     {}     │", symbol);
                next_5.push_str("│           │");
                next_6.push_str("│           │");
                next_7 += &format!("│          {}│", number);
                next_8.push_str("└───────────┘");
            }
        }
        println!("{}", top);
        println!("{}", next_1);
        println!("{}", next_2);
        println!("{}", next_3);
        println!("{}", next_4);
        println!("{}", next_5);
        println!("{}", next_6);
        println!("{}", next_7);
        println!("{}", next_8);
    }

    fn ace_cards(&self) -> u8 {
        let mut count: u8 = 0;
        for card in self.cards.iter() {
            if card.is_ace() {
                count += 1;
            }
        }
        return count;
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    name: String,
    suit: Suit,
    value: u8,
}

impl Card {
    pub fn new(name: &str, suit: Suit, value: u8) -> Self {
        Self { name: String::from(name), suit: suit, value: value }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_suit(&self) -> &Suit {
        return &self.suit;
    }

    pub fn get_value(&self) -> u8 {
        return self.value;
    }

    pub fn is_ace(&self) -> bool {
        return self.name == "Ace";
    }

    pub fn get_number_symbol(&self) -> &str {
        return match self.name.as_str() {
            "Two" => "2",
            "Three" => "3",
            "Four" => "4",
            "Five" => "5",
            "Six" => "6",
            "Seven" => "7",
            "Eight" => "8",
            "Nine" => "9",
            "Ten" => "10",
            "Jack" => "J",
            "Queen" => "Q",
            "King" => "K",
            "Ace" => "A",
            _ => "",
        }
    }

    pub fn get_suit_symbol(&self) -> &str {
        return match self.suit {
            Suit::Club => "♣",
            Suit::Spade => "♠",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: HashMap<u8, Card>,
    taken_cards: Vec<u8>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: HashMap<u8, Card> = HashMap::new();
        let numbers = [("Two", 2), ("Three", 3), ("Four", 4), ("Five", 5), ("Six", 6), ("Seven", 7), ("Eight", 8), ("Nine", 9), ("Ten", 10), ("Jack", 10), ("Queen", 10), ("King", 10), ("Ace", 11)];
        let mut index: u8 = 0;
        for suit in [Suit::Club, Suit::Spade, Suit::Diamond, Suit::Heart] {
            for (name, value) in numbers.iter() {
                cards.insert(index, Card::new(name, suit, *value));
                index += 1;
            }
        }
        return Self { cards: cards, taken_cards: Vec::new() }
    }

    pub fn get_random_card(&mut self) -> Card {
        if self.taken_cards.len() == 52 {
            panic!("Ran out of cards!");
        }

        let mut valid_numbers: Vec<u8> = Vec::new();
        for card_number in self.cards.keys() {
            if !self.taken_cards.contains(card_number) {
                valid_numbers.push(*card_number);
            }
        }

        let random_card_index = valid_numbers.choose(&mut thread_rng()).unwrap();
        self.taken_cards.push(*random_card_index);
        return self.cards.get(&random_card_index).unwrap().clone();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

pub struct Player {
    name: String,
    hand: Hand,
    score: usize,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self { name: String::from(name), hand: Hand::new(), score: 0 }
    }

    pub fn display(&self) {
        println!("Name: {}", self.name);
        self.hand.display_cards();
        // println!("Cards: {:?}", self.hand.get_cards());
        println!("Points: {}", self.points());
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    pub fn points(&self) -> u32 {
        return self.hand.points();
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }
}

pub struct Game {
    dealer: Player,
    players: Vec<Player>,
    deck: Deck,
    rounds: usize,
}

impl Game {
    pub fn new() -> Self {
        Self { dealer: Player::new("Dealer"), players: Vec::new(), deck: Deck::new(), rounds: 0 }
    }

    pub fn reset_deck(&mut self) {
        self.deck = Deck::new();
    }

    pub fn add_player(&mut self, player_name: &str) {
        self.players.push(Player::new(player_name));
    }

    pub fn round(&mut self) {
        if self.dealer.hand.is_empty() {
            self.dealer.add_card(self.deck.get_random_card());
            self.dealer.add_card(self.deck.get_random_card());
        }
        for player in self.players.iter_mut() {
            if player.hand.is_empty() {
                player.add_card(self.deck.get_random_card());
                player.add_card(self.deck.get_random_card());
            }
        }
        self.player_round();
    }

    pub fn player_round(&mut self) {
        let mut index = 0;
        for player in self.players.iter_mut() {
            let mut sticking: bool = false;
            while !sticking {
                println!("Stick (s) or twist (t)?\n");
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_n) => {
                            if input.eq(&String::from("s\n")) {
                                sticking = true;
                            } else if input.eq(&String::from("t\n")) {
                                player.add_card(self.deck.get_random_card());
                                if player.hand.check_bust() {
                                    sticking = true;
                                }
                            } else {}
                        }
                        Err(error) => println!("error: {}", error),
                    }
            }
            index += 1;
        }
    }
}

fn run_game() {
    let mut deck = Deck::new();
    
    let mut game = true;

    let mut dealer = Player::new("Dealer");
    let mut player = Player::new("John");
    // let mut player = Player::new(get_input("Enter your name: "));

    dealer.add_card(deck.get_random_card());
    dealer.add_card(deck.get_random_card());

    player.add_card(deck.get_random_card());
    player.add_card(deck.get_random_card());

    while game {
        dealer.display();
        player.display();
        println!();
        println!("Stick (s) or twist (t)?\n");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                println!("{}", input);
                if input.eq(&String::from("s\n")) {
                    game = false;
                } else if input.eq(&String::from("t\n")) {
                    player.add_card(deck.get_random_card());
                } else {}
            }
            Err(error) => println!("error: {}", error),
        }
    }
    if player.points() > dealer.points() {
        println!("You beat the dealer!");
    } else if player.points() < dealer.points() {
        println!("You lost to the dealer!");
    } else {
        println!("You drew with the dealer!");
    }
}

fn main() {
    run_game();
}
