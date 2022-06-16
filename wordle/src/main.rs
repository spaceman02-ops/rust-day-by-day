use std::io;
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let c = String::from("clear");
    let vw: Vec<String> = vec![c];
    let mut w = Wordle {
        guesses: Vec::new(),
        guess: String::from(""),
        words: Vec::new(),
        valid_words: vw,
        target_word: String::from(""),
    };
    w.get_input();
    println!("{}", w.guess);
}
struct Wordle {
    guesses: Vec<String>,
    guess: String,
    words: Vec<String>,
    valid_words: Vec<String>,
    target_word: String,
}

impl Wordle {
    fn get_input(&mut self) {
        println!("What is your guess?");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Invalid guess.");
        let guess = String::from(guess.trim());
        if guess.graphemes(true).collect::<Vec<&str>>().len() != 5 {
            print!("Your guess must be five letters long!");
            return;
        }
        if !self.valid_words.contains(&guess) {
            println!("That is not a valid word!");
            return;
        }
        self.guess = guess;
    }
    fn display_guesses(&mut self) {}
}
