use std::path;

use crate::game::check::check;

use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::input;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use glam::Vec2;

use super::board::Chessboard;
use super::check;
use super::config::match_key;
use super::config::{load_config, Config};
use super::notation::print_move;
use super::pieces::Piece;
use super::player::Player;

#[derive(Clone)]
pub struct MainGame {
    pub board: Chessboard,
    pub selected_piece: char,
    pub selected_piece_pos: (usize, usize),
    pub castling: (bool, bool, bool, bool),
    pub en_passant: Option<usize>,
    pub turn: Player,
    pub turn_count: usize,
    pub config: Config,
    pub last_move: Option<((usize, usize), (usize, usize))>,
    pub draw_mode: bool,
    pub drawn_pixels: Vec<(f32, f32)>,
}

impl MainGame {
    fn new() -> GameResult<MainGame> {
        let b = MainGame {
            board: Chessboard::new(),
            selected_piece: ' ',
            selected_piece_pos: (9, 9),
            castling: (true, true, true, true),
            en_passant: None,
            turn: Player::White,
            turn_count: 1,
            config: load_config(),
            last_move: None,
            draw_mode: false,
            drawn_pixels: Vec::new(),
        };

        Ok(b)
    }

    /// Moves a piece on the MainGame board.
    /// Checks for the "special moves" such as castling, en passant etc.
    pub fn make_move(
        &mut self,
        ctx: &Context,
        piece: char,
        piece_pos: (usize, usize),
        possible_moves: Vec<(usize, usize)>,
        y_sq: usize,
        x_sq: usize,
    ) {
        if possible_moves.contains(&(y_sq, x_sq)) && Piece::get_player(piece) == self.turn {
            // Checking for Castling:
            if piece == 'K' {
                if self.castling.0 == true
                    && self.board.board[7][6] == ' '
                    && self.board.board[7][5] == ' '
                    && self.board.board[7][7] == 'R'
                    && y_sq == 7
                    && x_sq == 6
                {
                    self.board.board[7][7] = ' ';
                    self.board.board[7][4] = ' ';
                    self.board.board[7][5] = 'R';
                } else if self.castling.1 == true
                    && self.board.board[7][1] == ' '
                    && self.board.board[7][2] == ' '
                    && self.board.board[7][3] == ' '
                    && self.board.board[7][0] == 'R'
                    && y_sq == 7
                    && x_sq == 2
                {
                    self.board.board[7][0] = ' ';
                    self.board.board[7][4] = ' ';
                    self.board.board[7][3] = 'R';
                }
            }

            if piece == 'k' {
                if self.castling.2 == true
                    && self.board.board[0][6] == ' '
                    && self.board.board[0][5] == ' '
                    && self.board.board[0][7] == 'r'
                    && y_sq == 0
                    && x_sq == 6
                {
                    self.board.board[0][7] = ' ';
                    self.board.board[0][4] = ' ';
                    self.board.board[0][5] = 'r';
                } else if self.castling.3 == true
                    && self.board.board[0][1] == ' '
                    && self.board.board[0][2] == ' '
                    && self.board.board[0][3] == ' '
                    && self.board.board[0][0] == 'r'
                    && y_sq == 0
                    && x_sq == 2
                {
                    self.board.board[0][0] = ' ';
                    self.board.board[0][4] = ' ';
                    self.board.board[0][3] = 'r';
                }
            }

            // Actually making the move.
            self.board.board[piece_pos.0][piece_pos.1] = ' ';
            self.board.board[y_sq][x_sq] = piece;

            // Checking for upgrading a pawn.
            self.upgrade_pawn(ctx, (y_sq, x_sq));

            // Checking for en passant:
            if piece == 'P' && piece_pos.0 == 3 && self.en_passant == Some(x_sq) && y_sq == 2 {
                self.board.board[y_sq + 1][x_sq] = ' ';
            }

            if piece == 'p' && piece_pos.0 == 4 && self.en_passant == Some(x_sq) && y_sq == 5 {
                self.board.board[y_sq - 1][x_sq] = ' ';
            }

            // Enabling en passant.
            self.en_passant = None;

            if piece == 'p' && piece_pos.0 == 1 && y_sq == 3 {
                self.en_passant = Some(piece_pos.1);
            } else if piece == 'P' && piece_pos.0 == 6 && y_sq == 4 {
                self.en_passant = Some(piece_pos.1);
            }

            // Disabling castling.
            if piece == 'k' {
                self.castling.2 = false;
                self.castling.3 = false;
            } else if piece == 'K' {
                self.castling.0 = false;
                self.castling.1 = false;
            }

            if piece == 'r' {
                if piece_pos == (0, 0) {
                    self.castling.3 = false;
                } else if piece_pos == (0, 7) {
                    self.castling.2 = false;
                }
            }

            if piece == 'R' {
                if piece_pos == (7, 0) {
                    self.castling.1 = false;
                } else if piece_pos == (7, 7) {
                    self.castling.0 = false;
                }
            }

            // Switching turns.
            if self.turn == Player::White {
                self.turn = Player::Black;
            } else {
                self.turn = Player::White;
            }
        }
        self.selected_piece = ' ';
    }

