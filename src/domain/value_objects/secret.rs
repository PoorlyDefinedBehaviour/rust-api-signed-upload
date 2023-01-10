pub struct Secret<T> (T);

impl<T> Secret<T> {
    pub fn expose(&self) -> &T {
        &self.0
    }
}

impl From<String> for Secret<String> {
    fn from(value: String) -> Self {
        Secret(value)
    }
}

impl<'a> From<&'a str> for Secret<&'a str> {
    fn from(value: &'a str) -> Self {
        Secret(value)
    }
}
