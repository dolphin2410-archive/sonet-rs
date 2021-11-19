pub struct Optional<T> {
    pub data: OptionalData<T>
}

pub enum OptionalData<T> {
    NULL,
    DATA(T)
}

impl <T> Optional<T> {
    pub fn of(data: OptionalData<T>) -> Optional<T> {
        Optional { data }
    }

    pub fn set(&mut self, data: OptionalData<T>) {
        self.data = data;
    }
}

impl <T> OptionalData<T> {
    pub fn get(&self) -> Result<&T, &str> {
        match self {
            OptionalData::DATA(value) => Ok(value),
            _ => Err("NULL")
        }
    }

    pub fn as_mut(&mut self) -> Result<&mut T, &str> {
        match self {
            OptionalData::DATA(value) => Ok(value),
            _ => Err("NULL")
        }
    }
}