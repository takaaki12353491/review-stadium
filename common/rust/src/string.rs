pub trait StringExt {
    fn to_option(self) -> Option<String>;
}

impl StringExt for String {
    fn to_option(self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
