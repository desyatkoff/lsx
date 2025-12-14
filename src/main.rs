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
use serde::Serialize;
use std::{
    cmp::Ordering,
    env,
    fs::{self, Metadata},
    io,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tabled::{
    Table, Tabled,
    settings::{
        Color, Remove, Style,
        location::ByColumnName,
        object::{Columns, Rows},
    },
};
use users::get_user_by_uid;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Contains data for JSON output
#[derive(Serialize)]
struct Entry {
    permissions: String,
    owner_uid: u32,
    owner: String,
    size_in_bytes: u64,
    date_modified: u64,
    name: String,
}

/// Contains data for table view output
#[derive(Tabled)]
struct EntryHuman {
    #[tabled{rename="Permissions"}]
    permissions: String,
    #[tabled{rename="Owner"}]
    owner: String,
    #[tabled{rename="Size"}]
    size: String,
    #[tabled{rename="Date Modified"}]
    date_modified: String,
    #[tabled{rename="Name"}]
    name: String,
}

/// Entry point
fn main() {
    let args: Vec<String> = env::args().collect();
    let (
        _all,
        _group_directories_first,
        _group_directories_last,
        _show_all_columns,
        _show_permissions,
        _show_owner,
        _show_size,
        _show_date_modified,
        _show_total,
        _colors,
        _table,
        _json,
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
}

/// Parses arguments/options from CLI
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
    bool,
    bool,
    bool,
    bool,
    PathBuf,
) {
    let mut all = false;
    let mut group_directories_first = false;
    let mut group_directories_last = false;
    let mut show_all_columns = false;
    let mut show_permissions = false;
    let mut show_owner = false;
    let mut show_size = false;
    let mut show_date_modified = false;
    let mut show_total = false;
    let mut colors = false;
    let mut table = false;
    let mut json = false;
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
            "--show-all-columns" => {
                show_all_columns = true;
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
            "--show-total" => {
                show_total = true;
            }
            "-c" | "--colors" => {
                colors = true;
            }
            "-t" | "--table" => {
                table = true;
            }
            "-j" | "--json" => {
                json = true;
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
        show_all_columns,
        show_permissions,
        show_owner,
        show_size,
        show_date_modified,
        show_total,
        colors,
        table,
        json,
        help,
        version,
        directory,
    )
}

/// Creates a permissions string from entry metadata
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

/// Parses owner username by UID
fn get_owner(uid: u32) -> String {
    get_user_by_uid(uid)
        .unwrap()
        .name()
        .to_str()
        .unwrap()
        .to_string()
}

/// Converts bytes to human-readable `String`
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

/// Converts `SystemTime` to human-readable `String`
fn system_time_to_human_time(time: SystemTime) -> String {
    DateTime::<Local>::from(time)
        .format("%d %b %H:%M")
        .to_string()
}

/// Prints the entries from specified directory
fn list_dir_content(dir: PathBuf) -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (
        all,
        group_directories_first,
        group_directories_last,
        show_all_columns,
        show_permissions,
        show_owner,
        show_size,
        show_date_modified,
        show_total,
        colors,
        table,
        json,
        _help,
        _version,
        _dir,
    ) = parse_args(&args);

    let mut entries_array = Vec::new();
    let mut entries_human_array = Vec::new();

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
                let permissions_human = get_permissions_string(&entry.metadata().unwrap());
                let owner_uid = entry.metadata().map(|m| m.uid()).unwrap();
                let owner_human = get_owner(owner_uid);
                let size_in_bytes = entry.metadata().map(|m| m.size()).unwrap();
                let size_human = bytes_to_human_size(size_in_bytes);
                let date_modified = entry
                    .metadata()
                    .map(|m| m.modified())
                    .unwrap()
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let date_modified_human = system_time_to_human_time(
                    entry.metadata().map(|m| m.modified()).unwrap().unwrap(),
                );
                let name = entry.file_name().to_string_lossy().to_string();

                if all || !name.starts_with('.') {
                    if !table && !json {
                        if colors {
                            if show_all_columns || show_permissions {
                                print!("{} ", permissions_human.red());
                            }

                            if show_all_columns || show_owner {
                                print!("{} ", owner_human.green());
                            }

                            if show_all_columns || show_size {
                                print!("{} ", size_human.yellow());
                            }

                            if show_all_columns || show_date_modified {
                                print!("{} ", date_modified_human.magenta());
                            }

                            println!("{}", name.cyan());
                        } else {
                            if show_all_columns || show_permissions {
                                print!("{permissions_human} ");
                            }

                            if show_all_columns || show_owner {
                                print!("{owner_human} ");
                            }

                            if show_all_columns || show_size {
                                print!("{size_human} ");
                            }

                            if show_all_columns || show_date_modified {
                                print!("{date_modified_human} ");
                            }

                            println!("{name}");
                        }
                    }

                    entries_array.push(Entry {
                        permissions: String::from(&permissions_human),
                        owner_uid: owner_uid,
                        owner: String::from(&owner_human),
                        size_in_bytes: size_in_bytes,
                        date_modified: date_modified,
                        name: String::from(&name),
                    });

                    entries_human_array.push(EntryHuman {
                        permissions: String::from(&permissions_human),
                        owner: String::from(&owner_human),
                        size: size_human,
                        date_modified: date_modified_human,
                        name: String::from(&name),
                    });

                    count += 1;
                }
            }

            if table && !json && entries_array.is_empty() {
                let mut table_instance = Table::new(&entries_human_array);

                table_instance.with(Style::rounded());

                if colors {
                    table_instance.modify(Columns::one(0), Color::FG_RED);
                    table_instance.modify(Columns::one(1), Color::FG_GREEN);
                    table_instance.modify(Columns::one(2), Color::FG_YELLOW);
                    table_instance.modify(Columns::one(3), Color::FG_MAGENTA);
                    table_instance.modify(Columns::one(4), Color::FG_CYAN);
                    table_instance.modify(Rows::first(), Color::FG_BLUE);
                }

                if !show_all_columns && !show_permissions {
                    table_instance.with(Remove::column(ByColumnName::new("Permissions")));
                }

                if !show_all_columns && !show_owner {
                    table_instance.with(Remove::column(ByColumnName::new("Owner")));
                }

                if !show_all_columns && !show_size {
                    table_instance.with(Remove::column(ByColumnName::new("Size")));
                }

                if !show_all_columns && !show_date_modified {
                    table_instance.with(Remove::column(ByColumnName::new("Date Modified")));
                }

                println!("{table_instance}");
            }

            if json && !table {
                println!("{}", serde_json::to_string_pretty(&entries_array).unwrap());
            }

            if (!table && !json) && (show_all_columns || show_total) {
                println!("-------{}", "-".repeat(count.to_string().len()));
                println!("Total: {count}");
            }
        } else {
            eprintln!("{}: no such file or directory", "error".red().bold());
        }
    }

    Ok(())
}

/// `--help`
fn print_help() {
    println!(
        r#"USAGE:
lsx [OPTIONS] [DIRECTORY]

OPTIONS:
    -a, --all                    Do not ignore entries starting with .
    --group-directories-first    List directories before other files
    --group-directories-last     List directories after other files
    --show-all-columns           Enable every --show-* option below
    --show-permissions           Show entry permissions column
    --show-owner                 Show entry owner column
    --show-size                  Show entry size column
    --show-date-modified         Show entry date modified column
    --show-total                 Show total entries count
    -c, --colors                 Colorize output
    -t, --table                  Use table view
    -j, --json                   JSON output
    -h, --help                   Print help
    -V, --version                Print version"#
    );
}

/// `--version`
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
