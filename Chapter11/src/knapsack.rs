use std::cmp::max;

#[derive(Debug, PartialEq)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub struct Knapsack {
    capacity: usize,
}

pub trait DynamicProgramming {
    fn fill(&self, items: Vec<&Item>) -> u64;
}

pub trait Backtracking {
    fn fill(&self, items: Vec<&Item>) -> u64;
    fn fill_r(&self, remaining: &[&Item], current_weight: usize) -> i64;
}

impl Knapsack {
    pub fn new(capacity: usize) -> Knapsack {
        Knapsack { capacity: capacity }
    }
}

impl Backtracking for Knapsack {

    fn fill(&self, items: Vec<&Item>) -> u64 {
        let value = self.fill_r(&items, 0);
        if value < 0 {
            0
        } else {
            value as u64
        }
    }

    fn fill_r(&self, remaining: &[&Item], current_weight: usize) -> i64 {
        let w = current_weight;

        if w > self.capacity {
            return i64::min_value();
        }

        if remaining.len() > 0 && w < self.capacity {
            let include = remaining[0].value as i64
                + self.fill_r(&remaining[1..], current_weight + remaining[0].weight as usize);
            let exclude = self.fill_r(&remaining[1..], current_weight);
            if include >= exclude {
                include
            } else {
                exclude
            }
        } else {
            0
        }
    }

}

impl DynamicProgramming for Knapsack {
    
    fn fill(&self, items: Vec<&Item>) -> u64 {
        let mut cache = vec![vec![0u64; self.capacity + 1]; items.len() + 1];
        for i in 1..items.len() + 1 {
            for w in 1..self.capacity + 1 {
                if items[i -1].weight as usize <= w {
                    let prev_weight = w - (items[i - 1].weight as usize);
                    cache[i][w] = max(
                        items[i - 1].value as u64 + cache[i - 1][prev_weight],
                        cache[i - 1][w],
                    );
                } else {
                    cache[i][w] = cache[i - 1][w]
                }
            }
        }
        cache[items.len()][self.capacity]
    }
}
