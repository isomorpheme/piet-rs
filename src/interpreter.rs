use std::cmp::Ordering;
use std::collections::HashSet;

use num::Integer;

use command::Command;
use errors::*;
use program::Program;
use stack::Stack;
use util::Coords;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug)]
pub struct Interpreter {
    program: Program,
    dp: DirectionPointer,
    cc: CodelChooser,
    stack: Stack,
    position: Coords,
    last_block: u64,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Interpreter {
            program: program,
            dp: DirectionPointer::Right,
            cc: CodelChooser::Left,
            stack: Stack::new(),
            position: (0, 0),
            last_block: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while let Some(result) = self.step() {
            result?;
        }

        Ok(())
    }

    pub fn step(&mut self) -> Option<Result<()>> {
        unimplemented!()
    }

    fn current_block(&self) -> HashSet<Coords> {
        self.program.color_block(self.position)
    }

    fn next_coords(&self) -> Coords {
        use self::DirectionPointer as DP;
        use self::CodelChooser as CC;

        fn x_key(&(x, _): &Coords) -> usize {
            x
        }
        fn y_key(&(_, y): &Coords) -> usize {
            y
        }

        let current_block = self.current_block();

        let edge = if self.dp == DP::Up || self.dp == DP::Down {
            let iter = current_block.iter().map(|&c| c);

            let (_, farthest_y) = match self.dp {
                DP::Up => iter.min_by_key(y_key),
                DP::Down => iter.max_by_key(y_key),
                _ => unreachable!(),
            }.unwrap(); // We can unwrap here because the current block is never empty.

            current_block.iter()
                .filter(|&&(x, y)| y == farthest_y)
                .map(|&y| y)
                .collect::<Vec<_>>()
        } else if self.dp == DP::Left || self.dp == DP::Right {
            let iter = current_block.iter().map(|&c| c);

            let (farthest_x, _) = match self.dp {
                DP::Left => iter.min_by_key(x_key),
                DP::Right => iter.max_by_key(x_key),
                _ => unreachable!(),
            }.unwrap(); // Same as above.

            current_block.iter()
                .filter(|&&(x, y)| x == farthest_x)
                .map(|&x| x)
                .collect::<Vec<_>>()
        } else { unreachable!() }.into_iter();

        match self.dp {
            DP::Left => {
                match self.cc {
                    CC::Right => edge.max_by_key(y_key),
                    CC::Left => edge.min_by_key(y_key),
                }
            }
            DP::Right => {
                match self.cc {
                    CC::Right => edge.min_by_key(y_key),
                    CC::Left => edge.max_by_key(y_key),
                }
            }
            DP::Up => {
                match self.cc {
                    CC::Right => edge.max_by_key(x_key),
                    CC::Left => edge.min_by_key(x_key),
                }
            }
            DP::Down => {
                match self.cc {
                    CC::Right => edge.min_by_key(x_key),
                    CC::Left => edge.max_by_key(x_key),
                }
            }
        }
        .unwrap() // See above.
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
