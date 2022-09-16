extern "C" {
    fn tree_sitter_c() -> Language;
}
use anyhow::{anyhow, Ok, Result};
use clap::Parser as ClapParser;
use std::fs::{read, write};
use tree_sitter::{Language, Parser};
use tree_sitter_traversal::Order;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    input: String,

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
        let in_string = transform_escape(in_string).unwrap().1;
        self.count += 1;
        for ref c in in_string {
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

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, map_res, value},
    multi::many0,
    sequence::preceded,
    IResult,
};

fn transform_escape(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    all_consuming(many0(alt((
        preceded(
            tag(b"\\"),
            alt((
                value(0x07, tag(b"a")),
                value(0x08, tag(b"b")),
                value(0x1b, tag(b"e")),
                value(0x0c, tag(b"f")),
                value(b'\n', tag(b"n")),
                value(b'\r', tag(b"r")),
                value(b'\t', tag(b"t")),
                value(0x0b, tag(b"v")),
                value(b'\\', tag(b"\\")),
                value(b'\'', tag(b"'")),
                value(b'"', tag(b"\"")),
                map_res(preceded(tag(b"x"), take(2usize)), |hex: &[u8]| {
                    u8::from_str_radix(
                        &String::from_utf8(hex.to_vec()).or(Err(anyhow!("Not utf8")))?,
                        16,
                    )
                    .or(Err(anyhow!("Not u8 byte")))
                }),
            )),
        ),
        map(take(1usize), |foo: &[u8]| foo[0]),
    ))))(input)
}

#[cfg(test)]
mod test {
    use crate::{transform_escape, StringStore};

    #[test]
    fn test_parse() {
        let foo = b"\\x77\\b";
        let (_, parsed) = transform_escape(foo).unwrap();
        println!("{}", String::from_utf8(foo.to_vec()).unwrap());
        println!("{}", String::from_utf8(parsed).unwrap());
    }

    #[test]
    fn test_hex() {
        let foo = u8::from_str_radix("ff", 16).unwrap();
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
