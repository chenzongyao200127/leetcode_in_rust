impl Solution {
    fn fractional_knapsack(prices: Vec<usize>, weights: usize, capacity: usize) -> usize {
        let len = prices.len();

        let mut items = vec![];
        for i in 0..len {
            items.push(prices[i] / weights[i], prices[i], weights[i]);
        }

        items.sort_by_key(|a, b| (b.0).cmp((a.0)));

        let mut total_value = 0;
        let mut remaining_capacity = capacity;

        for (value_density, value, weight) in items {
            if remaining_capacity >= weight {
                total_value += value;
                remaining_capacity -= weight;
            } else {
                let mut fraction = remaining_capacity / weight;
                total_value += value * fraction;
                break
            }
        }

        return total_value
    }
}