use std::fmt;

// Define the ListNode struct which will be the building block for our List.
pub struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

// Define the List struct. It will only contain the head of the list.
pub struct List<T> {
    head: Option<Box<ListNode<T>>>,
}

impl<T> List<T> {
    // Create a new, empty list.
    pub fn new() -> Self {
        List { head: None }
    }

    // Insert a new element at the beginning of the list.
    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(ListNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Reverse the list in place.
    pub fn revert(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
            boxed_node.next = prev;
            prev = Some(boxed_node);
        }
        self.head = prev;
    }
}

// Implementing the Drop trait to handle cleanup when the list goes out of scope.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
            // boxed_node goes out of scope and is dropped here, no need for explicit cleanup
        }
    }
}

// Implement Display trait to print the list.
impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        while let Some(boxed_node) = current {
            write!(f, "{} ", boxed_node.value)?;
            current = &boxed_node.next;
        }
        Ok(())
    }
}

fn main() {
    let mut lst = List::new();
    lst.insert(2);
    lst.insert(3);
    lst.insert(4);
    lst.insert(5);

    println!("Original List: {}", lst);

    lst.revert();

    println!("Reverted List: {}", lst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_reversal() {
        let mut list = List::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);

        // The list should be 3 -> 2 -> 1
        let mut current = &list.head;
        let mut values = Vec::new();
        while let Some(ref node) = current {
            values.push(node.value);
            current = &node.next;
        }
        assert_eq!(values, vec![3, 2, 1]);

        // After reversal, the list should be 1 -> 2 -> 3
        list.revert();
        current = &list.head;
        values.clear();
        while let Some(ref node) = current {
            values.push(node.value);
            current = &node.next;
        }
        assert_eq!(values, vec![1, 2, 3]);
    }
}
