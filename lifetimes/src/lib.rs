
struct MyIterWrapper<'a, T>{
    slice: &'a [T]
}

impl<'a,T> Iterator for MyIterWrapper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty(){
            return None;
        }

        let elem = self.slice.get(0);
        self.slice = &self.slice[1..];
        elem
    }
}


struct MyMutIterWrapper<'a, T>{
    slice: &'a mut [T]
}

impl<'a,T> Iterator for MyMutIterWrapper<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty(){
            return None;
        }
        let slice = &mut self.slice;
        let slice = std::mem::replace(slice, & mut []);
        let (first,rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let collection = vec![1,2,3,4,5];
        //collection[..];
        let wrapper = MyIterWrapper{
            slice: &collection[..],
        };
        for (index,elem) in wrapper.enumerate(){
            print!("{}-{} \n", index, elem)
        }


        let mut collection = vec![1,2,3,4,5];
        //collection[..];
        let wrapper = MyMutIterWrapper{
            slice: &mut collection[..],
        };
        for (index,elem) in wrapper.enumerate(){
            *elem = *elem + 1; 
            print!("{}-{} \n", index, elem)
        }

    }
}
