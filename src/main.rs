/*
 * This file is part of LSX
 *
 * Copyright (C) 2025 Sergey Desyatkov
 *
 * LSX is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License,
 * or (at your option) any later version
 *
 * LSX is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details
 *
 * You should have received a copy of the GNU General Public License
 * along with LSX. If not, see <https://www.gnu.org/licenses/>
 */

use std::{
    cmp::Ordering,
    env,
    fs,
    io,
    path::PathBuf
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (all, group_directories_first, help, version, dir) = parse_args(&args);

    if help {
        println!(
            r#"USAGE:
    lsx [OPTIONS] [DIRECTORY]

OPTIONS:
    -a, --all                    Do not ignore entries starting with .
    --group-directories-first    List directories before other files
    -h, --help                   Print help
    -V, --version                Print version"#
        );
    }

    if version {
        println!(
            r#" _     ______  __
| |   / ___\ \/ /
| |   \___ \\  / 
| |___ ___) /  \ 
|_____|____/_/\_\

LSX v{}
Imagine ls command, but better

Copyright (C) 2025 Sergey Desyatkov
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version"#,
            VERSION
        );
    }

    if !help && !version {
        let mut entries: Vec<_> = fs::read_dir(&dir)?
            .filter_map(|e| e.ok())
            .collect();

        if group_directories_first {
            entries.sort_by(|a, b| {
                let a_name = a.file_name();
                let b_name = b.file_name();

                let a_meta = a.metadata();
                let b_meta = b.metadata();

                let a_is_dir = a_meta
                    .map(|m| m.is_dir())
                    .unwrap_or(false);
                let b_is_dir = b_meta
                    .map(|m| m.is_dir())
                    .unwrap_or(false);

                match (a_is_dir, b_is_dir) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    _ => {
                        return a_name.to_string_lossy().cmp(&b_name.to_string_lossy());                    }
                }
            });
        }

        for entry in entries {
            let name = entry
                .file_name()
                .to_string_lossy()
                .to_string();

            if all || !name.starts_with('.') {
                println!("{}", name);
            }
        }
    }

    return Ok(());
}

fn parse_args(args: &[String]) -> (bool, bool, bool, bool, PathBuf) {
    let mut all = false;
    let mut group_directories_first = false;
    let mut help = false;
    let mut version = false;
    let mut directory = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-a" | "--all" => {
                all = true;
            },
            "--group-directories-first" => {
                group_directories_first = true;
            },
            "-h" | "--help" => {
                help = true;
            },
            "-V" | "--version" => {
                version = true;
            },
            _ if arg.starts_with('-') => {},
            _ => {
                if directory.is_none() {
                    directory = Some(PathBuf::from(arg));
                }
            }
        }
    }

    let directory = directory.unwrap_or_else(|| env::current_dir().unwrap());

    return (all, group_directories_first, help, version, directory);
}
