pub struct JIterator<T> {
    vec: Vec<T>,
    position: usize,
}

impl<T> JIterator<T> {
    pub fn new(vec: Vec<T>) -> Self {
        return Self {
            vec,
            position: 0,
        };
    }

    pub fn next(&mut self) -> &T {
        self.position += 1;
        &self.vec[self.position - 1]
    }
}