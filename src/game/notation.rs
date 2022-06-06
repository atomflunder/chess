use std::io::{self, Write};

use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::Context;

use super::check;
use super::config::match_key;
use super::game::MainGame;
use super::player::Player;

/// Prints the moves to the console in the standard chess notation.
pub fn print_move(
    ctx: &Context,
    game: MainGame,
    piece: char,
    piece_pos: (usize, usize),
    possible_moves: Vec<(usize, usize)>,
    end_pos: (usize, usize),
    en_passant: Option<usize>,
    castling: (bool, bool, bool, bool),
    turn_count: usize,
) {
    let end_column = match end_pos.1 {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => ' ',
    };

    let end_row = match end_pos.0 {
        0 => '8',
        1 => '7',
        2 => '6',
        3 => '5',
        4 => '4',
        5 => '3',
        6 => '2',
        7 => '1',
        _ => ' ',
    };

    let piece_print = match piece.to_ascii_lowercase() {
        'p' => '\u{200b}',
        'b' => 'B',
        'n' => 'N',
        'r' => 'R',
        'q' => 'Q',
        'k' => 'K',
        _ => ' ',
    };

    let mut check_symbol = '\u{200b}';
    let mut capture_symbol = "\u{200b}".to_string();
    let mut promotion_symbol = "\u{200b}";

    let mut temp_game = game.clone();

    temp_game.make_move(ctx, piece, piece_pos, possible_moves, end_pos.0, end_pos.1);

    if check::check(temp_game.board, Player::White, en_passant, castling) == true
        || check::check(temp_game.board, Player::Black, en_passant, castling) == true
    {
        check_symbol = '+';
    }

    if check::player_checkmate(
        temp_game.board,
        en_passant,
        castling,
        temp_game.clone(),
        ctx,
    ) != None
    {
        check_symbol = '#';
    }

    if check::is_tie(
        temp_game.board,
        en_passant,
        castling,
        temp_game.clone(),
        ctx,
    ) {
        check_symbol = '=';
    }

    if game.board.board[end_pos.0][end_pos.1] != ' ' {
        if piece.to_ascii_lowercase() == 'p' {
            let piece_column = match piece_pos.1 {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => ' ',
            };

            capture_symbol = format!("{}x", piece_column);
        } else {
            capture_symbol = "x".to_string();
        }
    }

    if (piece == 'P' && end_pos.0 == 0) || (piece == 'p' && end_pos.0 == 7) {
        let pressed_keys = keyboard::pressed_keys(ctx);

        if pressed_keys.contains(
            &match_key(game.config.queen_upgrade_button.to_string()).unwrap_or(KeyCode::Q),
        ) {
            promotion_symbol = "=Q";
        } else if pressed_keys
            .contains(&match_key(game.config.rook_upgrade_button.to_string()).unwrap_or(KeyCode::R))
        {
            promotion_symbol = "=R";
        } else if pressed_keys.contains(
            &match_key(game.config.knight_upgrade_button.to_string()).unwrap_or(KeyCode::N),
        ) {
            promotion_symbol = "=N";
        } else if pressed_keys.contains(
            &match_key(game.config.bishop_upgrade_button.to_string()).unwrap_or(KeyCode::B),
        ) {
            promotion_symbol = "=B";
        } else {
            promotion_symbol = "=Q";
        }
    }

    if piece == 'K' && castling.0 == true && end_pos == (7, 6) {
        print!("{}. O-O{}", turn_count, check_symbol);
    } else if piece == 'k' && castling.2 == true && end_pos == (0, 6) {
        print!(" O-O{}\n", check_symbol);
    } else if piece == 'K' && castling.1 == true && end_pos == (7, 2) {
        print!("{}. O-O-O{}", turn_count, check_symbol);
    } else if piece == 'k' && castling.3 == true && end_pos == (0, 2) {
        print!(" O-O-O{}\n", check_symbol);
    } else if piece.is_uppercase() {
        print!(
            "{}. {}{}{}{}{}{}",
            turn_count,
            piece_print,
            capture_symbol,
            end_column,
            end_row,
            promotion_symbol,
            check_symbol
        );
    } else {
        print!(
            " {}{}{}{}{}{}\n",
            piece_print, capture_symbol, end_column, end_row, promotion_symbol, check_symbol
        );
    }

    io::stdout().flush().unwrap();
}
