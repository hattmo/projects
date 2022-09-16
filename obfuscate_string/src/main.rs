extern "C" {
    fn tree_sitter_c() -> Language;
}
use anyhow::{anyhow, Result};
use std::fs::{read, write};
use tree_sitter::{Language, Parser};
use tree_sitter_traversal::Order;

use clap::Parser as ClapParser;

/// Simple program to greet a person
#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(value_parser)]
    input: String,

    /// Number of times to greet
    #[clap(value_parser)]
    output: String,
}

fn main() -> Result<()> {
    let settings = Args::parse();
    if let Err(error) = obfuscate_source(settings.input, settings.output) {
        println!("{:?}", error);
    };
    Ok(())
}

fn obfuscate_source(input: String, output: String) -> Result<()> {
    let language = unsafe { tree_sitter_c() };
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .or(Err(anyhow!("Could not set language of parser to C")))?;
    let mut file = read(input).or(Err(anyhow!("Could not read from input file")))?;
    let mut store = StringStore::new();
    loop {
        if !pass_source(&mut file, &mut parser, &mut store)? {
            break;
        }
    }
    let mut out = store.build_template();
    out.append(&mut file);
    write(output, out).or(Err(anyhow!("Could not write to output file")))?;
    Ok(())
}

fn pass_source(file: &mut Vec<u8>, parser: &mut Parser, store: &mut StringStore) -> Result<bool> {
    let tree = parser
        .parse(&file, None)
        .ok_or(anyhow!("Failed to parse source file"))?;
    let tree_traverse = tree_sitter_traversal::traverse_tree(&tree, Order::Pre);
    for node in tree_traverse {
        if node.kind() == "string_literal" && is_in_function(node) {
            let plain_text_range = node.byte_range();
            if plain_text_range.len() < 3 {
                continue;
            }

            let plain_text = &file[plain_text_range.clone()];
            let call = store.add_string(&plain_text[1..plain_text.len() - 1]);
            file.splice(plain_text_range, call.bytes());
            return Ok(true);
        };
    }
    return Ok(false);
}

fn is_in_function(mut node: tree_sitter::Node) -> bool {
    while let Some(parent) = node.parent() {
        if parent.kind() == "function_definition" {
            return true;
        }
        node = parent;
    }
    return false;
}

#[derive(Debug)]
struct StringStore {
    count: usize,
    blob1: Vec<u8>,
    blob2: Vec<u8>,
}

impl StringStore {
    pub fn new() -> Self {
        StringStore {
            count: 0,
            blob1: Vec::new(),
            blob2: Vec::new(),
        }
    }
    pub fn add_string(&mut self, in_string: &[u8]) -> String {
        let start = self.blob1.len();
        let end = start + in_string.len() + 1;
        let index = self.count;
        let _ = transform_escape(in_string);
        self.count += 1;
        for c in in_string {
            let key: u8 = rand::random();
            self.blob1.push(key);
            self.blob2.push(key ^ c);
        }

        let key: u8 = rand::random();
        self.blob1.push(key);
        self.blob2.push(key);

        return format!("lookup({},{},{})", index, start, end);
    }

    fn build_template(self) -> Vec<u8> {
        let table_string = (0..self.count)
            .into_iter()
            .map(|_| "0".to_owned())
            .collect::<Vec<String>>()
            .join(",");
        let blob1_string = self
            .blob1
            .iter()
            .map(|item| format!("0x{:02x}", item))
            .collect::<Vec<String>>()
            .join(",");
        let blob2_string = self
            .blob2
            .iter()
            .map(|item| format!("0x{:02x}", item))
            .collect::<Vec<String>>()
            .join(",");
        format!(
            "
#include <stdlib.h>

char blob1[] = {{{}}};
char blob2[] = {{{}}};
char *table[] = {{{}}};

static char *lookup(int index, int start, int end)
{{
    if (table[index] == 0)
    {{
        char *target = malloc(end - start);
        table[index] = target;
        for (int i = start; i < end; i++)
        {{
            *target = blob1[i] ^ blob2[i];
            target++;
        }}
    }}
    return table[index];
}}\n\n",
            blob1_string, blob2_string, table_string
        )
        .into_bytes()
    }
}

#[derive(Clone, Copy)]
enum EscapeState {
    Base,
    Escape,
    Hex,
}
use std::str::from_utf8;

fn transform_escape(in_string: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut hex_val = Vec::new();
    let mut escape_state = EscapeState::Base;
    for c in in_string {
        match (escape_state, c) {
            //Start escape
            (EscapeState::Base, 0x5c) => {
                escape_state = EscapeState::Escape;
            }
            (EscapeState::Hex, 0x5c) => {
                let _: u8 = from_utf8(&hex_val).unwrap().parse().unwrap();
                escape_state = EscapeState::Escape;
            }

            //Basic escapes
            (EscapeState::Escape, 0x61) => {
                out.push(0x07);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x62) => {
                out.push(0x08);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x65) => {
                out.push(0x1b);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x66) => {
                out.push(0x0c);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x6e) => {
                out.push(0x0a);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x72) => {
                out.push(0x0d);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x74) => {
                out.push(0x09);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x76) => {
                out.push(0x0b);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x5c) => {
                out.push(0x5c);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x27) => {
                out.push(0x27);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x22) => {
                out.push(0x22);
                escape_state = EscapeState::Base;
            }
            (EscapeState::Escape, 0x3f) => {
                out.push(0x3f);
                escape_state = EscapeState::Base;
            }

            (EscapeState::Escape, 0x78) => {
                escape_state = EscapeState::Hex;
            }

            (EscapeState::Hex, c) if (0x30..0x39).contains(c) || (0x61..0x7a).contains(c) => {
                hex_val.push(*c);
            }

            // Not an escape sequence
            (EscapeState::Escape, _) => {
                panic!("Invalid escape sequence");
            }

            (EscapeState::Hex, c) => {
                escape_state = EscapeState::Base;
                out.push(c.clone());
            }
            // Just regular text
            (EscapeState::Base, c) => {
                out.push(c.clone());
            }
        }
    }
    return out;
}

#[cfg(test)]
mod test {
    use crate::StringStore;

    #[test]
    fn test_hex() {
        let foo: u8 = "ff".parse().unwrap();
        println!("{}", foo);
    }
    #[test]
    fn test_build_template() {
        let mut store = StringStore::new();
        let res = store.add_string("Hello world".as_bytes());
        println!("{:?}", res);
        println!("{:?}", store);
        println!("{:?}", store.build_template());
    }
}
