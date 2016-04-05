use num::Integer;

use program::Program;
use command::Command;

#[derive(Debug, Copy, Clone)]
enum DirectionPointer {
    Up,
    Right,
    Down,
    Left,
}

impl DirectionPointer {
    pub fn rotate_clockwise(&mut self) {
        use self::DirectionPointer::*;

        *self = match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        use self::DirectionPointer::*;

        *self = match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum CodelChooser {
    Left,
    Right,
}

impl CodelChooser {
    pub fn switch(&mut self) {
        *self = match *self {
            CodelChooser::Left => CodelChooser::Right,
            CodelChooser::Right => CodelChooser::Left,
        }
    }
}

pub struct Interpreter {
    program: Program,
    dp: DirectionPointer,
    cc: CodelChooser,
    stack: Vec<i64>,
    position: (usize, usize),
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Interpreter {
            program: program,
            dp: DirectionPointer::Right,
            cc: CodelChooser::Left,
            stack: Vec::new(),
            position: (0, 0),
        }
    }

    pub fn step(&mut self) {
        unimplemented!();
    }

    fn calculate_offset(&self) -> (usize, usize) {
        unimplemented!()
    }

    fn execute_command(&mut self, command: &Command) {
        match *command {
            Command::Push => {
                unimplemented!();
            }

            Command::Pop => {
                self.stack.pop();
            }

            Command::Add => {
                self.combine_and_push(|a, b| a + b);
            }

            Command::Subtract => {
                self.combine_and_push(|a, b| a - b);
            }

            Command::Multiply => {
                self.combine_and_push(|a, b| a * b);
            }

            Command::Divide => {
                self.combine_and_push(|a, b| a / b);
            }

            Command::Mod => {
                self.combine_and_push(|a, b| a.mod_floor(&b));
            }

            Command::Not => {
                if let Some(value) = self.stack.pop() {
                    self.stack.push(if value != 0 {
                        0
                    } else {
                        1
                    });
                }
            }

            Command::Greater => {
                self.combine_and_push(|a, b| {
                    if a > b {
                        1
                    } else {
                        0
                    }
                });
            }

            Command::Pointer => {
                if let Some(value) = self.stack.pop() {
                    self.pointer(value);
                }
            }

            Command::Switch => {
                if let Some(value) = self.stack.pop() {
                    self.switch(value);
                }
            }

            Command::Duplicate => {
                if let Some(&value) = self.stack.last() {
                    self.stack.push(value);
                }
            }

            Command::Roll => {
                unimplemented!();
            }

            Command::IntIn => {
                unimplemented!();
            }

            Command::CharIn => {
                unimplemented!();
            }

            Command::IntOut => {
                unimplemented!();
            }

            Command::CharOut => {
                unimplemented!();
            }

            Command::NoOp => return,
        }
    }

    fn pointer(&mut self, steps: i64) {
        if steps >= 0 {
            for _ in 0..steps {
                self.dp.rotate_clockwise();
            }
        } else {
            for _ in 0..steps.abs() {
                self.dp.rotate_counterclockwise();
            }
        }
    }

    fn switch(&mut self, times: i64) {
        for _ in 0..times {
            self.cc.switch();
        }
    }

    fn roll(&mut self, depth: usize, times: i64) {
        if depth < 0 || depth >= self.stack.len() {
            return;
        }

        let index = self.stack.len() - depth;

        if times >= 0 {
            for _ in 0..times {
                let value = self.stack.pop().unwrap();
                // We can unwrap here, because if the stack is empty, the depth
                // is always larger than the length, so there would be an
                // early return.

                self.stack.insert(index, value);
            }
        } else {
            for _ in 0..times.abs() {
                let value = self.stack.swap_remove(index);
                self.stack.push(value);
            }
        }
    }

    fn combine_and_push<F>(&mut self, combine: F)
        where F: Fn(i64, i64) -> i64
    {
        if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(combine(a, b));
        }
    }
}
