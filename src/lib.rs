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
    /// Will return an error if given a string that isn't 5 letters.
    ///
    /// # Examples
    /// ```
    /// let rad = wordle::Guess::build("rad".to_string());
    ///
    /// assert!(rad.is_err())
    /// ```
    ///
    /// ```
    /// let crane = wordle::Guess::build("crane".to_string());
    ///
    /// assert!(crane.is_ok())
    /// ```
    pub fn build(text: String) -> Result<Self, GuessError> {
        if text.len() != 5 {
            return Err(GuessError::NotFiveLetters);
        }
        if !text.chars().all(char::is_alphabetic) {
            return Err(GuessError::NotAlphabetic);
        }
        Ok(Self { text })
    }

    /// # Safety
    /// Has no checking for length or alphanumeric-ness.
    /// Recommended that you use build() instead.
    ///
    /// # Examples
    /// ```
    /// unsafe {
    /// let crane_safe = wordle::Guess::build("crane".to_string()).unwrap();
    /// let crane_unsafe = wordle::Guess::new("crane".to_string());
    ///
    /// assert_eq!(crane_safe, crane_unsafe);
    /// }
    /// ```
    #[must_use]
    pub const unsafe fn new(text: String) -> Self {
        Self { text }
    }

    fn verify(&self, answer: &Self) -> GameResponse {
        let mut resp: [GameResponseChar; 5] = [
            GameResponseChar::Gray,
            GameResponseChar::Gray,
            GameResponseChar::Gray,
            GameResponseChar::Gray,
            GameResponseChar::Gray,
        ];
        let mut guess_array: [char; 5] = ['a', 'a', 'a', 'a', 'a'];
        for (i, c) in self.text.chars().enumerate() {
            guess_array[i] = c;
        }
        let guess_array = guess_array; //make immutable
        let mut answer_array: [char; 5] = ['a', 'a', 'a', 'a', 'a'];
        for (i, c) in answer.text.chars().enumerate() {
            answer_array[i] = c;
        }
        let answer_array = answer_array; //make immutable

        for (i, guessed_char) in guess_array.iter().enumerate() {
            let answer_char = answer_array[i];
            if *guessed_char == answer_char {
                resp[i] = GameResponseChar::Green;
            } else if answer.to_string().contains(*guessed_char) {
                resp[i] = GameResponseChar::Yellow(None);
            }
        }
        let mut new_resp = resp.clone();
        let mut taken: [bool; 5] = [false, false, false, false, false];
        for (i, resp_char) in resp.iter().enumerate() {
            if *resp_char == GameResponseChar::Yellow(None) {
                let char_to_match = guess_array[i];
                for (j, _) in answer_array
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| **x == char_to_match)
                {
                    if !taken[j] {
                        new_resp[i] = GameResponseChar::Yellow(Some(j));
                        taken[j] = true;
                    } else {
                        new_resp[i] = GameResponseChar::Gray;
                    }
                }
            }
        }
        assert!(
            new_resp
                .iter()
                .filter(|x| **x == GameResponseChar::Yellow(None))
                .next()
                == None
        );
        GameResponse::new_from_game_resp_char(new_resp)
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
    Yellow(Option<usize>),
    Gray,
}

impl GameResponseChar {
    const fn to_char(&self) -> char {
        match *self {
            Self::Green => 'G',
            Self::Yellow(_) => 'Y',
            Self::Gray => 'X',
        }
    }

    const fn to_emoji(&self) -> char {
        match *self {
            Self::Green => GREEN,
            Self::Yellow(_) => YELLOW,
            Self::Gray => GRAY,
        }
    }
}

struct GameResponse {
    text: [GameResponseChar; 5],
}

impl GameResponse {
    /// X for Grey, C for Green, Y for Yellow
    #[allow(clippy::needless_pass_by_value)]
    fn new(text: String) -> Self {
        let mut my_array: [GameResponseChar; 5] = [
            GameResponseChar::Gray,
            GameResponseChar::Gray,
            GameResponseChar::Gray,
            GameResponseChar::Gray,
            GameResponseChar::Gray,
        ];
        for (i, c) in text.chars().enumerate() {
            my_array[i] = match c {
                'G' => GameResponseChar::Green,
                'Y' => GameResponseChar::Yellow(None),
                'X' => GameResponseChar::Gray,
                _ => panic!("GameResponse builder string contains char that isn't G, Y, or X!"),
            }
        }
        Self { text: my_array }
    }

    fn unpretty_string(&self) -> String {
        self.text.iter().map(GameResponseChar::to_char).collect()
    }
    fn pretty_string(&self) -> String {
        self.text.iter().map(GameResponseChar::to_emoji).collect()
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
        let _ = Guess::build("wow".to_owned()).expect("I want this test to fail");
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
    // test_gameresp!(speed_crepe: "crepe", "XYGYX");
    // test_gameresp!(speed_erase: "erase", "YXYYX");
    test_gameresp!(speed_abide: "abide", "XXYXY");
    // test_gameresp!(speed_steal: "steal", "GXGXX");

    #[test]
    fn test_gameresp_pretty() {
        let resp = GameResponse::new("GYXYG".to_string());
        assert_eq!(resp.pretty_string(), "🟩🟨⬜🟨🟩");
    }

    #[test]
    #[should_panic(expected = "char that isn't G, Y, or X!")]
    fn test_gameresp_pretty_crash() {
        let resp = GameResponse::new("GYGAX".to_string());
        resp.pretty_string();
    }

    #[test]
    #[should_panic(expected = "NotAlphabetic")]
    fn test_guess_with_numbers() {
        let bad_guess = Guess::build("12345".to_string());
        bad_guess.expect("I want this test to fail");
    }
}
