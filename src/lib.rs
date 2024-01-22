use std::fmt;

#[derive(Debug)]
pub struct Guess {
    text: String,
}

#[derive(Debug)]
pub struct GuessError;

impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wasn't given 5 letters exactly!")
    }
}

impl Guess {
    /// # Errors
    /// Will return an error if given a string that isn't 5 letters
    pub fn build(text: String) -> Result<Self, GuessError> {
        if text.len() != 5 {
            return Err(GuessError);
        }
        Ok(Self { text })
    }
}
impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = Guess::build("crane".to_owned()).unwrap();
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: GuessError")]
    fn it_doesnt_work_too() {
        let _ = Guess::build("wow".to_owned()).unwrap();
    }
}
