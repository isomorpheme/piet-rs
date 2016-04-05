use self::Command::*;

/// A command, which operates on Piet's stack.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    NoOp,

    // Stack related.
    Push,
    Pop,
    Duplicate,
    Roll,

    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,

    // Boolean
    Not,
    Greater,

    // Control Flow
    Pointer,
    Switch,

    // I/O
    IntIn,
    IntOut,
    CharIn,
    CharOut,
}

impl Command {
    const COMMAND_TABLE: [[Command; 3]; 6] = [[NoOp, Push, Pop],
                                              [Add, Subtract, Multiply],
                                              [Divide, Mod, Not],
                                              [Greater, Pointer, Switch],
                                              [Duplicate, Roll, IntIn],
                                              [CharIn, IntOut, CharOut]];

    pub fn from_transition(transition: (u8, u8)) -> Self {
        let (hue, lightness) = transition;

        Command::COMMAND_TABLE[hue as usize][lightness as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_transition(){
        assert_eq!(Command::from_transition((0, 0)), Command::NoOp);
        assert_eq!(Command::from_transition((5, 2)), Command::CharOut);
        assert_eq!(Command::from_transition((3, 1)), Command::Pointer);
    }
}
