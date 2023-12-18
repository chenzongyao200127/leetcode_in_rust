// 2276_Count_Integers_in_Intervals
// https://leetcode.cn/problems/count-integers-in-intervals/description/?envType=daily-question&envId=2023-12-16

// Import BTreeMap from the standard collections library
use std::collections::BTreeMap;

// Define the CountIntervals struct
struct CountIntervals {
    // A BTreeMap to store intervals in a sorted order.
    mp: BTreeMap<i32, i32>,
    // An integer to keep track of the total count of unique elements in all intervals.
    cnt: i32,
}

// Implementation block for CountIntervals
impl CountIntervals {
    // Constructor for CountIntervals
    fn new() -> Self {
        CountIntervals {
            // Initialize an empty BTreeMap for storing intervals
            mp: BTreeMap::new(),
            // Initialize the count to 0
            cnt: 0,
        }
    }

    // Method to add a new interval to the collection
    fn add(&mut self, mut left: i32, mut right: i32) {
        // Find the last interval that might overlap with the new interval
        let mut interval_index = self.mp.range(..=right).next_back();

        // Loop to merge overlapping intervals
        while let Some((&l, &r)) = interval_index {
            // Break the loop if there is no overlap
            if l > right || r < left {
                break;
            }

            // Update the bounds of the new interval to encompass the current interval
            left = left.min(l);
            right = right.max(r);

            // Update the count by removing the size of the current interval
            self.cnt -= r - l + 1;
            // Remove the current interval from the map
            self.mp.remove(&l);

            // Look for the next interval that might overlap
            interval_index = self.mp.range(..=right).next_back();
        }

        // Add the size of the new/merged interval to the total count
        self.cnt += right - left + 1;
        // Insert the new/merged interval into the map
        self.mp.insert(left, right);
    }

    // Method to get the total count of unique elements in all intervals
    fn count(&self) -> i32 {
        self.cnt
    }
}


/**
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 */