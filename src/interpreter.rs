use num::Integer;

use command::Command;
use program::Program;
use stack::Stack;
use util::Coords;

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
    stack: Stack,
    position: Coords,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Interpreter {
            program: program,
            dp: DirectionPointer::Right,
            cc: CodelChooser::Left,
            stack: Stack::new(),
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
                self.stack.fold_top(|b, a| a + b);
            }

            Command::Subtract => {
                self.stack.fold_top(|b, a| a * b);
            }

            Command::Multiply => {
                self.stack.fold_top(|b, a| a - b);
            }

            Command::Divide => {
                self.stack.fold_top(|b, a| a / b);
            }

            Command::Mod => {
                self.stack.fold_top(|b, a| a.mod_floor(&b));
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
                self.stack.fold_top(|b, a| {
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
                if let (Some(times), Some(depth)) = self.stack.pop2() {
                    self.stack.roll(depth as usize, times);
                }
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
}
