use std::fs;
use std::path::Path;

use ggez::input::keyboard::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    // I hope there is a better way to do this.
    #[serde(default = "light_sqaure_color_default")]
    pub light_square_color: (u8, u8, u8),

    #[serde(default = "dark_sqaure_color_default")]
    pub dark_square_color: (u8, u8, u8),

    #[serde(default = "selected_piece_color_default")]
    pub selected_piece_color: (u8, u8, u8),

    #[serde(default = "possible_moves_color_default")]
    pub possible_moves_color: (u8, u8, u8),

    #[serde(default = "possible_captures_color_default")]
    pub possible_captures_color: (u8, u8, u8),

    #[serde(default = "last_move_start_color_default")]
    pub last_move_start_color: (u8, u8, u8),

    #[serde(default = "last_move_end_color_default")]
    pub last_move_end_color: (u8, u8, u8),

    #[serde(default = "check_color_default")]
    pub check_color: (u8, u8, u8),

    #[serde(default = "checkmate_color_default")]
    pub checkmate_color: (u8, u8, u8),

    #[serde(default = "tie_color_default")]
    pub tie_color: (u8, u8, u8),

    #[serde(default = "font_color_default")]
    pub font_color: (u8, u8, u8),

    #[serde(default = "draw_color_default")]
    pub draw_color: (u8, u8, u8),

    #[serde(default = "font_size_default")]
    pub font_size: f32,

    #[serde(default = "draw_thickness_default")]
    pub draw_thickness: f32,

    #[serde(default = "queen_upgrade_button_default")]
    pub queen_upgrade_button: String,

    #[serde(default = "rook_upgrade_button_default")]
    pub rook_upgrade_button: String,

    #[serde(default = "bishop_upgrade_button_default")]
    pub bishop_upgrade_button: String,

    #[serde(default = "knight_upgrade_button_default")]
    pub knight_upgrade_button: String,

    #[serde(default = "draw_clear_button_default")]
    pub draw_clear_button: String,

    #[serde(default = "window_size_horizontal_default")]
    pub window_size_horizontal: f32,

    #[serde(default = "window_size_vertical_default")]
    pub window_size_vertical: f32,

    #[serde(default = "volume_default")]
    pub volume: f32,
}

fn light_sqaure_color_default() -> (u8, u8, u8) {
    return (232, 206, 162);
}

fn dark_sqaure_color_default() -> (u8, u8, u8) {
    return (153, 100, 33);
}

fn selected_piece_color_default() -> (u8, u8, u8) {
    return (0, 128, 128);
}

fn possible_moves_color_default() -> (u8, u8, u8) {
    return (0, 255, 255);
}

fn possible_captures_color_default() -> (u8, u8, u8) {
    return (255, 0, 255);
}

fn last_move_start_color_default() -> (u8, u8, u8) {
    return (0, 150, 30);
}

fn last_move_end_color_default() -> (u8, u8, u8) {
    return (0, 250, 50);
}

fn check_color_default() -> (u8, u8, u8) {
    return (255, 0, 0);
}

fn checkmate_color_default() -> (u8, u8, u8) {
    return (169, 0, 0);
}

fn tie_color_default() -> (u8, u8, u8) {
    return (0, 0, 169);
}

fn font_color_default() -> (u8, u8, u8) {
    return (60, 60, 60);
}

fn draw_color_default() -> (u8, u8, u8) {
    return (255, 211, 69);
}

fn font_size_default() -> f32 {
    return 12.0;
}

fn draw_thickness_default() -> f32 {
    return 5.0;
}

fn queen_upgrade_button_default() -> String {
    return "Q".to_string();
}

fn rook_upgrade_button_default() -> String {
    return "R".to_string();
}

fn bishop_upgrade_button_default() -> String {
    return "B".to_string();
}

fn knight_upgrade_button_default() -> String {
    return "N".to_string();
}

fn draw_clear_button_default() -> String {
    return "Space".to_string();
}

fn window_size_horizontal_default() -> f32 {
    return 480f32;
}

fn window_size_vertical_default() -> f32 {
    return 480f32;
}

fn volume_default() -> f32 {
    return 0.5f32;
}

impl Default for Config {
    fn default() -> Self {
        Config {
            light_square_color: light_sqaure_color_default(),
            dark_square_color: dark_sqaure_color_default(),
            selected_piece_color: selected_piece_color_default(),
            possible_moves_color: possible_captures_color_default(),
            possible_captures_color: possible_captures_color_default(),
            last_move_start_color: last_move_start_color_default(),
            last_move_end_color: last_move_end_color_default(),
            check_color: check_color_default(),
            checkmate_color: checkmate_color_default(),
            tie_color: tie_color_default(),
            font_color: font_color_default(),
            draw_color: draw_color_default(),
            font_size: font_size_default(),
            draw_thickness: draw_thickness_default(),
            queen_upgrade_button: queen_upgrade_button_default(),
            rook_upgrade_button: rook_upgrade_button_default(),
            bishop_upgrade_button: bishop_upgrade_button_default(),
            knight_upgrade_button: knight_upgrade_button_default(),
            draw_clear_button: draw_clear_button_default(),
            window_size_horizontal: window_size_horizontal_default(),
            window_size_vertical: window_size_vertical_default(),
            volume: volume_default(),
        }
    }
}

