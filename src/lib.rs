use std::fmt;

#[derive(Debug)]
pub struct Guess {
    text: String,
}

#[derive(Debug)]
pub struct GuessError<'a>(&'a str);

impl fmt::Display for GuessError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(error_text) = self;
        write!(f, "{error_text}")
    }
}

impl Guess {
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
        if !text.chars().all(char::is_alphanumeric) {
            return Err(GuessError("wasn't given alphanumeric string!"))
        }
        Ok(Self { text })
    }

    /// # Safety
    /// Has no checking for length or alphanumeric-ness.
    /// Recommended that you use build() instead.
    #[must_use]
    pub const unsafe fn new (text:String) -> Self {
        Self{ text }
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
        let _ = Guess::build("radio".to_owned())
            .expect("Building radio shouldn't ever fail in this test");
    }

    #[test]
    #[should_panic(expected = "I want this test to fail: GuessError")]
    fn it_doesnt_work_too() {
        let _ = Guess::build("wow".to_owned()).expect("I want this test to fail");
    }
}
