extern crate crossterm;
use crate::data::{
    // data_manager::{SubArgs, DataManager},
    json_manager::{self},
    model::DataModel,
};
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, BufRead, BufReader, Write};

pub enum MenuAction {
    Default,
    Scan,
    Pair,
    Unset,
}

pub fn navigate_menu(action: MenuAction) {
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut cursor_x: usize = 0;
    let mut cursor_y = 0;

    let mut stage_num = 0;
    let data_file = "./data.json";
    let mut menu = DataModel::parse_json();
    println!("{:?}", menu);
    let submenu: &[&str];

    let default_submenu = &["[s] SCAN", "[l] LINK", "[u] UNSET"];
    let pair_submenu = &menu
        .targets
        .iter()
        .map(|target| target.target_path.as_str())
        .collect::<Vec<&str>>();

    match action {
        MenuAction::Default => {
            submenu = default_submenu;
        }
        MenuAction::Pair => submenu = pair_submenu,

        _ => {
            submenu = &[""];
        }
    }

    loop {
        execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
        print_stage(&menu, stage_num, cursor_y, cursor_x, submenu);

        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if stage_num != 1 && cursor_y > 0 {
                        cursor_y -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if stage_num != 1 && cursor_y < menu.pairs.len() {
                        cursor_y += 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if cursor_x < submenu.len() - 1 {
                        cursor_x += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    }
                }
                KeyCode::Enter => {
                    if stage_num == submenu.len() - 1 - 1 {
                        break;
                    }
                    stage_num += 1;
                }
                KeyCode::Backspace => {
                    while stage_num > 0 {
                        stage_num -= 1;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    disable_raw_mode().expect("Failed to disable raw mode");
    println!("\r");
}

fn print_stage(
    menu: &DataModel,
    stage_num: usize,
    cursor_y: usize,
    cursor_x: usize,
    submenu: &[&str],
) {
    match stage_num {
        0 => print_menu(menu, cursor_y),
        1 => {
            println!("\r");
            println!("\x1B[33m{:?} \r", menu.pairs[cursor_y]);
            println!("\x1B[0m\n");

            print_submenu(submenu, cursor_x);
        }
        _ => {}
    }
}

fn print_menu(menu: &DataModel, cursor_y: usize) {
    for (i, source) in menu.pairs.iter().enumerate() {
        if i == cursor_y {
            println!("> {:?} \r", source);
        } else {
            println!("  {:?} \r", source);
        }
    }
}

fn print_submenu(submenu: &[&str], cursor_x: usize) {
    for (i, submenu_item) in submenu.iter().enumerate() {
        if i == cursor_x {
            print!("\x1B[4m{}\x1B[0m  ", submenu_item);
        } else {
            print!("{}  ", submenu_item);
        }
    }
    println!("\r");
}

pub fn get_yn_input() -> bool {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    match input {
        "y" | "Y" => {
            return true;
        }
        "n" | "N" => {
            return false;
        }
        _ => get_yn_input(),
    }
}
