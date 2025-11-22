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
    env,
    fs,
    io,
    path::PathBuf
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = parse_args(&args);

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {            println!("{}", name);
        }
    }

    return Ok(());
}

fn parse_args(args: &[String]) -> PathBuf {
    let mut directory = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            _ if arg.starts_with('-') => {},
            _ => {
                if directory.is_none() {
                    directory = Some(PathBuf::from(arg));
                }
            }
        }
    }

    let directory = directory.unwrap_or_else(|| env::current_dir().unwrap());

    return directory;
}
