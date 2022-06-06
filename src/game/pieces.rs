use super::{board::Chessboard, check::check, player::Player};

fn get_pawn_moves(
    piece: char,
    position: (usize, usize),
    board: Chessboard,
    en_passant: Option<usize>,
) -> Vec<(usize, usize)> {
    let mut possible_moves = vec![];

    // If the pawn is white, we move upwards.
    if piece.is_uppercase() {
        // Pawns can only move forward when there is no piece in the way.
        if position.0 > 0 {
            if board.board[position.0 - 1][position.1] == ' ' {
                possible_moves.push((position.0 - 1, position.1));
                // Checking for the first pawn move.
                if position.0 == 6 && board.board[position.0 - 2][position.1] == ' ' {
                    possible_moves.push((position.0 - 2, position.1));
                }
            }

            // Checking for capturing to the sides.
            if position.1 > 0 {
                if board.board[position.0 - 1][position.1 - 1] != ' '
                    && board.board[position.0 - 1][position.1 - 1].is_lowercase()
                {
                    possible_moves.push((position.0 - 1, position.1 - 1));
                }
            }
            if position.1 < 7 {
                if board.board[position.0 - 1][position.1 + 1] != ' '
                    && board.board[position.0 - 1][position.1 + 1].is_lowercase()
                {
                    possible_moves.push((position.0 - 1, position.1 + 1));
                }
            }

            // Checking for en passant.
            if position.0 == 3 && en_passant != None {
                if position.0 > 0 && (position.1 as i8 - en_passant.unwrap() as i8 == 1) {
                    possible_moves.push((position.0 - 1, en_passant.unwrap()));
                }
                if position.0 < 7 && (en_passant.unwrap() as i8 - position.1 as i8 == 1) {
                    possible_moves.push((position.0 - 1, en_passant.unwrap()));
                }
            }
        }

    // If the pawn is black, we move downwards.
    } else {
        if position.0 < 7 {
            if board.board[position.0 + 1][position.1] == ' ' {
                possible_moves.push((position.0 + 1, position.1));
                if position.0 == 1 && board.board[position.0 + 2][position.1] == ' ' {
                    possible_moves.push((position.0 + 2, position.1));
                }
            }
            if position.1 < 7 {
                if board.board[position.0 + 1][position.1 + 1] != ' '
                    && board.board[position.0 + 1][position.1 + 1].is_uppercase()
                {
                    possible_moves.push((position.0 + 1, position.1 + 1));
                }
            }
            if position.1 > 0 {
                if board.board[position.0 + 1][position.1 - 1] != ' '
                    && board.board[position.0 + 1][position.1 - 1].is_uppercase()
                {
                    possible_moves.push((position.0 + 1, position.1 - 1));
                }
            }
        }

        if position.0 == 4 && en_passant != None {
            if position.0 > 0 && (position.1 as i8 - en_passant.unwrap() as i8 == 1) {
                possible_moves.push((position.0 + 1, en_passant.unwrap()));
            }
            if position.0 < 7 && (en_passant.unwrap() as i8 - position.1 as i8 == 1) {
                possible_moves.push((position.0 + 1, en_passant.unwrap()));
            }
        }
    }

    return possible_moves;
}
fn get_bishop_moves(
    piece: char,
    position: (usize, usize),
    board: Chessboard,
) -> Vec<(usize, usize)> {
    let mut possible_moves = vec![];

    let mut x = position.0;
    let mut y = position.1;

    // Diagonally up-left.
    loop {
        if board.board[x][y] == ' ' {
            possible_moves.push((x, y));
        } else {
            if Piece::is_same_player(piece, board.board[x][y]) {
                if (x, y) != (position.0, position.1) {
                    break;
                }
            } else {
                possible_moves.push((x, y));
                break;
            }
        }

        if x != 0 && y != 0 {
            x -= 1;
            y -= 1;
        } else {
            break;
        }
    }

    let mut x = position.0;
    let mut y = position.1;

    // Diagonally down-right.
    loop {
        if board.board[x][y] == ' ' {
            possible_moves.push((x, y));
        } else {
            if Piece::is_same_player(piece, board.board[x][y]) {
                if (x, y) != (position.0, position.1) {
                    break;
                }
            } else {
                possible_moves.push((x, y));
                break;
            }
        }

        if x != 7 && y != 7 {
            x += 1;
            y += 1;
        } else {
            break;
        }
    }

    let mut x = position.0;
    let mut y = position.1;

    // Diagonally down-left.
    loop {
        if board.board[x][y] == ' ' {
            possible_moves.push((x, y));
        } else {
            if Piece::is_same_player(piece, board.board[x][y]) {
                if (x, y) != (position.0, position.1) {
                    break;
                }
            } else {
                possible_moves.push((x, y));
                break;
            }
        }

        if x != 7 && y != 0 {
            x += 1;
            y -= 1;
        } else {
            break;
        }
    }

    let mut x = position.0;
    let mut y = position.1;

    // Diagonally up-right.
    loop {
        if board.board[x][y] == ' ' {
            possible_moves.push((x, y));
        } else {
            if Piece::is_same_player(piece, board.board[x][y]) {
                if (x, y) != (position.0, position.1) {
                    break;
                }
            } else {
                possible_moves.push((x, y));
                break;
            }
        }

        if y != 7 && x != 0 {
            x -= 1;
            y += 1;
        } else {
            break;
        }
    }

    return possible_moves;
}
fn get_knight_moves(
    piece: char,
    position: (usize, usize),
    board: Chessboard,
) -> Vec<(usize, usize)> {
    let mut possible_moves = vec![];

    // Knight has its own unique moves.
    let knight_moves = [
        (2, 1),
        (1, 2),
        (2, -1),
        (1, -2),
        (-2, 1),
        (-1, 2),
        (-2, -1),
        (-1, -2),
    ];

    for km in knight_moves {
        let end_pos_x = (position.0 as i8) + km.0;
        let end_pos_y = (position.1 as i8) + km.1;

        if end_pos_x >= 0 && end_pos_x <= 7 && end_pos_y >= 0 && end_pos_y <= 7 {
            if !Piece::is_same_player(piece, board.board[end_pos_x as usize][end_pos_y as usize]) {
                possible_moves.push((end_pos_x as usize, end_pos_y as usize));
            }
        }
    }

    return possible_moves;
}
fn get_rook_moves(piece: char, position: (usize, usize), board: Chessboard) -> Vec<(usize, usize)> {
    let mut possible_moves = vec![];

    let mut x = position.0;
    let y = position.1;

    // Up.
    loop {
        if board.board[x][y] == ' ' {
            possible_moves.push((x, y));
        } else {
            if Piece::is_same_player(piece, board.board[x][y]) {
                if (x, y) != (position.0, position.1) {
                    break;
                }
            } else {
                possible_moves.push((x, y));
                break;
            }
        }
        if x != 0 {
            x -= 1;
        } else {
            break;
        }
    }

    let mut x = position.0;
    let y = position.1;

    // Down.
    while x <= 7 {
        if board.board[x][y] == ' ' {
            possible_moves.push((x, y));
        } else {
            if Piece::is_same_player(piece, board.board[x][y]) {
                if (x, y) != (position.0, position.1) {
                    break;
                }
            } else {
                possible_moves.push((x, y));
                break;
            }
        }
        x += 1;
    }

    let x = position.0;
    let mut y = position.1;

    // Left.
    loop {
        if board.board[x][y] == ' ' {
            possible_moves.push((x, y));
        } else {
            if Piece::is_same_player(piece, board.board[x][y]) {
                if (x, y) != (position.0, position.1) {
                    break;
                }
            } else {
                possible_moves.push((x, y));
                break;
            }
        }
        if y != 0 {
            y -= 1;
        } else {
            break;
        }
    }

    let x = position.0;
    let mut y = position.1;

    // Right.
    while y <= 7 {
        if board.board[x][y] == ' ' {
            possible_moves.push((x, y));
        } else {
            if Piece::is_same_player(piece, board.board[x][y]) {
                if (x, y) != (position.0, position.1) {
                    break;
                }
            } else {
                possible_moves.push((x, y));
                break;
            }
        }
        y += 1;
    }

    return possible_moves;
}
fn get_queen_moves(
    piece: char,
    position: (usize, usize),
    board: Chessboard,
) -> Vec<(usize, usize)> {
    let mut possible_moves = vec![];

    // Queen moves like a rook and bishop combined.
    possible_moves.append(&mut get_bishop_moves(piece, position, board));
    possible_moves.append(&mut get_rook_moves(piece, position, board));

    return possible_moves;
}
fn get_king_moves(
    piece: char,
    position: (usize, usize),
    board: Chessboard,
    castling: (bool, bool, bool, bool),
    en_passant: Option<usize>,
) -> Vec<(usize, usize)> {
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();

    for i in ((position.0).max(1) - 1)..=position.0.min(6) + 1 {
        for j in ((position.1).max(1) - 1)..=position.1.min(6) + 1 {
            if board.board[i][j] == ' ' || Piece::is_opposite_player(piece, board.board[i][j]) {
                possible_moves.push((i, j));
            }
        }
    }

    // We check if castling would drag the king through a square that is targeted by the other player.
    if piece.is_lowercase() {
        if castling.2 == true
            && board.board[0][6] == ' '
            && board.board[0][5] == ' '
            && board.board[0][7] == 'r'
            && !check(board, Player::Black, en_passant, castling)
        {
            let mut temp_board = board;

            temp_board.board[0][5] = 'k';
            temp_board.board[0][4] = ' ';

            if !check(temp_board, Player::Black, en_passant, castling) {
                possible_moves.push((0, 6))
            }
        }
        if castling.3 == true
            && board.board[0][1] == ' '
            && board.board[0][2] == ' '
            && board.board[0][3] == ' '
            && board.board[0][0] == 'r'
            && !check(board, Player::Black, en_passant, castling)
        {
            let mut temp_board = board;

            temp_board.board[0][3] = 'k';
            temp_board.board[0][4] = ' ';

            if !check(temp_board, Player::Black, en_passant, castling) {
                possible_moves.push((0, 2))
            }
        }
    } else if piece.is_uppercase() {
        if castling.0 == true
            && board.board[7][6] == ' '
            && board.board[7][5] == ' '
            && board.board[7][7] == 'R'
            && !check(board, Player::White, en_passant, castling)
        {
            let mut temp_board = board;

            temp_board.board[7][5] = 'K';
            temp_board.board[7][4] = ' ';

            if !check(temp_board, Player::White, en_passant, castling) {
                possible_moves.push((7, 6))
            }
        }
        if castling.1 == true
            && board.board[7][1] == ' '
            && board.board[7][2] == ' '
            && board.board[7][3] == ' '
            && board.board[7][0] == 'R'
            && !check(board, Player::White, en_passant, castling)
        {
            let mut temp_board = board;

            temp_board.board[7][3] = 'K';
            temp_board.board[7][4] = ' ';

            if !check(temp_board, Player::White, en_passant, castling) {
                possible_moves.push((7, 2))
            }
        }
    }

    return possible_moves;
}

