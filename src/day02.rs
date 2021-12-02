use crate::inputs::read_file_to_list;
use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    h: u32,
    d: u32,
}

impl Add<Command> for Pos {
    type Output = Pos;

    fn add(self, rhs: Command) -> Self::Output {
        match rhs {
            Command::Down(amt) => Self {
                d: self.d + amt,
                ..self
            },
            Command::Forward(amt) => Self {
                h: self.h + amt,
                ..self
            },
            Command::Up(amt) => Self {
                d: self.d - amt,
                ..self
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug)]
struct CommandParseError {
    msg: String,
}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldn't parse {} as a string", self.msg)
    }
}

impl std::error::Error for CommandParseError {}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command: Vec<&str> = s.split_whitespace().collect();
        if command.len() != 2 {
            return Err(CommandParseError { msg: s.to_string() });
        }

        let value: Result<u32, _> = command[1].parse();
        if value.is_err() {
            return Err(CommandParseError { msg: s.to_string() });
        }
        let value = value.unwrap();
        match command[0] {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(CommandParseError { msg: s.to_string() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, Pos};

    #[test]
    fn it_parses_commands() {
        let c: Command = "forward 42".parse().unwrap();
        assert_eq!(Command::Forward(42), c);
        let c: Command = "down 2".parse().unwrap();
        assert_eq!(Command::Down(2), c);
        let c: Command = "up 55".parse().unwrap();
        assert_eq!(Command::Up(55), c);

        let err_strings = vec!["blargh 55", "forward forever", "eenie mine moo"];
        for command_str in err_strings.iter() {
            let res: Result<Command, _> = command_str.parse();
            assert!(res.is_err());
        }
    }

    #[test]
    fn it_can_add_commands_to_positions() {
        let start_pos = Pos { h: 0, d: 0 };
        let next_pos = start_pos + Command::Forward(5);
        assert_eq!(Pos { h: 5, d: 0 }, next_pos);
        let next_pos = next_pos + Command::Down(10);
        assert_eq!(Pos { h: 5, d: 10 }, next_pos);
        let next_pos = next_pos + Command::Up(7);
        assert_eq!(Pos { h: 5, d: 3 }, next_pos)
    }
}
