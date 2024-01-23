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
pub struct GuessError<'a>(&'a str);

impl fmt::Display for GuessError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(error_text) = self;
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
    pub fn build(text: String) -> Result<Self, GuessError<'static>> {
        if text.len() != 5 {
            return Err(GuessError("wasn't given 5 letters exactly!"));
        }
        if !text.chars().all(char::is_alphabetic) {
            return Err(GuessError("wasn't given alphabetic string!"));
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
        let mut resp: [char; 5] = ['A', 'A', 'A', 'A', 'A'];
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
            } else {
                resp[i] = 'X';
            }
        }
        GameResponse {
            text: resp.iter().collect(),
        }
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

/// X for Grey, C for Green, Y for Yellow
#[derive(Debug)]
struct GameResponse {
    text: String,
}

impl GameResponse {
    /// X for Grey, C for Green, Y for Yellow
    const fn new(text: String) -> Self {
        Self { text }
    }

    fn pretty_string(&self) -> String {
        self.text
            .chars()
            .map(|c| match c {
                'G' => GREEN,
                'Y' => YELLOW,
                'X' => GRAY,
                _ => panic!("GameResponse contains char that isn't G, Y, or X!"),
            })
            .collect()
    }
}

impl fmt::Display for GameResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
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
    #[should_panic(expected = "I want this test to fail: GuessError")]
    fn it_doesnt_work_too() {
        let _ = Guess::build("wow".to_owned()).expect("I want this test to fail");
    }

    #[test]
    fn test_speed_speed() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let resp: GameResponse = speed.verify(&speed);
        assert_eq!(resp.text, "GGGGG");
    }

    #[test]
    fn test_speed_abide() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let abide = Guess::build("abide".to_string()).expect("value is hardcoded, shouldn't fail");

        let resp: GameResponse = speed.verify(&abide);
        assert_eq!(resp.text, "XXYXY");
    }

    #[test]
    fn test_speed_erase() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let erase = Guess::build("erase".to_string()).expect("value is hardcoded, shouldn't fail");

        let resp: GameResponse = speed.verify(&erase);
        assert_eq!(resp.text, "YXYYX");
    }

    #[test]
    fn test_speed_steal() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let steal = Guess::build("steal".to_string()).expect("value is hardcoded, shouldn't fail");

        let resp: GameResponse = speed.verify(&steal);
        assert_eq!(resp.text, "GXGXX");
    }

    #[test]
    fn test_speed_crepe() {
        let speed = Guess::build("speed".to_string()).expect("value is hardcoded, shouldn't fail");
        let crepe = Guess::build("crepe".to_string()).expect("value is hardcoded, shouldn't fail");

        let resp: GameResponse = speed.verify(&crepe);
        assert_eq!(resp.text, "XYGYX");
    }

    #[test]
    fn test_gameresp_pretty() {
        let resp = GameResponse {
            text: "GYX".to_string(),
        };
        assert_eq!(resp.pretty_string(), "🟩🟨⬜");
    }

    #[test]
    #[should_panic(expected = "GameResponse contains char that isn't G, Y, or X!")]
    fn test_gameresp_pretty_crash() {
        let resp = GameResponse {
            text: "GYGAX".to_string(),
        };
        resp.pretty_string();
    }

    #[test]
    #[should_panic(expected = "wasn't given alphabetic string")]
    fn test_guess_with_numbers() {
        let bad_guess = Guess::build("12345".to_string());
        bad_guess.unwrap();
    }
}