pub struct Piece {}

impl Piece {
    /// We get the corresponding piece image.
    /// The Windows file system is case-insensitve by default, so we add this little prefix.
    pub fn get_image(piece: char) -> String {
        if piece == ' ' {
            return "/images/ep.png".to_string();
        }

        if piece.is_lowercase() {
            return format!("/images/b{}.png", piece);
        } else {
            return format!("/images/w{}.png", piece.to_ascii_lowercase());
        }
    }

    /// Getting the controlling player of a piece.
    pub fn get_player(piece: char) -> Player {
        if piece == ' ' {
            return Player::None;
        } else if piece.is_lowercase() {
            return Player::Black;
        } else {
            return Player::White;
        }
    }

    /// Checking if two pieces belong to the same player.
    pub fn is_same_player(piece1: char, piece2: char) -> bool {
        return Self::get_player(piece1) == Self::get_player(piece2);
    }

    /// Getting all pieces owned by a player.
    pub fn get_all_player_pieces(player: Player, board: Chessboard) -> Vec<(char, (usize, usize))> {
        let mut all_pieces: Vec<(char, (usize, usize))> = Vec::new();

        for (x, i) in board.board.iter().enumerate() {
            for (y, j) in i.iter().enumerate() {
                if player == Player::White && j.is_uppercase() {
                    all_pieces.push((*j, (x, y)));
                } else if player == Player::Black && j.is_lowercase() {
                    all_pieces.push((*j, (x, y)));
                }
            }
        }

        return all_pieces;
    }

