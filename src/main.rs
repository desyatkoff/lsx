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
use clap::Parser;
use colored::Colorize;
use serde::Serialize;
use std::{
    cmp::Ordering,
    env,
    fs::{self, Metadata},
    io::Result,
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

/// CLI options and arguments
#[derive(Debug, Parser)]
#[command(name = "LSX", about = "Imagine ls command, but better", version)]
struct Cli {
    /// Do not ignore entries starting with `.`
    #[arg(short = 'a', long = "all")]
    all: bool,
    /// List directories before other files
    #[arg(
        long = "group-directories-first",
        conflicts_with = "group_directories_last"
    )]
    group_directories_first: bool,
    /// List directories after other files
    #[arg(
        long = "group-directories-last",
        conflicts_with = "group_directories_first"
    )]
    group_directories_last: bool,
    /// Enable every `--show-*` option below
    #[arg(long = "show-all-columns")]
    show_all_columns: bool,
    /// Show entry permissions column
    #[arg(long = "show-permissions")]
    show_permissions: bool,
    /// Show entry owner column
    #[arg(long = "show-owner")]
    show_owner: bool,
    /// Show entry size column
    #[arg(long = "show-size")]
    show_size: bool,
    /// Show entry date modified column
    #[arg(long = "show-date-modified")]
    show_date_modified: bool,
    /// Show total entries count
    #[arg(long = "show-total")]
    show_total: bool,
    /// Colorize output
    #[arg(short = 'c', long = "colors")]
    colors: bool,
    /// Use table view
    #[arg(short = 't', long = "table", conflicts_with = "json")]
    table: bool,
    /// JSON output
    #[arg(short = 'j', long = "json", conflicts_with = "table")]
    json: bool,
    /// Directory to list items from
    #[arg(value_name = "PATH", value_parser = clap::value_parser!(PathBuf))]
    directory: Option<PathBuf>,
}

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
fn main() -> Result<()> {
    let args = Cli::parse();

    let mut entries_array = Vec::new();
    let mut entries_human_array = Vec::new();

    if let Ok(exists) = fs::exists(
        &args
            .directory
            .clone()
            .unwrap_or_else(|| env::current_dir().unwrap()),
    ) {
        if exists {
            let mut entries: Vec<_> = fs::read_dir(
                &args
                    .directory
                    .clone()
                    .unwrap_or_else(|| env::current_dir().unwrap()),
            )
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

            if args.group_directories_first || args.group_directories_last {
                entries.sort_by(|a, b| {
                    let a_name = a.file_name();
                    let b_name = b.file_name();

                    let a_meta = a.metadata();
                    let b_meta = b.metadata();

                    let a_is_dir = a_meta.map(|m| m.is_dir()).unwrap_or(false);
                    let b_is_dir = b_meta.map(|m| m.is_dir()).unwrap_or(false);

                    if args.group_directories_first {
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

                if args.all || !name.starts_with('.') {
                    if !args.table && !args.json {
                        if args.colors {
                            if args.show_all_columns || args.show_permissions {
                                print!("{} ", permissions_human.red());
                            }

                            if args.show_all_columns || args.show_owner {
                                print!("{} ", owner_human.green());
                            }

                            if args.show_all_columns || args.show_size {
                                print!("{} ", size_human.yellow());
                            }

                            if args.show_all_columns || args.show_date_modified {
                                print!("{} ", date_modified_human.magenta());
                            }

                            println!("{}", name.cyan());
                        } else {
                            if args.show_all_columns || args.show_permissions {
                                print!("{permissions_human} ");
                            }

                            if args.show_all_columns || args.show_owner {
                                print!("{owner_human} ");
                            }

                            if args.show_all_columns || args.show_size {
                                print!("{size_human} ");
                            }

                            if args.show_all_columns || args.show_date_modified {
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

            if args.table && !args.json && !entries_array.is_empty() {
                let mut table_instance = Table::new(&entries_human_array);

                table_instance.with(Style::rounded());

                if args.colors {
                    table_instance.modify(Columns::one(0), Color::FG_RED);
                    table_instance.modify(Columns::one(1), Color::FG_GREEN);
                    table_instance.modify(Columns::one(2), Color::FG_YELLOW);
                    table_instance.modify(Columns::one(3), Color::FG_MAGENTA);
                    table_instance.modify(Columns::one(4), Color::FG_CYAN);
                    table_instance.modify(Rows::first(), Color::FG_BLUE);
                }

                if !args.show_all_columns && !args.show_permissions {
                    table_instance.with(Remove::column(ByColumnName::new("Permissions")));
                }

                if !args.show_all_columns && !args.show_owner {
                    table_instance.with(Remove::column(ByColumnName::new("Owner")));
                }

                if !args.show_all_columns && !args.show_size {
                    table_instance.with(Remove::column(ByColumnName::new("Size")));
                }

                if !args.show_all_columns && !args.show_date_modified {
                    table_instance.with(Remove::column(ByColumnName::new("Date Modified")));
                }

                println!("{table_instance}");
            }

            if args.json && !args.table {
                println!("{}", serde_json::to_string_pretty(&entries_array).unwrap());
            }

            if (!args.table && !args.json) && (args.show_all_columns || args.show_total) {
                println!("-------{}", "-".repeat(count.to_string().len()));
                println!("Total: {count}");
            }
        } else {
            eprintln!("{}: no such file or directory", "error".red().bold());
        }
    }

    Ok(())
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
