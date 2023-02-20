use std::ptr::NonNull;

pub struct MyVec<T> {
    prt: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        Self {
            prt: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self) -> (){

    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self)-> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();
        // vec.push(1usize);
        // vec.push(2usize);
        // vec.push(3usize);

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }
}
