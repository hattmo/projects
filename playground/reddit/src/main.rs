#![feature(tcplistener_into_incoming)]
#![feature(result_option_inspect)]
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() {
    TcpListener::bind("0.0.0.0:8080")
        .or(Err("Failed to bind"))
        .unwrap()
        .into_incoming()
        .filter_map(Result::ok)
        .map(|mut conn| {
            conn.write_all(
                &b"HTTP/1.1 200 OK\nContent-Type: text/plain\n\n"
                    .iter()
                    .chain(
                        BufReader::new(conn.try_clone().unwrap())
                            .lines()
                            .next()
                            .map(|inner| inner.map_err(|_| "Failed to parse request".to_owned()))
                            .unwrap_or(Err("Not valid http".to_owned()))
                            .and_then(|line| {
                                if line.len() > 40 {
                                    Err("ARE YOU FUZZING ME".to_owned())
                                } else {
                                    Ok(line)
                                }
                            })
                            .and_then(|line| {
                                line.split(' ')
                                    .nth(1)
                                    .map(ToOwned::to_owned)
                                    .ok_or("No path".to_string())
                            })
                            .and_then(|url| {
                                url.split('?')
                                    .nth(1)
                                    .map(ToOwned::to_owned)
                                    .ok_or("No query".to_string())
                            })
                            .and_then(|query| {
                                query
                                    .split('&')
                                    .find(|field| field.starts_with("text"))
                                    .map(ToOwned::to_owned)
                                    .ok_or("No text query".to_string())
                            })
                            .and_then(|field| {
                                field
                                    .split('=')
                                    .nth(1)
                                    .map(|query| query.replace("%20", " "))
                                    .ok_or("No text query value".to_string())
                            })
                            .unwrap_or_else(|e| e)
                            .chars()
                            .collect::<Vec<char>>()
                            .chunks(16)
                            .filter_map(|chunk| {
                                chunk
                                    .iter()
                                    .filter(|c| c.is_ascii_alphabetic() || **c == ' ')
                                    .cloned()
                                    .flat_map(char::to_lowercase)
                                    .map(|c| match c {
                                        'a' => [
                                            "  ##   ", " #  #  ", "#    # ", "###### ", "#    # ",
                                            "#    # ",
                                        ],
                                        'b' => [
                                            "#####  ", "#    # ", "#####  ", "#    # ", "#    # ",
                                            "#####  ",
                                        ],
                                        'c' => [
                                            " ####  ", "#    # ", "#      ", "#      ", "#    # ",
                                            " ####  ",
                                        ],
                                        'd' => [
                                            "#####  ", "#    # ", "#    # ", "#    # ", "#    # ",
                                            "#####  ",
                                        ],
                                        'e' => [
                                            "###### ", "#      ", "#####  ", "#      ", "#      ",
                                            "###### ",
                                        ],
                                        'f' => [
                                            "###### ", "#      ", "#####  ", "#      ", "#      ",
                                            "#      ",
                                        ],
                                        'g' => [
                                            " ####  ", "#    # ", "#      ", "#  ### ", "#    # ",
                                            " ####  ",
                                        ],
                                        'h' => [
                                            "#    # ", "#    # ", "###### ", "#    # ", "#    # ",
                                            "#    # ",
                                        ],
                                        'i' => [
                                            "###### ", "  ##   ", "  ##   ", "  ##   ", "  ##   ",
                                            "###### ",
                                        ],
                                        'j' => [
                                            "     # ", "     # ", "     # ", "     # ", "#    # ",
                                            " ####  ",
                                        ],
                                        'k' => [
                                            "#    # ", "#   #  ", "####   ", "#  #   ", "#   #  ",
                                            "#    # ",
                                        ],
                                        'l' => [
                                            "#      ", "#      ", "#      ", "#      ", "#      ",
                                            "###### ",
                                        ],
                                        'm' => [
                                            "#    # ", "##  ## ", "# ## # ", "#    # ", "#    # ",
                                            "#    # ",
                                        ],
                                        'n' => [
                                            "#    # ", "##   # ", "# #  # ", "#  # # ", "#   ## ",
                                            "#    # ",
                                        ],
                                        'o' => [
                                            " ####  ", "#    # ", "#    # ", "#    # ", "#    # ",
                                            " ####  ",
                                        ],
                                        'p' => [
                                            "#####  ", "#    # ", "#    # ", "#####  ", "#      ",
                                            "#      ",
                                        ],
                                        'q' => [
                                            " ####  ", "#    # ", "#    # ", "#  # # ", "#   #  ",
                                            " ### # ",
                                        ],
                                        'r' => [
                                            "#####  ", "#    # ", "#    # ", "#####  ", "#   #  ",
                                            "#    # ",
                                        ],
                                        's' => [
                                            " ####  ", "#      ", " ####  ", "     # ", "#    # ",
                                            " ####  ",
                                        ],
                                        't' => [
                                            "###### ", "  ##   ", "  ##   ", "  ##   ", "  ##   ",
                                            "  ##   ",
                                        ],
                                        'u' => [
                                            "#    # ", "#    # ", "#    # ", "#    # ", "#    # ",
                                            " ####  ",
                                        ],
                                        'v' => [
                                            "#    # ", "#    # ", "#    # ", "#    # ", " #  #  ",
                                            "  ##   ",
                                        ],
                                        'w' => [
                                            "#    # ", "#    # ", "#    # ", "# ## # ", "##  ## ",
                                            "#    # ",
                                        ],
                                        'x' => [
                                            "#    # ", " #  #  ", "  ##   ", "  ##   ", " #  #  ",
                                            "#    # ",
                                        ],
                                        'y' => [
                                            "#    # ", " #  #  ", "  ##   ", "  ##   ", "  ##   ",
                                            "  ##   ",
                                        ],
                                        'z' => [
                                            "###### ", "    #  ", "   #   ", "  #    ", " #     ",
                                            "###### ",
                                        ],
                                        ' ' => [
                                            "       ", "       ", "       ", "       ", "       ",
                                            "       ",
                                        ],
                                        _ => panic!(),
                                    })
                                    .chain([["\n", "\n", "\n", "\n", "\n", "\n"]].iter().cloned())
                                    .map(|i| {
                                        i.into_iter()
                                            .map(|i| i.to_string())
                                            .collect::<Vec<String>>()
                                    })
                                    .reduce(|coll, next| {
                                        coll.into_iter()
                                            .zip(next)
                                            .map(|(coll, next)| format!("{}{}", coll, next))
                                            .collect()
                                    })
                            })
                            .map(|i| i.into_iter().collect::<String>())
                            .collect::<String>()
                            .as_bytes(),
                    )
                    .copied()
                    .collect::<Vec<u8>>(),
            )
        })
        .for_each(|_| {});
}
