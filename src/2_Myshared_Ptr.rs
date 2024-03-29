use std::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct MySharedPtr<T> {
    value: NonNull<T>,                     // Non-null raw pointer
    reference_count: NonNull<Cell<usize>>, // Non-null raw pointer with reference count
    _marker: PhantomData<T>,               // Enables proper covariance and contravariance
}

impl<T> MySharedPtr<T> {
    // Create a new MySharedPtr
    pub fn new(value: T) -> Self {
        let value_box = Box::new(value);
        let count_box = Box::new(Cell::new(1));

        MySharedPtr {
            value: NonNull::new(Box::into_raw(value_box)).expect("Failed to create value pointer"),
            reference_count: NonNull::new(Box::into_raw(count_box))
                .expect("Failed to create reference count pointer"),
            _marker: PhantomData,
        }
    }

    // Get a reference to the value
    pub fn get(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

// Implementing Clone to increase the reference count
impl<T> Clone for MySharedPtr<T> {
    fn clone(&self) -> MySharedPtr<T> {
        unsafe {
            let count_cell = self.reference_count.as_ref();
            count_cell.set(count_cell.get() + 1);
        }

        MySharedPtr {
            value: self.value,
            reference_count: self.reference_count,
            _marker: PhantomData,
        }
    }
}

// Implementing Drop to decrease the reference count and drop the value if it's the last reference
impl<T> Drop for MySharedPtr<T> {
    fn drop(&mut self) {
        unsafe {
            let count_cell = self.reference_count.as_ref();
            if count_cell.get() == 1 {
                drop(Box::from_raw(self.value.as_ptr()));
                drop(Box::from_raw(self.reference_count.as_ptr()));
            } else {
                count_cell.set(count_cell.get() - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_owner() {
        let msp = MySharedPtr::new(10);
        assert_eq!(*msp.get(), 10);
    }

    #[test]
    fn multiple_owners() {
        let msp1 = MySharedPtr::new(20);
        {
            let msp2 = msp1.clone();
            assert_eq!(*msp1.get(), 20);
            assert_eq!(*msp2.get(), 20);
        }
        // msp2 goes out of scope here, msp1 should still be valid
        assert_eq!(*msp1.get(), 20);
    }

    #[test]
    fn owner_after_clone_drop() {
        let msp1 = MySharedPtr::new(30);
        {
            let msp2 = msp1.clone();
            assert_eq!(*msp1.get(), 30);
            assert_eq!(*msp2.get(), 30);
        }
        // After msp2 is dropped, msp1 should still be valid
        assert_eq!(*msp1.get(), 30);
    }
}

fn main() {
    let a = MySharedPtr::new(10);
    println!("a: {}", a.get());

    {
        let b = a.clone();
        println!("a: {}, b: {}", a.get(), b.get());
        // b goes out of scope here, and its destructor is called, but the data is not dropped
    }

    // a is still valid, so we can still access the data
    println!("a: {}", a.get());
}