    /// Upgrades a pawn when it reaches the end of the board.
    /// Hold the Q, R, N or B keys to get the different pieces.
    fn upgrade_pawn(&mut self, ctx: &Context, position: (usize, usize)) {
        let pressed_keys = keyboard::pressed_keys(ctx);

        if self.selected_piece == 'P' && position.0 == 0 {
            // They are sorted roughly in order of usefulness, in my opinion.
            if pressed_keys.contains(
                &match_key(self.config.queen_upgrade_button.to_string()).unwrap_or(KeyCode::Q),
            ) {
                self.board.board[position.0][position.1] = 'Q';
            } else if pressed_keys.contains(
                &match_key(self.config.rook_upgrade_button.to_string()).unwrap_or(KeyCode::R),
            ) {
                self.board.board[position.0][position.1] = 'R';
            } else if pressed_keys.contains(
                &match_key(self.config.knight_upgrade_button.to_string()).unwrap_or(KeyCode::N),
            ) {
                self.board.board[position.0][position.1] = 'N';
            } else if pressed_keys.contains(
                &match_key(self.config.bishop_upgrade_button.to_string()).unwrap_or(KeyCode::B),
            ) {
                self.board.board[position.0][position.1] = 'B';
            } else {
                self.board.board[position.0][position.1] = 'Q';
            }
        } else if self.selected_piece == 'p' && position.0 == 7 {
            if pressed_keys.contains(
                &match_key(self.config.queen_upgrade_button.to_string()).unwrap_or(KeyCode::Q),
            ) {
                self.board.board[position.0][position.1] = 'q';
            } else if pressed_keys.contains(
                &match_key(self.config.rook_upgrade_button.to_string()).unwrap_or(KeyCode::R),
            ) {
                self.board.board[position.0][position.1] = 'r';
            } else if pressed_keys.contains(
                &match_key(self.config.knight_upgrade_button.to_string()).unwrap_or(KeyCode::N),
            ) {
                self.board.board[position.0][position.1] = 'n';
            } else if pressed_keys.contains(
                &match_key(self.config.bishop_upgrade_button.to_string()).unwrap_or(KeyCode::B),
            ) {
                self.board.board[position.0][position.1] = 'b';
            } else {
                self.board.board[position.0][position.1] = 'q';
            }
        }
    }

    /// Takes a list of possible moves as an input and only returns the legal moves.
    pub fn get_legal_moves(
        &mut self,
        ctx: &Context,
        possible_moves: Vec<(usize, usize)>,
        y_sq: usize,
        x_sq: usize,
    ) -> Vec<(usize, usize)> {
        let mut possible_moves = possible_moves.clone();

        possible_moves = self.prevent_selfcheck_moves(ctx, possible_moves, y_sq, x_sq);
        possible_moves = self.only_check_preventing_moves(ctx, possible_moves, y_sq, x_sq);

        return possible_moves;
    }

