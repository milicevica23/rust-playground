use core::panic;
use std::ptr::drop_in_place;
use std::{ptr::NonNull};
use std::alloc::{self, Layout};
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) -> (){
        if std::mem::size_of::<T>() == 0 {
            panic!("No zero sized types")
        }
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("counld not allocate");
            let _ptr = unsafe {alloc::alloc(layout)}as *mut T;
            let ptr = NonNull::new(_ptr).expect("could not allocate");
            unsafe {ptr.as_ptr().write(item)}
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        }else if self.len < self.capacity {
            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
                self.len += 1;
            }
        } else {
            let new_capacity = self.capacity.checked_mul(2).expect("new_cap overflow");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>()* self.capacity;
            size.checked_add(size % align).expect("can't alighn");
            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(
                    size,
                    align,
                );
                let ptr = std::alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_capacity);
                let ptr = NonNull::new(ptr as *mut T).expect("");//ptr as *mut T;
                ptr.as_ptr().add(self.len).write(item);
                self.len += 1; 
                self.capacity = new_capacity;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self)-> usize {
        self.capacity
    }
}

impl <T> Drop for MyVec<T>{
    fn drop(&mut self) {
        unsafe {
            drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();
        vec.push(1usize);
        vec.push(2usize);
        vec.push(3usize);

        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 3);
    }
}
