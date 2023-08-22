// 849_Maximize_Distance_to_Closest_Person
// https://leetcode.cn/problems/maximize-distance-to-closest-person/

// You are given an array representing a row of seats where seats[i] = 1 
// represents a person sitting in the ith seat, and seats[i] = 0 represents that the ith seat is empty (0-indexed).

// There is at least one empty seat, and at least one person sitting.

// Alex wants to sit in the seat such that the distance between him and the closest person to him is maximized. 

// Return that maximum distance to the closest person.

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut l: Option<usize> = None; // Initialize l as None.
        let mut max_distance = 0;

        for i in 0..n {
            if seats[i] == 1 {
                if l.is_none() { // Check if l is None.
                    max_distance = i as i32;
                } else {
                    // Unwrap safely because we checked with is_none().
                    max_distance = max_distance.max((i - l.unwrap()) as i32 / 2);
                }
                l = Some(i);
            }
        }

        // For the case where the last seated person is not at the end.
        if seats[n-1] == 0 {
            max_distance = max_distance.max((n - 1 - l.unwrap()) as i32);
        }

        max_distance
    }
}