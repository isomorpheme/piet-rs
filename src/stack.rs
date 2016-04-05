use std::ops::{Deref, DerefMut};

pub struct Stack(Vec<i64>);

impl Stack {
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    pub fn peek(&self) -> Option<&i64> {
        self.last()
    }

    pub fn map_top<F>(&mut self, function: F)
        where F: Fn(i64) -> i64
    {
        if let Some(reference) = self.last_mut() {
            *reference = function(*reference);
        }
    }

    pub fn fold_top<F>(&mut self, function: F)
        where F: Fn(i64, i64) -> i64
    {
        if let (Some(first), Some(second)) = self.pop2() {
            self.push(function(first, second));
        }
    }

    pub fn pop2(&mut self) -> (Option<i64>, Option<i64>) {
        (self.pop(), self.pop())
    }

    pub fn roll(&mut self, depth: usize, times: i64) {
        if depth >= self.len() {
            return;
        }

        let index = self.len() - depth;

        if times >= 0 {
            for _ in 0..times {
                let value = self.pop().unwrap();
                // We can unwrap here, because if the stack is empty, the depth
                // is always larger than the length, so there would be an
                // early return.

                self.insert(index, value);
            }
        } else {
            for _ in 0..times.abs() {
                let value = self.swap_remove(index);
                self.push(value);
            }
        }
    }
}

impl Deref for Stack {
    type Target = Vec<i64>;

    fn deref(&self) -> &Vec<i64> {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Vec<i64> {
        &mut self.0
    }
}
