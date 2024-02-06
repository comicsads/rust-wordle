#![allow(dead_code)]
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Guess {
    text: String,
}

const GREEN: char = '🟩';
const YELLOW: char = '🟨';
const GRAY: char = '⬜';

#[derive(Debug)]
pub enum GuessError {
    NotFiveLetters,
    NotAlphabetic,
}

impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_text = match *self {
            Self::NotFiveLetters => "wasn't given 5 letters exactly!",
            Self::NotAlphabetic => "wasn't given alphabetic string!",
        };
        write!(f, "{error_text}")
    }
}

impl Guess {
    /// It is on the calling code to verify that text is in your dictionary
    /// # Errors
    /// Will return an error if given a string that isn't 5 letters or if it's not a-z.
    ///
    /// # Examples
    /// ```
    /// let crane = wordle_lib::Guess::build("crane".into());
    /// let rad = wordle_lib::Guess::build("rad".into());
    /// let bad_word = wordle_lib::Guess::build("@!@@4".into());
    ///
    /// assert!(crane.is_ok());
    /// assert!(rad.is_err());
    /// assert!(bad_word.is_err());
    /// ```
    pub fn build(text: String) -> Result<Self, GuessError> {
        if text.len() != 5 {
            return Err(GuessError::NotFiveLetters);
        }
        if !text.chars().all(char::is_alphabetic) {
            return Err(GuessError::NotAlphabetic);
        }
        unsafe { Ok(Self::new(text)) }
    }

    /// # Safety
    /// Has no checking for length or alphabetic-ness.
    /// Recommended that you use build() instead.
    ///
    /// # Examples
    /// ```
    /// let crane_safe = wordle_lib::Guess::build("crane".into()).unwrap();
    /// let crane_unsafe = unsafe {wordle_lib::Guess::new("crane".into())};
    ///
    /// assert_eq!(crane_safe, crane_unsafe);
    /// ```
    #[must_use]
    pub const unsafe fn new(text: String) -> Self {
        Self { text }
    }

    fn as_array(&self) -> [char; 5] {
        let mut build_array: [char; 5] = ['a', 'a', 'a', 'a', 'a'];
        for (i, c) in self.text.chars().enumerate() {
            build_array[i] = c;
        }
        build_array
    }

    #[must_use]
    pub fn verify(&self, answer: &Self) -> GameResponse {
        let mut resp: [GameResponseChar; 5] = GameResponseChar::five_greys();
        let mut answer_char_pointed_to_by_guess: [bool; 5] = [false, false, false, false, false];

        let guess_array = self.as_array();

        let answer_array = answer.as_array();

        for (i, guessed_char) in guess_array.iter().enumerate() {
            let answer_char = answer_array[i];
            if *guessed_char == answer_char {
                resp[i] = GameResponseChar::Green;
                answer_char_pointed_to_by_guess[i] = true;
            } else if answer.to_string().contains(*guessed_char) {
                resp[i] = GameResponseChar::Yellow;
            }
        }

        let resp_copy = resp.clone();
        // Goes through response and makes sure that two yellow characters aren't pointing to the
        // same letter in the answer
        // TODO: Try to make this more readable
        for (half_baked_resp_index, resp_char) in resp_copy.iter().enumerate() {
            if *resp_char == GameResponseChar::Yellow {
                let char_to_match = guess_array[half_baked_resp_index];
                for (answer_index, _char) in answer_array
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| **x == char_to_match)
                {
                    if !answer_char_pointed_to_by_guess[answer_index] {
                        resp[half_baked_resp_index] = GameResponseChar::Yellow;
                        answer_char_pointed_to_by_guess[answer_index] = true;
                        break;
                    }
                    resp[half_baked_resp_index] = GameResponseChar::Gray;
                }
            }
        }
        GameResponse::new_from_game_resp_char(resp)
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum GameResponseChar {
    Green,
    Yellow,
    Gray,
}

impl GameResponseChar {
    const fn to_char(&self) -> char {
        match *self {
            Self::Green => 'G',
            Self::Yellow => 'Y',
            Self::Gray => '-',
        }
    }