    /// Returns moves that do not put the own king in check.
    pub fn prevent_selfcheck_moves(
        &mut self,
        ctx: &Context,
        possible_moves: Vec<(usize, usize)>,
        y_sq: usize,
        x_sq: usize,
    ) -> Vec<(usize, usize)> {
        let mut possible_moves = possible_moves;

        if self.turn == Player::White {
            let mut temp_board = self.clone();

            temp_board.make_move(
                ctx,
                self.selected_piece,
                self.selected_piece_pos,
                possible_moves.clone(),
                y_sq,
                x_sq,
            );

            if check(
                temp_board.board.clone(),
                Player::White,
                temp_board.en_passant,
                temp_board.castling,
            ) {
                possible_moves.retain(|x| x != &(y_sq, x_sq));
            }
        } else if self.turn == Player::Black {
            let mut temp_board = self.clone();

            temp_board.make_move(
                ctx,
                self.selected_piece,
                self.selected_piece_pos,
                possible_moves.clone(),
                y_sq,
                x_sq,
            );
            if check(
                temp_board.board,
                Player::Black,
                temp_board.en_passant,
                temp_board.castling,
            ) {
                possible_moves.retain(|x| x != &(y_sq, x_sq));
            }
        }

        return possible_moves;
    }

    /// If the king is in check, only returns moves that are legal in that position.
    pub fn only_check_preventing_moves(
        &mut self,
        ctx: &Context,
        possible_moves: Vec<(usize, usize)>,
        y_sq: usize,
        x_sq: usize,
    ) -> Vec<(usize, usize)> {
        let mut possible_moves = possible_moves.clone();

        if self.turn == Player::White {
            if check(self.board, Player::White, self.en_passant, self.castling) == true {
                let all_pieces = Piece::get_all_player_pieces(Player::White, self.board);

                for p in all_pieces {
                    for mv in Piece::get_possible_moves(
                        p.0,
                        p.1,
                        self.board,
                        self.en_passant,
                        self.castling,
                    ) {
                        let mut temp_game: MainGame = self.clone();

                        temp_game.make_move(
                            ctx,
                            self.selected_piece,
                            self.selected_piece_pos,
                            possible_moves.clone(),
                            y_sq,
                            x_sq,
                        );

                        if check(
                            temp_game.board,
                            Player::White,
                            temp_game.en_passant,
                            temp_game.castling,
                        ) {
                            possible_moves.retain(|x| x != &mv);
                        }
                    }
                }
            }
        } else if self.turn == Player::Black {
            if check(self.board, Player::Black, self.en_passant, self.castling) == true {
                let all_pieces = Piece::get_all_player_pieces(Player::Black, self.board.clone());

                for p in all_pieces {
                    for mv in Piece::get_possible_moves(
                        p.0,
                        p.1,
                        self.board,
                        self.en_passant,
                        self.castling,
                    ) {
                        let mut temp_game: MainGame = self.clone();

                        temp_game.make_move(
                            ctx,
                            self.selected_piece,
                            self.selected_piece_pos,
                            possible_moves.clone(),
                            y_sq,
                            x_sq,
                        );

                        if check(
                            temp_game.board,
                            Player::Black,
                            temp_game.en_passant,
                            temp_game.castling,
                        ) {
                            possible_moves.retain(|x| x != &mv);
                        }
                    }
                }
            }
        }

        return possible_moves;
    }
}

