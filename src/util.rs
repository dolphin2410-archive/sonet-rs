/// Works like Java's Iterator. Used to add order-based macro functions
pub struct JIterator<T> {

    /// The full data
    vec: Vec<T>,

    /// The current position
    position: usize,
}

/// Default Implementation
impl<T> JIterator<T> {

    /// Creates an iterator instance
    pub fn new(vec: Vec<T>) -> Self {
        return Self {
            vec,
            position: 0,
        };
    }

    /// Returns the value of the current position and increments the position by one.
    pub fn next(&mut self) -> &T {
        self.position += 1;
        &self.vec[self.position - 1]
    }
}