/// Loads the config from the config.json file.
pub fn load_config() -> Config {
    if !Path::new("./config.json").exists() {
        println!(
            "Create a config.json file to configure this game. Using default settings for now..."
        );

        return Config::default();
    }

    let json_file = fs::read_to_string("./config.json").unwrap();

    let c: Config = serde_json::from_str(&json_file).unwrap();

    return Config {
        light_square_color: c.light_square_color,
        dark_square_color: c.dark_square_color,
        selected_piece_color: c.selected_piece_color,
        possible_moves_color: c.possible_moves_color,
        possible_captures_color: c.possible_captures_color,
        last_move_start_color: c.last_move_start_color,
        last_move_end_color: c.last_move_end_color,
        check_color: c.check_color,
        checkmate_color: c.checkmate_color,
        tie_color: c.tie_color,
        font_color: c.font_color,
        draw_color: c.draw_color,
        font_size: c.font_size,
        draw_thickness: c.draw_thickness,
        queen_upgrade_button: c.queen_upgrade_button,
        rook_upgrade_button: c.rook_upgrade_button,
        bishop_upgrade_button: c.bishop_upgrade_button,
        knight_upgrade_button: c.knight_upgrade_button,
        draw_clear_button: c.draw_clear_button,
        window_size_horizontal: c.window_size_horizontal,
        window_size_vertical: c.window_size_vertical,
        volume: c.volume,
    };
}

/// Matches the string to the actual KeyCode,
/// cannot really Deserialize a KeyCode, so the extra step is needed.
pub fn match_key(input: String) -> Option<KeyCode> {
    match input.to_ascii_uppercase().as_str() {
        "A" => Some(KeyCode::A),
        "B" => Some(KeyCode::B),
        "C" => Some(KeyCode::C),
        "D" => Some(KeyCode::D),
        "E" => Some(KeyCode::E),
        "F" => Some(KeyCode::F),
        "G" => Some(KeyCode::G),
        "H" => Some(KeyCode::H),
        "I" => Some(KeyCode::I),
        "J" => Some(KeyCode::J),
        "K" => Some(KeyCode::K),
        "L" => Some(KeyCode::L),
        "M" => Some(KeyCode::M),
        "N" => Some(KeyCode::N),
        "O" => Some(KeyCode::O),
        "P" => Some(KeyCode::P),
        "Q" => Some(KeyCode::Q),
        "R" => Some(KeyCode::R),
        "S" => Some(KeyCode::S),
        "T" => Some(KeyCode::T),
        "U" => Some(KeyCode::U),
        "V" => Some(KeyCode::V),
        "W" => Some(KeyCode::W),
        "X" => Some(KeyCode::X),
        "Y" => Some(KeyCode::Y),
        "Z" => Some(KeyCode::Z),
        "1" => Some(KeyCode::Key1),
        "2" => Some(KeyCode::Key2),
        "3" => Some(KeyCode::Key3),
        "4" => Some(KeyCode::Key4),
        "5" => Some(KeyCode::Key5),
        "6" => Some(KeyCode::Key6),
        "7" => Some(KeyCode::Key7),
        "8" => Some(KeyCode::Key8),
        "9" => Some(KeyCode::Key9),
        "0" => Some(KeyCode::Key0),
        "F1" => Some(KeyCode::F1),
        "F2" => Some(KeyCode::F2),
        "F3" => Some(KeyCode::F3),
        "F4" => Some(KeyCode::F4),
        "F5" => Some(KeyCode::F5),
        "F6" => Some(KeyCode::F6),
        "F7" => Some(KeyCode::F7),
        "F8" => Some(KeyCode::F8),
        "F9" => Some(KeyCode::F9),
        "F10" => Some(KeyCode::F10),
        "F11" => Some(KeyCode::F11),
        "F12" => Some(KeyCode::F12),
        "SPACE" => Some(KeyCode::Space),
        "SPACEBAR" => Some(KeyCode::Space),
        " " => Some(KeyCode::Space),
        "ESC" => Some(KeyCode::Escape),
        "ESCAPE" => Some(KeyCode::Escape),
        "ENTER" => Some(KeyCode::Return),
        "UP" => Some(KeyCode::Up),
        "DOWN" => Some(KeyCode::Down),
        "LEFT" => Some(KeyCode::Left),
        "RIGHT" => Some(KeyCode::Right),
        "PLUS" => Some(KeyCode::Plus),
        "+" => Some(KeyCode::Plus),
        "MINUS" => Some(KeyCode::Minus),
        "-" => Some(KeyCode::Minus),
        "TAB" => Some(KeyCode::Tab),
        "CAPS" => Some(KeyCode::Capital),
        "SHIFT" => Some(KeyCode::LShift),
        "CTRL" => Some(KeyCode::LControl),
        "CONTROL" => Some(KeyCode::LControl),
        "ALT" => Some(KeyCode::LAlt),
        "NUM0" => Some(KeyCode::Numpad0),
        "NUM1" => Some(KeyCode::Numpad1),
        "NUM2" => Some(KeyCode::Numpad2),
        "NUM3" => Some(KeyCode::Numpad3),
        "NUM4" => Some(KeyCode::Numpad4),
        "NUM5" => Some(KeyCode::Numpad5),
        "NUM6" => Some(KeyCode::Numpad6),
        "NUM7" => Some(KeyCode::Numpad7),
        "NUM8" => Some(KeyCode::Numpad8),
        "NUM9" => Some(KeyCode::Numpad9),
        "NUMENTER" => Some(KeyCode::NumpadEnter),

        _ => None,
    }
}
