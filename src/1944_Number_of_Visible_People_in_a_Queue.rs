// 1944_Number_of_Visible_People_in_a_Queue
// https://leetcode.cn/problems/number-of-visible-people-in-a-queue/description/

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut answer = vec![0; n]; // Initialize answer vector with zeros
        let mut stack: Vec<usize> = Vec::new(); // Stack to store indices

        for i in 0..n {
            // Count how many people on the stack are shorter than the current person
            while !stack.is_empty() && heights[*stack.last().unwrap()] < heights[i] {
                let idx = stack.pop().unwrap();
                answer[idx] += 1; // Increment count for the person at idx
            }

            // If stack is not empty, increment the count for the person at the top of the stack
            if !stack.is_empty() {
                let idx = *stack.last().unwrap();
                answer[idx] += 1;
            }

            stack.push(i); // Push the current index onto the stack
        }

        answer
    }
}

// To implement the function `can_see_persons_count` in Rust, we need to follow the same logic as previously described.
// We will use a stack to efficiently determine the number of people each person can see to their right in the queue.
// The stack will keep track of indices of the people in the queue.

// Here's how we can implement this:

// 1. Iterate through the `heights` vector.
// 2. Use a stack to keep track of indices. When we find a person who is taller than the person at the top of the stack, it means the taller person can see the shorter person. We pop from the stack until we find someone taller or the stack is empty.
// 3. The number of pops is the number of people the current person can see before seeing someone taller.
// 4. After the loop, we process the remaining indices in the stack. These are the people who couldn't see anyone taller than themselves to their right.

// Let's write the Rust code for this:

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut answer = vec![0; n]; // Initialize answer vector with zeros
        let mut stack: Vec<usize> = Vec::new(); // Stack to store indices

        for i in 0..n {
            // Count how many people on the stack are shorter than the current person
            while !stack.is_empty() && heights[*stack.last().unwrap()] < heights[i] {
                let idx = stack.pop().unwrap();
                answer[idx] += 1; // Increment count for the person at idx
            }

            // If stack is not empty, increment the count for the person at the top of the stack
            if !stack.is_empty() {
                let idx = *stack.last().unwrap();
                answer[idx] += 1;
            }

            stack.push(i); // Push the current index onto the stack
        }

        answer
    }
}

// This implementation should efficiently compute the number of people each person in the queue can see to their right, adhering to the rules specified in your problem statement.