    const fn to_emoji(&self) -> char {
        match *self {
            Self::Green => GREEN,
            Self::Yellow => YELLOW,
            Self::Gray => GRAY,
        }
    }

    const fn five_greys() -> [Self; 5] {
        [Self::Gray, Self::Gray, Self::Gray, Self::Gray, Self::Gray]
    }
}

pub struct GameResponse {
    text: [GameResponseChar; 5],
}

impl GameResponse {
    /// - for Grey, C for Green, Y for Yellow.
    /// # Panics
    /// Will panic if string contains any characters that aren't G, Y, X or -.
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    fn new(text: String) -> Self {
        let mut my_array: [GameResponseChar; 5] = GameResponseChar::five_greys();
        for (i, c) in text.chars().enumerate() {
            my_array[i] = match c {
                'G' => GameResponseChar::Green,
                'Y' => GameResponseChar::Yellow,
                'X' | '-' => GameResponseChar::Gray,
                _ => {
                    unreachable!("GameResponse builder string contains char that isn't G, Y, or -!")
                }
            }
        }
        Self { text: my_array }
    }

    /// Returns string of G, Y, and -'s.
    pub fn unpretty_string(&self) -> String {
        self.text.iter().map(GameResponseChar::to_char).collect()
    }
    /// Returns string of Emoji's representing G, Y, and -.
    pub fn pretty_string(&self) -> String {
        self.text.iter().map(GameResponseChar::to_emoji).collect()
    }

    #[must_use]
    pub fn victory(&self) -> bool {
        self.text.iter().all(|x| *x == GameResponseChar::Green)
    }

    const fn new_from_game_resp_char(resp: [GameResponseChar; 5]) -> Self {
        Self { text: resp }
    }
}

impl fmt::Display for GameResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.unpretty_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = Guess::build("radio".to_owned())
            .expect("Building radio shouldn't ever fail in this test");
    }

    #[test]
    #[should_panic(expected = "NotFiveLetters")]
    fn it_doesnt_work_too() {
        let bad_guess = Guess::build("wow".to_owned());
        bad_guess.expect("I want this test to fail");
    }

    #[test]
    #[should_panic(expected = "NotAlphabetic")]
    fn test_guess_with_numbers() {
        let bad_guess = Guess::build("12345".to_string());
        bad_guess.expect("I want this test to fail");
    }

    macro_rules! test_gameresp {
        ($name_of_function:ident: $answer:expr, $result:expr) => {
            #[test]
            fn $name_of_function() {
                let guess =
                    Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
                let answer =
                    Guess::build($answer.to_string()).expect("value is hardcoded, shouldn't fail");
                let resp: GameResponse = guess.verify(&answer);
                assert_eq!(resp.to_string(), $result);
            }
        };
    }

    test_gameresp!(speed_speed: "speed", "GGGGG");
    test_gameresp!(speed_crepe: "crepe", "-YGY-");
    test_gameresp!(speed_erase: "erase", "Y-YY-");
    test_gameresp!(speed_abide: "abide", "--Y-Y");
    test_gameresp!(speed_steal: "steal", "G-G--");

    #[test]
    fn verify_response() {
        let guess = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let answer = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let resp: GameResponse = guess.verify(&answer);
        assert!(resp.victory());
    }

    #[test]
    fn verify_response_fail() {
        let guess = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let answer = Guess::build("speep".to_string()).expect("value is hardcoded, shouldn't fail");
        let resp: GameResponse = guess.verify(&answer);
        assert!(!resp.victory());
    }

    #[test]
    fn test_gameresp_pretty() {
        let resp = GameResponse::new("GY-XG".to_string());
        let my_array: [char; 5] = [GREEN, YELLOW, GRAY, GRAY, GREEN];
        let correct: String = my_array.iter().collect();
        assert_eq!(resp.pretty_string(), correct);
    }

    #[test]
    #[should_panic(expected = "char that isn't G, Y, or -!")]
    fn test_gameresp_pretty_crash() {
        let resp = GameResponse::new("GYGAX".to_string());
        resp.pretty_string();
    }
}
