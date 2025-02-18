use std::{ascii::Char, borrow::Cow, io::Write, process::ExitCode};

pub fn echo_main(args: &[String], mut stdout: impl Write) -> Result<ExitCode, ExitCode> {
    let mut args = args.into_iter();
    let mut new_line = true;
    let mut expand = false;
    let mut out: Vec<Cow<[u8]>> = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-n" => {
                new_line = false;
            }
            "-e" => {
                expand = true;
            }
            "-E" => {
                expand = false;
            }
            "--help" => {
                return Ok(ExitCode::SUCCESS);
            }
            "--version" => {
                return Ok(ExitCode::SUCCESS);
            }
            other => {
                out.push(other.as_bytes().into());
                break;
            }
        }
    }

    out.extend(args.map(|i| i.as_bytes().into()));

    if expand {
        out = out
            .into_iter()
            .map(|i| {
                if i.contains(&b'\\') {
                    escape_chars(&i).into()
                } else {
                    i
                }
            })
            .collect();
    }

    let out: Vec<_> = out
        .into_iter()
        .map(|i| i.into_owned())
        .intersperse(vec![b' '])
        .flatten()
        .collect();
    stdout.write_all(&out).or(Err(ExitCode::FAILURE))?;
    if new_line {
        stdout.write_all(b"\n").or(Err(ExitCode::FAILURE))?;
    }
    Ok(ExitCode::SUCCESS)
}

enum EscapeMode {
    Slash,
    Normal,
    Hex,
    Oct,
}

fn escape_chars(to_escape: &[u8]) -> Vec<u8> {
    let mut mode = EscapeMode::Normal;
    let mut escape_count = 0;
    to_escape
        .chunk_by(|a, b| match (&mode, a, b) {
            (EscapeMode::Slash, _, _) => {
                mode = EscapeMode::Normal;
                false
            }
            (_, b'\\', b'x') => {
                mode = EscapeMode::Hex;
                true
            }
            (EscapeMode::Hex, _, c) => {
                if c.is_ascii_hexdigit() && escape_count < 2 {
                    escape_count += 1;
                    true
                } else {
                    escape_count = 0;
                    mode = EscapeMode::Normal;
                    false
                }
            }
            (_, b'\\', b'0') => {
                mode = EscapeMode::Oct;
                true
            }
            (EscapeMode::Oct, _, c) => {
                if c.is_ascii_octdigit() && escape_count < 3 {
                    escape_count += 1;
                    true
                } else {
                    escape_count = 0;
                    mode = EscapeMode::Normal;
                    false
                }
            }
            (EscapeMode::Normal, b'\\', b'\\') => {
                mode = EscapeMode::Slash;
                true
            }
            (EscapeMode::Normal, b'\\', _) => true,
            _ => false,
        })
        .map_while(|chunk| match chunk {
            [b'\\', b'x', rest @ ..] => {
                let digits: Box<str> = rest.into_iter().map(|&i| char::from(i)).collect();
                let c = u8::from_str_radix(&digits, 16).unwrap();
                Some(c.into())
            }
            [b'\\', b'0', rest @ ..] => {
                let digits: Box<str> = rest.into_iter().map(|&i| char::from(i)).collect();
                let c = u16::from_str_radix(&digits, 8).unwrap();
                Some((c as u8).into())
            }
            [b'\\', b'c'] => None,
            [b'\\', c] => Some(match c {
                b'a' => Char::Bell.into(),
                b'b' => Char::Backspace.into(),
                b'e' => Char::Escape.into(),
                b'f' => Char::FormFeed.into(),
                b'n' => Char::LineFeed.into(),
                b'r' => Char::CarriageReturn.into(),
                b't' => Char::CharacterTabulation.into(),
                b'v' => Char::LineTabulation.into(),
                c => *c,
            }),
            [c] => Some(*c),
            _ => panic!("Faile to parse"),
        })
        .collect()
}
#[cfg(test)]
mod test {
    use super::escape_chars;

    #[test]
    fn test_escape() {
        let cases = [
            (b"\\".as_slice(), b"\\".as_slice()),
            (b"\\n".as_slice(), b"\n".as_slice()),
            (b"\\xFGF".as_slice(), b"\x0fGF".as_slice()),
            (b"\\0777".as_slice(), b"\xff".as_slice()),
        ];
        for (input, expected) in cases.into_iter() {
            let actual = escape_chars(input);
            assert_eq!(&expected, &actual);
        }
    }
}
