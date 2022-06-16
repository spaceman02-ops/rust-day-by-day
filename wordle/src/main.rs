use colored::Colorize;
use std::{fs, io};
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let filename = "validWords.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|w| String::from(w))
        .collect();
    let mut w = Wordle {
        guesses: Vec::new(),
        guess: String::from(""),
        valid_words: contents,
        target_word: String::from("clean"),
    };
    w.game()
}
struct Wordle {
    guesses: Vec<Vec<colored::ColoredString>>,
    guess: String,
    valid_words: Vec<String>,
    target_word: String,
}

impl Wordle {
    fn get_input(&mut self) {
        println!("What is your guess?");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Invalid guess.\n");
        let guess = String::from(guess.trim());
        if guess.graphemes(true).collect::<Vec<&str>>().len() != 5 {
            println!("Your guess must be five letters long!\n");
            return;
        }
        if !self.valid_words.contains(&guess) {
            println!("That is not a valid word!\n");
            return;
        }
        self.guess = guess.clone();
    }
    fn display_guesses(&mut self) {
        for g in self.guesses.iter() {
            for c in g.iter() {
                print!("{}", c)
            }
            print!("\n")
        }
        print!("\n")
    }
    fn color_guess(&mut self, g: String) -> Vec<colored::ColoredString> {
        let mut output_string = vec![];
        let color_string = g.chars();
        let mut index = 0;
        for c in color_string {
            if c == self.target_word.chars().nth(index).unwrap() {
                output_string.push(c.to_string().white().on_green())
            } else if self.target_word.contains(c) {
                output_string.push(c.to_string().white().on_yellow())
            } else {
                output_string.push(c.to_string().white().on_black())
            }
            index = index + 1
        }
        return output_string;
    }
    fn game(&mut self) {
        self.display_guesses();
        self.get_input();
        if self.guess.len() < 1 {
            self.game();
            return;
        }
        let g = self.color_guess(self.guess.clone());
        self.guesses.push(g.clone());
        if self.guess == self.target_word {
            self.display_guesses();
            println!("You win!");
            return;
        }
        if self.guesses.len() > 5 {
            self.display_guesses();
            println!("You lose! The word was {}", self.target_word);
            return;
        }
        self.game()
    }
}
