use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
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