impl event::EventHandler<ggez::GameError> for MainGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, self.config.light_square_color.into());

        let mut offset = (0.0, 0.0);
        let mut coordinates: (usize, usize) = (0, 0);
        let mut last_square_dark = true;

        // Defining all of the different assets.
        let font = graphics::Font::new(ctx, "/fonts/consolas.ttf")?;

        let dark_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.dark_square_color.into(),
        )?;

        let selected_piece_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.selected_piece_color.into(),
        )?;

        let last_move_start_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.last_move_start_color.into(),
        )?;

        let last_move_end_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.last_move_end_color.into(),
        )?;

        let checkmate_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.checkmate_color.into(),
        )?;

        let tied_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.tie_color.into(),
        )?;

        let highlighted_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.possible_moves_color.into(),
        )?;

        let capturable_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.possible_captures_color.into(),
        )?;

        let checked_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                offset.0,
                offset.1,
                self.config.window_size_horizontal / 8.0,
                self.config.window_size_vertical / 8.0,
            ),
            self.config.check_color.into(),
        )?;

        let draw_pixel = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(offset.0, offset.1),
            self.config.draw_thickness,
            1.0,
            self.config.draw_color.into(),
        )?;

        // Checking for checks.
        let current_king_in_check: bool =
            check(self.board, self.turn, self.en_passant, self.castling);

        // Checking if the game is over.
        let tie = check::is_tie(
            self.board,
            self.en_passant,
            self.castling,
            self.clone(),
            ctx,
        );

        let checkmate = check::player_checkmate(
            self.board,
            self.en_passant,
            self.castling,
            self.clone(),
            ctx,
        );

        for y in self.board.board {
            offset.0 = 0.0;
            coordinates.1 = 0;
            for x in y {
                // First we draw on the dark squares.
                if !last_square_dark {
                    graphics::draw(
                        ctx,
                        &dark_square,
                        graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                    )?;
                }

                // Then we draw the last move made, by default in dark and light green.
                if self.last_move != None {
                    if coordinates == self.last_move.unwrap().0 {
                        graphics::draw(
                            ctx,
                            &last_move_start_square,
                            graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                        )?;
                    } else if coordinates == self.last_move.unwrap().1 {
                        graphics::draw(
                            ctx,
                            &last_move_end_square,
                            graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                        )?;
                    }
                }

                // Then we highlight the currently selected piece and the possible moves.
                if self.selected_piece != ' ' {
                    if coordinates == self.selected_piece_pos {
                        graphics::draw(
                            ctx,
                            &selected_piece_square,
                            graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                        )?;
                    }

                    let mut possible_moves = Piece::get_possible_moves(
                        self.selected_piece,
                        self.selected_piece_pos,
                        self.board,
                        self.en_passant,
                        self.castling,
                    );

                    // Removing the illegal moves from the possible moves.
                    possible_moves =
                        self.get_legal_moves(ctx, possible_moves, coordinates.0, coordinates.1);

                    // Highlighting some squares.
                    if possible_moves.contains(&coordinates) {
                        // If a piece is capturable we highlight it in magenta by default.
                        if Piece::is_opposite_player(
                            self.selected_piece,
                            self.board.board[coordinates.0][coordinates.1],
                        ) {
                            graphics::draw(
                                ctx,
                                &capturable_square,
                                graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                            )?;
                        } else {
                            // Highlighting the other possible moves with empty spaces in cyan by default.
                            graphics::draw(
                                ctx,
                                &highlighted_square,
                                graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                            )?;
                        }
                    }
                }

                // Highlighting the kings square in red by default if the king is in check.
                if self.turn == Player::White && x == 'K' {
                    if current_king_in_check == true {
                        graphics::draw(
                            ctx,
                            &checked_square,
                            graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                        )?;
                    }
                } else if self.turn == Player::Black && x == 'k' {
                    if current_king_in_check == true {
                        graphics::draw(
                            ctx,
                            &checked_square,
                            graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                        )?;
                    }
                }

                // If one player is in checkmate we color every piece of theirs in dark red by default.
                if checkmate != None {
                    if Piece::get_player(x) == checkmate.unwrap() {
                        graphics::draw(
                            ctx,
                            &checkmate_square,
                            graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                        )?;
                    }
                }

                // If the game is tied we highlight every piece in a dark blue color by default.
                if tie == true {
                    if Piece::get_player(x) != Player::None {
                        graphics::draw(
                            ctx,
                            &tied_square,
                            graphics::DrawParam::default().dest(Vec2::new(offset.0, offset.1)),
                        )?;
                    }
                }

                // We draw on the coordinates of the chess board.
                if coordinates.0 == 7 {
                    let displayed_text = match coordinates.1 {
                        0 => "A",
                        1 => "B",
                        2 => "C",
                        3 => "D",
                        4 => "E",
                        5 => "F",
                        6 => "G",
                        7 => "H",
                        _ => " ",
                    };

                    let text = graphics::Text::new((displayed_text, font, self.config.font_size));

                    graphics::draw(
                        ctx,
                        &text,
                        graphics::DrawParam::default()
                            .dest(Vec2::new(
                                offset.0,
                                offset.1 + (self.config.window_size_vertical / 8.0)
                                    // Not very sure why 1.25 is needed but it works with all scales.
                                    - (self.config.font_size / 1.25),
                            ))
                            .color(self.config.font_color.into()),
                    )?;
                }

                if coordinates.1 == 7 {
                    let text = graphics::Text::new((
                        (format!("{}", (8 - coordinates.0))),
                        font,
                        self.config.font_size,
                    ));

                    graphics::draw(
                        ctx,
                        &text,
                        graphics::DrawParam::default()
                            .dest(Vec2::new(
                                offset.0 + (self.config.window_size_horizontal / 8.0)
                                    // Same as above with the 1.25
                                    - (self.config.font_size / 2.0),
                                offset.1,
                            ))
                            .color(self.config.font_color.into()),
                    )?;
                }

                let image = Image::new(ctx, Piece::get_image(x))?;

                graphics::draw(
                    ctx,
                    &image,
                    graphics::DrawParam::default()
                        .dest(Vec2::new(offset.0, offset.1))
                        .scale(Vec2::new(
                            // The default size for the pieces is 60x60, and 60*8 = 480.
                            self.config.window_size_horizontal / 480.0,
                            self.config.window_size_vertical / 480.0,
                        )),
                )?;

                // ^= just reverses a boolean.
                last_square_dark ^= true;
                offset.0 += self.config.window_size_horizontal / 8.0;
                coordinates.1 += 1;
            }
            last_square_dark ^= true;
            offset.1 += self.config.window_size_vertical / 8.0;
            coordinates.0 += 1;
        }

        // We draw the selected piece under the cursor, to create a drag and drop effect.
        if self.selected_piece != ' ' {
            let image = Image::new(ctx, Piece::get_image(self.selected_piece))?;

            graphics::draw(
                ctx,
                &image,
                graphics::DrawParam::default()
                    .dest(input::mouse::position(ctx))
                    .scale(Vec2::new(
                        // The default size for the pieces is 60x60, and 60*8 = 480.
                        self.config.window_size_horizontal / 480.0,
                        self.config.window_size_vertical / 480.0,
                    ))
                    .offset(Vec2::new(0.5, 0.5)),
            )?;
        }

        // First we push the pixels to the drawn ones,
        // there's probably a way to do this that yields better performance.
        if self.draw_mode == true {
            self.drawn_pixels
                .push((input::mouse::position(ctx).x, input::mouse::position(ctx).y));
        }

        // Then we draw on the pixels in the Vec.
        if !self.drawn_pixels.is_empty() {
            for p in &self.drawn_pixels {
                graphics::draw(
                    ctx,
                    &draw_pixel,
                    graphics::DrawParam::default().dest(Vec2::new(p.0, p.1)),
                )?;
            }
        }

        graphics::present(ctx)?;

        // If we do not clear the font cache, it will leak a lot of memory (30MB/s).
        // We could also just use the default font, but I like the customisability.
        graphics::clear_font_cache(ctx);

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        match button {
            MouseButton::Left => {
                let x_sq = (x / (self.config.window_size_horizontal / 8.0)).floor() as usize;
                let y_sq = (y / (self.config.window_size_vertical / 8.0)).floor() as usize;

                if self.selected_piece == ' ' {
                    if Piece::get_player(self.board.board[y_sq][x_sq]) == self.turn {
                        self.selected_piece = self.board.board[y_sq][x_sq];
                        self.selected_piece_pos = (y_sq, x_sq);
                    }
                }

                // Checking if the game is over.
                let checkmate = check::player_checkmate(
                    self.board,
                    self.en_passant,
                    self.castling,
                    self.clone(),
                    ctx,
                );

                let tie = check::is_tie(
                    self.board,
                    self.en_passant,
                    self.castling,
                    self.clone(),
                    ctx,
                );

                // Restarting the game.
                if checkmate != None || tie == true {
                    let key_pressed = keyboard::pressed_keys(ctx);

                    if !key_pressed.is_empty() {
                        self.board = Chessboard::new();
                        self.en_passant = None;
                        self.castling = (true, true, true, true);
                        self.selected_piece = ' ';
                        self.selected_piece_pos = (9, 9);
                        self.turn = Player::White;
                        self.turn_count = 1;
                        self.last_move = None;
                        self.draw_mode = false;
                        self.drawn_pixels = Vec::new();
                    }

                    println!("\nPress any key and click the board to restart.\n")
                }
            }
            MouseButton::Right => {
                self.draw_mode = true;
            }
            _ => (),
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        match button {
            MouseButton::Left => {
                let x_sq = (x / (self.config.window_size_horizontal / 8.0)).floor() as usize;
                let y_sq = (y / (self.config.window_size_vertical / 8.0)).floor() as usize;

                let mut possible_moves = Piece::get_possible_moves(
                    self.selected_piece,
                    self.selected_piece_pos,
                    self.board,
                    self.en_passant,
                    self.castling,
                );

                // Only getting moves that are currently legal moves.
                possible_moves = self.get_legal_moves(ctx, possible_moves, y_sq, x_sq);

                if self.selected_piece == ' ' {
                    if Piece::get_player(self.board.board[y_sq][x_sq]) == self.turn {
                        self.selected_piece = self.board.board[y_sq][x_sq];
                        self.selected_piece_pos = (y_sq, x_sq);
                    }
                } else {
                    if possible_moves.contains(&(y_sq, x_sq)) {
                        // Printing the move to the console.
                        print_move(
                            ctx,
                            self.clone(),
                            self.selected_piece,
                            self.selected_piece_pos,
                            possible_moves.clone(),
                            (y_sq, x_sq),
                            self.en_passant,
                            self.castling,
                            self.turn_count,
                        );

                        // The turn counter only gets incremented every other turn,
                        // like in real chess, where one move is only a half-turn.
                        if self.turn == Player::Black {
                            self.turn_count += 1;
                        }

                        self.last_move = Some(((self.selected_piece_pos), (y_sq, x_sq)));

                        // Playing some sound effects. The default ones are from lichess.
                        if self.board.board[y_sq][x_sq] == ' ' {
                            let mut move_sound =
                                audio::Source::new(ctx, "/sounds/move.mp3").unwrap();
                            move_sound.set_volume(self.config.volume);
                            let _ = move_sound.play_detached(ctx);
                        } else {
                            let mut capture_sound =
                                audio::Source::new(ctx, "/sounds/capture.mp3").unwrap();
                            capture_sound.set_volume(self.config.volume);
                            let _ = capture_sound.play_detached(ctx);
                        }
                    }

                    // And then actually making the move.
                    self.make_move(
                        ctx,
                        self.selected_piece,
                        self.selected_piece_pos,
                        possible_moves,
                        y_sq,
                        x_sq,
                    );
                }

                // Setting the difference in piece value in the title, seemed like a nice touch.
                let mut difference: i8 = 0;

                let black_missing_pieces = Piece::get_missing_pieces(Player::Black, self.board);
                let white_missing_pieces = Piece::get_missing_pieces(Player::White, self.board);

                for piece in black_missing_pieces {
                    difference += Piece::get_value(piece) as i8;
                }

                for piece in white_missing_pieces {
                    difference -= Piece::get_value(piece) as i8;
                }

                if difference > 0 {
                    graphics::set_window_title(ctx, &format!("Chess! (+{})", difference));
                } else {
                    graphics::set_window_title(ctx, &format!("Chess! ({})", difference));
                }
            }
            MouseButton::Right => {
                self.draw_mode = false;
            }
            _ => (),
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == match_key(self.config.draw_clear_button.to_string()).unwrap_or(KeyCode::Space)
        {
            self.drawn_pixels = Vec::new();
        }
    }
}

pub fn run_game() -> GameResult {
    let config = load_config();

    let window = ggez::conf::WindowMode::default()
        .dimensions(config.window_size_horizontal, config.window_size_vertical);

    let window_setup = ggez::conf::WindowSetup::default()
        .title("Chess! (0)")
        .icon("/images/wp.png");

    let mut asset_path = path::PathBuf::from("./");
    asset_path.push("resources");

    let (ctx, event_loop) = ggez::ContextBuilder::new("Chess", "atomflunder")
        .window_setup(window_setup)
        .window_mode(window)
        .add_resource_path(asset_path)
        .build()?;

    let board = MainGame::new()?;

    event::run(ctx, event_loop, board)
}