    /// Checking if two pieces are owned by the opposing players.
    pub fn is_opposite_player(piece1: char, piece2: char) -> bool {
        if piece1 == ' ' || piece2 == ' ' {
            return false;
        }

        return !Self::is_same_player(piece1, piece2);
    }

    /// Gets the "value" of the chess piece.
    /// https://en.wikipedia.org/wiki/Chess_piece_relative_value
    pub fn get_value(piece: char) -> u8 {
        match piece {
            'p' | 'P' => 1,
            'n' | 'N' => 3,
            'b' | 'B' => 3,
            'r' | 'R' => 5,
            'q' | 'Q' => 9,
            _ => 0,
        }
    }

    /// Gets the pieces the player is missing, aka that have been captured.
    pub fn get_missing_pieces(player: Player, board: Chessboard) -> Vec<char> {
        let mut all_pieces: Vec<char> = Vec::new();

        let current_pieces = Self::get_all_player_pieces(player, board);

        if player == Player::White {
            all_pieces.extend_from_slice(&['P'; 8]);
            all_pieces.extend_from_slice(&['R'; 2]);
            all_pieces.extend_from_slice(&['B'; 2]);
            all_pieces.extend_from_slice(&['N'; 2]);
            all_pieces.extend_from_slice(&['Q']);
        } else if player == Player::Black {
            all_pieces.extend_from_slice(&['p'; 8]);
            all_pieces.extend_from_slice(&['r'; 2]);
            all_pieces.extend_from_slice(&['b'; 2]);
            all_pieces.extend_from_slice(&['n'; 2]);
            all_pieces.extend_from_slice(&['q']);
        }

        for existing_piece in current_pieces {
            if let Some(pos) = all_pieces.iter().position(|p| *p == existing_piece.0) {
                all_pieces.remove(pos);
            }
        }

        return all_pieces;
    }

    /// Getting the possible moves that can be made by a piece.
    pub fn get_possible_moves(
        piece: char,
        position: (usize, usize),
        board: Chessboard,
        en_passant: Option<usize>,
        castling: (bool, bool, bool, bool),
    ) -> Vec<(usize, usize)> {
        let mut possible_moves = vec![];

        match piece {
            'p' | 'P' => {
                possible_moves.append(&mut get_pawn_moves(piece, position, board, en_passant))
            }
            'n' | 'N' => possible_moves.append(&mut get_knight_moves(piece, position, board)),
            'b' | 'B' => possible_moves.append(&mut get_bishop_moves(piece, position, board)),
            'r' | 'R' => possible_moves.append(&mut get_rook_moves(piece, position, board)),
            'q' | 'Q' => possible_moves.append(&mut get_queen_moves(piece, position, board)),
            'k' | 'K' => possible_moves.append(&mut get_king_moves(
                piece, position, board, castling, en_passant,
            )),
            _ => (),
        }

        return possible_moves;
    }
}
