use std::{
    fmt::{Display, Write},
    slice::SliceIndex,
};

pub struct Text<'s>(pub &'s str);

enum Command {
    Dots,
    Ampersand,
    Dollar,
}

impl Command {
    fn from_span<R: SliceIndex<str, Output = str>>(s: &str, span: R) -> Option<Command> {
        match s.get(span) {
            Some("\\dots") => Some(Command::Dots),
            Some("\\&") => Some(Command::Ampersand),
            Some("\\$") => Some(Command::Dollar),
            _ => None,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Command::Dots => '…',
            Command::Ampersand => '&',
            Command::Dollar => '$',
        };

        f.write_char(c)
    }
}

enum PrintState {
    Normal,
    Dash,
    EnDash,
    Grave,
    Apostrofe,
    Command(usize),
}

impl<'s> Display for Text<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut state = PrintState::Normal;

        for (i, c) in self.0.chars().enumerate() {
            state = match state {
                PrintState::Normal => match c {
                    '-' => PrintState::Dash,
                    '`' => PrintState::Grave,
                    '\'' => PrintState::Apostrofe,
                    '\\' => PrintState::Command(i),
                    _ => {
                        f.write_char(c)?;
                        PrintState::Normal
                    }
                },
                PrintState::Dash => match c {
                    '-' => PrintState::EnDash,
                    c => {
                        f.write_char('-')?;
                        match c {
                            '`' => PrintState::Grave,
                            '\'' => PrintState::Apostrofe,
                            '\\' => PrintState::Command(i),
                            _ => {
                                f.write_char(c)?;
                                PrintState::Normal
                            }
                        }
                    }
                },
                PrintState::EnDash => match c {
                    '-' => {
                        f.write_char('—')?;
                        PrintState::Normal
                    }
                    c => {
                        f.write_char('–')?;
                        match c {
                            '`' => PrintState::Grave,
                            '\'' => PrintState::Apostrofe,
                            '\\' => PrintState::Command(i),
                            _ => {
                                f.write_char(c)?;
                                PrintState::Normal
                            }
                        }
                    }
                },
                PrintState::Grave => match c {
                    '`' => {
                        f.write_char('“')?;
                        PrintState::Normal
                    }
                    c => {
                        f.write_char('‘')?;
                        match c {
                            '-' => PrintState::Dash,
                            '\'' => PrintState::Apostrofe,
                            '\\' => PrintState::Command(i),
                            _ => {
                                f.write_char(c)?;
                                PrintState::Normal
                            }
                        }
                    }
                },
                PrintState::Apostrofe => match c {
                    '\'' => {
                        f.write_char('”')?;
                        PrintState::Normal
                    }
                    c => {
                        f.write_char('’')?;
                        match c {
                            '-' => PrintState::Dash,
                            '`' => PrintState::Grave,
                            '\\' => PrintState::Command(i),
                            _ => {
                                f.write_char(c)?;
                                PrintState::Normal
                            }
                        }
                    }
                },
                PrintState::Command(start) => match c {
                    ' ' => {
                        if let Some(cmd) = Command::from_span(self.0, start..i) {
                            write!(f, "{} ", cmd)?;
                        }
                        PrintState::Normal
                    }
                    '\\' => {
                        if start == i - 1 {
                            f.write_char('\\')?;
                            PrintState::Normal
                        } else {
                            if let Some(cmd) = Command::from_span(self.0, start..i) {
                                write!(f, "{}", cmd)?;
                            }
                            PrintState::Command(i)
                        }
                    }
                    _ => PrintState::Command(start),
                },
            };
        }

        match state {
            PrintState::Normal => Ok(()),
            PrintState::Dash => f.write_char('-'),
            PrintState::EnDash => f.write_char('–'),
            PrintState::Grave => f.write_char('‘'),
            PrintState::Apostrofe => f.write_char('’'),
            PrintState::Command(start) => {
                if let Some(cmd) = Command::from_span(self.0, start..) {
                    write!(f, "{}", cmd)
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[test]
fn number_range() {
    assert_eq!(format!("{}", Text("23--50")), "23–50",);
}

#[test]
fn em_dash() {
    assert_eq!(
        format!("{}", Text("Upon discovering the errors---all 124 of them---the publisher immediately recalled the books.")),
        "Upon discovering the errors—all 124 of them—the publisher immediately recalled the books.",
    );
}

#[test]
fn mixed() {
    assert_eq!(format!("{}", Text("--- -- -")), "— – -",)
}

#[test]
fn quotes() {
    assert_eq!(format!("{}", Text("`-'")), "‘-’",)
}

#[test]
fn quotes2() {
    assert_eq!(format!("{}", Text("``--''")), "“–”",)
}

#[test]
fn command() {
    assert_eq!(format!("{}", Text("\\$")), "$",);
    assert_eq!(format!("{}", Text("\\&")), "&",);
    assert_eq!(format!("{}", Text("\\dots")), "…",);
    assert_eq!(format!("{}", Text("\\invalid")), "",);
    assert_eq!(format!("{}", Text("\\$ \\& \\dots ")), "$ & … ",);
    assert_eq!(format!("{}", Text("\\$\\&\\\\\\dots")), "$&\\…",);
}

pub fn match_free_char(c: char) -> impl FnMut(char) -> bool {
    let mut count = 0u8;

    return move |d: char| {
        match d {
            d if c == d => count == 0,
            '{' => {
                count = count.saturating_add(1);
                false
            }
            '}' => {
                count = count.saturating_sub(1);
                false
            }
            _ => false,
        }
    }
}

#[test]
fn latex_space_match() {
    let s = "Os{ }Multantes da Record";

    assert_eq!(
        s.split_once(match_free_char(' ')),
        Some(("Os{ }Multantes", "da Record")),
    );
}
