use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden = s.chars().any(|g| forbidden_chars.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden {
            Err(format!("{} is not a valide subscriber name", s))
        } else {
            Ok(Self(s))
        }
    }

    //pub fn inner(self) -> String {
    //    self.0
    //}
    //pub fn inner_mut(&mut self) -> &mut str {
    //    &mut self.0
    //}
    //pub fn inner_ref(&self) -> &str {
    //    &self.0
    //}
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        claims::assert_ok!(SubscriberName::parse(name));
    }
    #[test]
    fn dummy_fail() {
        let result: Result<&str, &str> = Err("The app crashed due to an IO error");
        claims::assert_ok!(result);
    }
    #[test]
    fn dummy_fail() {
        let result: Result<&str, &str> = Err("The app crashed due to an IO error");
        claims::assert_ok!(result);
    }

    //#[test]
    //fn valid_name() {
    //    assert!(is_valid_name("Alice"));
    //}

    //#[test]
    //fn name_with_forbidden_char() {
    //    assert!(!is_valid_name("Alice <Test>"));
    //}
    //#[test]
    //fn name_too_long() {
    //    assert!(!is_valid_name(&"a".repeat(257)));
    //}
    //#[test]
    //fn name_empty() {
    //    assert!(!is_valid_name(""));
    //}
}
