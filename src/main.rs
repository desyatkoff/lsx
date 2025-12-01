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

use chrono::{DateTime, Local};
use colored::Colorize;
use std::{
    cmp::Ordering,
    env,
    fs::{self, Metadata},
    io,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::PathBuf,
    time::SystemTime,
};
use users::get_user_by_uid;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (
        _all,
        _group_directories_first,
        _group_directories_last,
        _show_total,
        _show_permissions,
        _show_owner,
        _show_size,
        _show_date_modified,
        help,
        version,
        dir,
    ) = parse_args(&args);

    if help {
        print_help();
    }

    if version {
        print_version();
    }

    if !help && !version {
        let _ = list_dir_content(dir);
    }

    Ok(())
}

fn parse_args(
    args: &[String],
) -> (
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    PathBuf,
) {
    let mut all = false;
    let mut group_directories_first = false;
    let mut group_directories_last = false;
    let mut show_total = false;
    let mut show_permissions = false;
    let mut show_owner = false;
    let mut show_size = false;
    let mut show_date_modified = false;
    let mut help = false;
    let mut version = false;
    let mut directory = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-a" | "--all" => {
                all = true;
            }
            "--group-directories-first" => {
                group_directories_first = true;
            }
            "--group-directories-last" => {
                group_directories_last = true;
            }
            "--show-total" => {
                show_total = true;
            }
            "--show-permissions" => {
                show_permissions = true;
            }
            "--show-owner" => {
                show_owner = true;
            }
            "--show-size" => {
                show_size = true;
            }
            "--show-date-modified" => {
                show_date_modified = true;
            }
            "-h" | "--help" => {
                help = true;
            }
            "-V" | "--version" => {
                version = true;
            }
            _ if arg.starts_with('-') => {}
            _ => {
                if directory.is_none() {
                    directory = Some(PathBuf::from(arg));
                }
            }
        }
    }

    let directory = directory.unwrap_or_else(|| env::current_dir().unwrap());

    (
        all,
        group_directories_first,
        group_directories_last,
        show_total,
        show_permissions,
        show_owner,
        show_size,
        show_date_modified,
        help,
        version,
        directory,
    )
}

fn get_permissions_string(meta: &Metadata) -> String {
    fn bit(x: u32, bit: u32, c: char) -> char {
        if x & bit != 0 { c } else { '-' }
    }

    let mut s = String::new();

    let mode = meta.permissions().mode();
    let file_type = meta.file_type();
    let type_char = if file_type.is_block_device() {
        'b'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else {
        '-'
    };

    s.push(type_char);
    s.push(bit(mode, 0o400, 'r'));
    s.push(bit(mode, 0o200, 'w'));
    s.push(if mode & 0o4000 != 0 {
        if mode & 0o100 != 0 { 's' } else { 'S' }
    } else {
        bit(mode, 0o100, 'x')
    });
    s.push(bit(mode, 0o040, 'r'));
    s.push(bit(mode, 0o020, 'w'));
    s.push(if mode & 0o2000 != 0 {
        if mode & 0o010 != 0 { 's' } else { 'S' }
    } else {
        bit(mode, 0o010, 'x')
    });
    s.push(bit(mode, 0o004, 'r'));
    s.push(bit(mode, 0o002, 'w'));
    s.push(if mode & 0o1000 != 0 {
        if mode & 0o001 != 0 { 't' } else { 'T' }
    } else {
        bit(mode, 0o001, 'x')
    });

    s
}

fn get_owner(uid: u32) -> String {
    get_user_by_uid(uid)
        .unwrap()
        .name()
        .to_str()
        .unwrap()
        .to_string()
}

fn bytes_to_human_size(bytes: u64) -> String {
    const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if size < 10.0 {
        format!("{:.2} {}", size, UNITS[unit])
    } else if size < 100.0 {
        format!("{:.1} {}", size, UNITS[unit])
    } else {
        format!("{:.0} {}", size, UNITS[unit])
    }
}

fn system_time_to_human_time(time: SystemTime) -> String {
    DateTime::<Local>::from(time)
        .format("%d %b %H:%M")
        .to_string()
}

fn list_dir_content(dir: PathBuf) -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (
        all,
        group_directories_first,
        group_directories_last,
        show_total,
        show_permissions,
        show_owner,
        show_size,
        show_date_modified,
        _help,
        _version,
        _dir,
    ) = parse_args(&args);

    if let Ok(exists) = fs::exists(&dir) {
        if exists {
            let mut entries: Vec<_> = fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();

            if group_directories_first || group_directories_last {
                entries.sort_by(|a, b| {
                    let a_name = a.file_name();
                    let b_name = b.file_name();

                    let a_meta = a.metadata();
                    let b_meta = b.metadata();

                    let a_is_dir = a_meta.map(|m| m.is_dir()).unwrap_or(false);
                    let b_is_dir = b_meta.map(|m| m.is_dir()).unwrap_or(false);

                    if group_directories_first {
                        match (a_is_dir, b_is_dir) {
                            (true, false) => Ordering::Less,
                            (false, true) => Ordering::Greater,
                            _ => a_name.to_string_lossy().cmp(&b_name.to_string_lossy()),
                        }
                    } else {
                        match (a_is_dir, b_is_dir) {
                            (true, false) => Ordering::Greater,
                            (false, true) => Ordering::Less,
                            _ => a_name.to_string_lossy().cmp(&b_name.to_string_lossy()),
                        }
                    }
                });
            }

            let mut count = 0;

            for entry in entries {
                let permissions = get_permissions_string(&entry.metadata().unwrap());
                let owner = get_owner(entry.metadata().map(|m| m.uid()).unwrap());
                let size = bytes_to_human_size(entry.metadata().map(|m| m.size()).unwrap());
                let date_modified = system_time_to_human_time(
                    entry.metadata().map(|m| m.modified()).unwrap().unwrap(),
                );
                let name = entry.file_name().to_string_lossy().to_string();

                if all || !name.starts_with('.') {
                    if show_permissions {
                        print!("{} ", permissions)
                    }

                    if show_owner {
                        print!("{} ", owner);
                    }

                    if show_size {
                        print!("{} ", size);
                    }

                    if show_date_modified {
                        print!("{} ", date_modified);
                    }

                    println!("{}", name);

                    count += 1;
                }
            }

            if show_total {
                println!("-------{}", "-".repeat(count.to_string().len()));
                println!("Total: {}", count);
            }
        } else {
            eprintln!("{}: no such file or directory", "error".red().bold());
        }
    }

    Ok(())
}

fn print_help() {
    println!(
        r#"USAGE:
lsx [OPTIONS] [DIRECTORY]

OPTIONS:
    -a, --all                    Do not ignore entries starting with .
    --group-directories-first    List directories before other files
    --group-directories-last     List directories after other files
    --show-total                 Show total entries count
    --show-permissions           Show entry permissions
    --show-owner                 Show entry owner
    --show-size                  Show entry size
    --show-date-modified         Show last modified short date
    -h, --help                   Print help
    -V, --version                Print version"#
    );
}

fn print_version() {
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
