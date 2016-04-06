use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
pub struct Stack(Vec<i64>);

impl Stack {
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    pub fn from_vec(vec: Vec<i64>) -> Self {
        Stack(vec)
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
        match self.pop2() {
            (Some(first), Some(second)) => {
                self.push(function(first, second));
            }

            (Some(first), None) => self.push(first),
            _ => {}
        }
    }

    pub fn pop2(&mut self) -> (Option<i64>, Option<i64>) {
        (self.pop(), self.pop())
    }

    pub fn roll(&mut self, depth: usize, times: i64) {
        if depth >= self.len() {
            return;
        }

        let index = (self.len() - 1) - depth;

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
                let value = self.remove(index);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_new() {
        assert_eq!(Stack::new(), Stack(Vec::new()));
    }

    #[test]
    fn test_stack_peek() {
        let stack = Stack(vec![1, 2]);
        assert_eq!(stack.peek(), Some(&2));

        let stack = Stack(vec![1]);
        assert_eq!(stack.peek(), Some(&1));

        let stack = Stack(vec![]);
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_stack_map_top() {
        let mut stack = Stack(vec![1, 2]);
        stack.map_top(|_| 42);
        assert_eq!(stack, Stack(vec![1, 42]));

        let mut stack = Stack(vec![1]);
        stack.map_top(|_| 42);
        assert_eq!(stack, Stack(vec![42]));

        let mut stack = Stack(vec![]);
        stack.map_top(|_| 42);
        assert_eq!(stack, Stack(vec![]));
    }

    #[test]
    fn test_stack_fold_top() {
        let mut stack = Stack(vec![1, 4, 6]);
        stack.fold_top(|a, b| a - b);
        assert_eq!(stack, Stack(vec![1, 2]));

        let mut stack = Stack(vec![1, 3]);
        stack.fold_top(|a, b| a - b);
        assert_eq!(stack, Stack(vec![2]));

        let mut stack = Stack(vec![1]);
        stack.fold_top(|a, b| a - b);
        assert_eq!(stack, Stack(vec![1]));

        let mut stack = Stack(vec![]);
        stack.fold_top(|a, b| a - b);
        assert_eq!(stack, Stack(vec![]));
    }

    #[test]
    fn test_stack_pop2() {
        let mut stack = Stack(vec![1, 2, 3]);
        assert_eq!(stack.pop2(), (Some(3), Some(2)));
        assert_eq!(stack, Stack(vec![1]));

        let mut stack = Stack(vec![1, 2]);
        assert_eq!(stack.pop2(), (Some(2), Some(1)));
        assert_eq!(stack, Stack(vec![]));

        let mut stack = Stack(vec![1]);
        assert_eq!(stack.pop2(), (Some(1), None));
        assert_eq!(stack, Stack(vec![]));

        let mut stack = Stack(vec![]);
        assert_eq!(stack.pop2(), (None, None));
        assert_eq!(stack, Stack(vec![]));
    }
     #[test]
    fn test_stack_roll() {
        let mut stack = Stack(vec![1, 2, 3, 4]);
        stack.roll(2, 1);
        assert_eq!(stack, Stack(vec![1, 4, 2, 3]));

        let mut stack = Stack(vec![1, 2, 3, 4]);
        stack.roll(2, 2);
        assert_eq!(stack, Stack(vec![1, 3, 4, 2]));

        let mut stack = Stack(vec![1, 2, 3, 4]);
        stack.roll(2, -1);
        assert_eq!(stack, Stack(vec![1, 3, 4, 2]));

        let mut stack = Stack(vec![1, 2, 3, 4]);
        stack.roll(2, -2);
        assert_eq!(stack, Stack(vec![1, 4, 2, 3]));

        let mut stack = Stack(vec![]);
        stack.roll(1, 1);
        assert_eq!(stack, Stack(vec![]));
    }
}
