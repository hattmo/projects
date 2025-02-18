#![feature(
    slice_pattern,
    ascii_char,
    ascii_char_variants,
    is_ascii_octdigit,
    iter_intersperse
)]
use core::slice::SlicePattern;
use std::{
    io::{Stdout, Write, stdout},
    process::ExitCode,
};

mod echo;
fn main() -> ExitCode {
    let args: Box<[_]> = std::env::args().into_iter().collect();
    let Some((command, args)) = get_subcommand(&args).or_else(|| {
        let [_, rest @ ..] = args.as_slice() else {
            return None;
        };
        get_subcommand(&rest)
    }) else {
        return ExitCode::FAILURE;
    };
    let stdout = stdout();
    command(&args, stdout).unwrap_or_else(|exit| exit)
}

fn get_subcommand(
    args: &[String],
) -> Option<(
    fn(&[String], stdout: Stdout) -> Result<ExitCode, ExitCode>,
    &[String],
)> {
    let [command, rest @ ..] = args else {
        return None;
    };
    match command.as_str() {
        "echo" => Some((echo::echo_main, rest)),
        _ => None,
    }
}
