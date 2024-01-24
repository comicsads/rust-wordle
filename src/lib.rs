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
        let mut resp: [char; 5] = ['X', 'X', 'X', 'X', 'X'];
        for (i, guessed_char) in self.to_string().chars().enumerate() {
            let answer_char = answer
                .text
                .chars()
                .nth(i)
                .expect("assuming guess and answer are both length 5 has failed us");
            if guessed_char == answer_char {
                resp[i] = 'G';
            } else if answer.to_string().contains(guessed_char) {
                resp[i] = 'Y';
            }
        }
        GameResponse::new(resp.iter().collect())
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

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
            Self::Gray => 'X',
        }
    }

    const fn to_emoji(&self) -> char {
        match *self {
            Self::Green => GREEN,
            Self::Yellow => YELLOW,
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
                'Y' => GameResponseChar::Yellow,
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

    #[test]
    fn test_speed_speed() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let resp: GameResponse = speed.verify(&speed);
        assert_eq!(resp.to_string(), "GGGGG");
    }

    #[test]
    #[ignore]
    fn test_speed_abide() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let abide = Guess::build("abide".to_string()).expect("value is hardcoded, shouldn't fail");

        let resp: GameResponse = speed.verify(&abide);
        assert_eq!(resp.to_string(), "XXYXY");
    }

    #[test]
    fn test_speed_erase() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let erase = Guess::build("erase".to_string()).expect("value is hardcoded, shouldn't fail");

        let resp: GameResponse = speed.verify(&erase);
        assert_eq!(resp.to_string(), "YXYYX");
    }

    #[test]
    #[ignore]
    fn test_speed_steal() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let steal = Guess::build("steal".to_string()).expect("value is hardcoded, shouldn't fail");

        let resp: GameResponse = speed.verify(&steal);
        assert_eq!(resp.to_string(), "GXGXX");
    }

    #[test]
    fn test_speed_crepe() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let crepe = Guess::build("crepe".to_string()).expect("value is hardcoded, shouldn't fail");

        let resp: GameResponse = speed.verify(&crepe);
        assert_eq!(resp.to_string(), "XYGYX");
    }

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
