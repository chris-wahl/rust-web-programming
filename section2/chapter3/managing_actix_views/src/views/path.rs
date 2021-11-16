/// Defines the standard prefix for a URL
pub struct Path {
    pub prefix: String,
}

impl Path {
    pub fn define(&self, following_path: String) -> String {
        // Want to retain original prefix for re-use, so
        // using `.to_owned()` to create a clone of the
        // borrowed data before returning the connected result.
        self.prefix.to_owned() + &following_path
    }
}