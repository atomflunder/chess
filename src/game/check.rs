use super::board::Chessboard;
use super::game::MainGame;
use super::pieces::Piece;
use super::player::Player;

use ggez::Context;

/// Returns true if the given player is in check.
pub fn check(
    board: Chessboard,
    player: Player,
    en_passant: Option<usize>,
    castling: (bool, bool, bool, bool),
) -> bool {
    let mut white_king_pos: (usize, usize) = (0, 0);
    let mut black_king_pos: (usize, usize) = (0, 0);

    for (x, i) in board.board.iter().enumerate() {
        for (y, j) in i.iter().enumerate() {
            if j == &'k' {
                black_king_pos = (x, y);
            } else if j == &'K' {
                white_king_pos = (x, y);
            }
        }
    }

    if player == Player::White {
        for (x, i) in board.board.iter().enumerate() {
            for (y, j) in i.iter().enumerate() {
                if Piece::get_player(*j) == Player::Black {
                    if Piece::get_possible_moves(*j, (x, y), board, en_passant, castling)
                        .contains(&white_king_pos)
                    {
                        return true;
                    }
                }
            }
        }
    } else if player == Player::Black {
        for (x, i) in board.board.iter().enumerate() {
            for (y, j) in i.iter().enumerate() {
                if Piece::get_player(*j) == Player::White {
                    if Piece::get_possible_moves(*j, (x, y), board, en_passant, castling)
                        .contains(&black_king_pos)
                    {
                        return true;
                    }
                }
            }
        }
    }

    return false;
}

/// Returns the player that is in checkmate, or None if no one is in checkmate.
pub fn player_checkmate(
    board: Chessboard,
    en_passant: Option<usize>,
    castling: (bool, bool, bool, bool),
    game: MainGame,
    ctx: &Context,
) -> Option<Player> {
    if check(board, Player::White, en_passant, castling) == true {
        let all_pieces = Piece::get_all_player_pieces(Player::White, board);

        for piece in all_pieces {
            let moves = Piece::get_possible_moves(piece.0, piece.1, board, en_passant, castling);

            for mv in moves.clone() {
                let mut temp_game = game.clone();

                temp_game.make_move(ctx, piece.0, piece.1, moves.clone(), mv.0, mv.1);

                if !check(
                    temp_game.board,
                    Player::White,
                    temp_game.en_passant,
                    temp_game.castling,
                ) {
                    return None;
                }
            }
        }

        return Some(Player::White);
    } else if check(board, Player::Black, en_passant, castling) == true {
        let all_pieces = Piece::get_all_player_pieces(Player::Black, board);

        for piece in all_pieces {
            let moves = Piece::get_possible_moves(piece.0, piece.1, board, en_passant, castling);

            for mv in moves.clone() {
                let mut temp_game = game.clone();

                temp_game.make_move(ctx, piece.0, piece.1, moves.clone(), mv.0, mv.1);

                if !check(
                    temp_game.board,
                    Player::Black,
                    temp_game.en_passant,
                    temp_game.castling,
                ) {
                    return None;
                }
            }
        }

        return Some(Player::Black);
    } else {
        return None;
    }
}

/// Returns true if the game is tied.
pub fn is_tie(
    board: Chessboard,
    en_passant: Option<usize>,
    castling: (bool, bool, bool, bool),
    game: MainGame,
    ctx: &Context,
) -> bool {
    let black_pieces = Piece::get_all_player_pieces(Player::Black, board);
    let white_pieces = Piece::get_all_player_pieces(Player::White, board);

    if black_pieces.len() == 1 && white_pieces.len() == 1 {
        return true;
    }

    // You cannot checkmate the other king with just a knight or bishop.
    if black_pieces.len() == 2 && white_pieces.len() == 1 {
        // One of the black pieces will be the King,
        // so we only need to check if the other is either a bishop or a knight.
        if black_pieces[0].0 == 'b'
            || black_pieces[1].0 == 'b'
            || black_pieces[0].0 == 'n'
            || black_pieces[1].0 == 'n'
        {
            return true;
        }
    }

    if black_pieces.len() == 1 && white_pieces.len() == 2 {
        if white_pieces[0].0 == 'B'
            || white_pieces[1].0 == 'B'
            || white_pieces[0].0 == 'N'
            || white_pieces[1].0 == 'N'
        {
            return true;
        }
    }

    // Checking for stalemates.
    if game.turn == Player::White && check(board, Player::White, en_passant, castling) == false {
        let all_pieces = Piece::get_all_player_pieces(Player::White, board);

        for piece in all_pieces {
            let mut moves =
                Piece::get_possible_moves(piece.0, piece.1, board, en_passant, castling);

            for mv in moves.clone() {
                let mut temp_game = game.clone();

                temp_game.make_move(ctx, piece.0, piece.1, moves.clone(), mv.0, mv.1);

                if check(
                    temp_game.board,
                    Player::White,
                    temp_game.en_passant,
                    temp_game.castling,
                ) {
                    moves.retain(|m| m != &mv);
                }
            }

            if !moves.is_empty() {
                return false;
            }
        }
        return true;
    } else if game.turn == Player::Black
        && check(board, Player::Black, en_passant, castling) == false
    {
        let all_pieces = Piece::get_all_player_pieces(Player::Black, board);

        for piece in all_pieces {
            let mut moves =
                Piece::get_possible_moves(piece.0, piece.1, board, en_passant, castling);

            for mv in moves.clone() {
                let mut temp_game = game.clone();

                temp_game.make_move(ctx, piece.0, piece.1, moves.clone(), mv.0, mv.1);

                if check(
                    temp_game.board,
                    Player::Black,
                    temp_game.en_passant,
                    temp_game.castling,
                ) {
                    moves.retain(|m| m != &mv);
                }
            }

            if !moves.is_empty() {
                return false;
            }
        }

        return true;
    }
    return false;
}
