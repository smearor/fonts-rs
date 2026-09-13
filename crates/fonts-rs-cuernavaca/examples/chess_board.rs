//! Chess Board — Interactive chess game with Cuernavaca font pieces.
//!
//! Renders an 8x8 chess board with pieces from the Cuernavaca chess font.
//! Click to select a piece, then click a destination square to move.
//! Includes move validation, captured pieces display, move history,
//! and board flip.
//!
//! Run with:
//! ```sh
//! cargo run --example chess_board --features gtk,render,embed-fonts
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::OutlineCurve;

use fonts_rs_cuernavaca::fonts;
use fonts_rs_cuernavaca::register_glyphs;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box as GtkBox;
use gtk4::Button;
use gtk4::DrawingArea;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::cairo;
use gtk4::glib;
use gtk4::prelude::*;

use miette::IntoDiagnostic;
use miette::Result;

const APP_ID: &str = "io.smearor.fonts_rs.chess_board";

/// Board colors (RGB tuples).
const LIGHT_SQUARE: (f64, f64, f64) = (0.93, 0.85, 0.65);
const DARK_SQUARE: (f64, f64, f64) = (0.55, 0.40, 0.25);
const HIGHLIGHT: (f64, f64, f64, f64) = (0.20, 0.60, 0.20, 0.45);
const LAST_MOVE: (f64, f64, f64, f64) = (0.80, 0.70, 0.10, 0.40);
const CHECK_COLOR: (f64, f64, f64, f64) = (0.80, 0.15, 0.15, 0.50);

/// Piece types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

/// Piece color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

/// A chess piece.
#[derive(Clone, Copy, Debug)]
struct Piece {
    piece_type: PieceType,
    color: Color,
}

/// Square coordinates (0-based, from bottom-left).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Square {
    file: u8,
    rank: u8,
}

impl Square {
    fn new(file: u8, rank: u8) -> Self {
        Self { file, rank }
    }

    #[allow(dead_code)]
    fn from_algebraic(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }
        let file = bytes[0].checked_sub(b'a')?;
        let rank = bytes[1].checked_sub(b'1')?;
        if file < 8 && rank < 8 { Some(Self { file, rank }) } else { None }
    }

    fn to_algebraic(self) -> String {
        format!("{}{}", (b'a' + self.file) as char, (b'1' + self.rank) as char)
    }
}

/// A move record.
#[derive(Clone, Debug)]
struct Move {
    from: Square,
    #[allow(dead_code)]
    to: Square,
    piece: PieceType,
    #[allow(dead_code)]
    captured: Option<PieceType>,
    #[allow(dead_code)]
    promotion: Option<PieceType>,
    #[allow(dead_code)]
    is_castle: bool,
    #[allow(dead_code)]
    is_en_passant: bool,
    notation: String,
}

/// Game state.
struct GameState {
    board: [[Option<Piece>; 8]; 8],
    turn: Color,
    selected: Option<Square>,
    history: Vec<Move>,
    last_move: Option<(Square, Square)>,
    flipped: bool,
    en_passant_target: Option<Square>,
    white_captured: Vec<PieceType>,
    black_captured: Vec<PieceType>,
    game_over: bool,
    status_message: String,
}

impl GameState {
    fn new() -> Self {
        let mut board = [[None; 8]; 8];
        // White back rank (rank 0 = rank 1 in chess)
        let white_back = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];
        // Black back rank (rank 7 = rank 8 in chess)
        let black_back = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];
        for file in 0..8 {
            board[0][file] = Some(Piece {
                piece_type: white_back[file],
                color: Color::White,
            });
            board[1][file] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            });
            board[6][file] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            });
            board[7][file] = Some(Piece {
                piece_type: black_back[file],
                color: Color::Black,
            });
        }
        Self {
            board,
            turn: Color::White,
            selected: None,
            history: vec![],
            last_move: None,
            flipped: false,
            en_passant_target: None,
            white_captured: vec![],
            black_captured: vec![],
            game_over: false,
            status_message: "White to move".to_string(),
        }
    }

    /// Get the piece at a square.
    fn piece_at(&self, sq: Square) -> Option<Piece> {
        self.board[sq.rank as usize][sq.file as usize]
    }

    /// Check if a square is attacked by the given color.
    fn is_attacked(&self, sq: Square, by: Color) -> bool {
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.board[rank][file]
                    && piece.color == by
                {
                    let from = Square::new(file as u8, rank as u8);
                    if self.can_attack(from, sq, piece) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if a piece at `from` can attack `to` (without considering checks).
    fn can_attack(&self, from: Square, to: Square, piece: Piece) -> bool {
        let df = to.file as i8 - from.file as i8;
        let dr = to.rank as i8 - from.rank as i8;
        let abs_df = df.abs();
        let abs_dr = dr.abs();
        match piece.piece_type {
            PieceType::Pawn => {
                let dir = if piece.color == Color::White { 1 } else { -1 };
                abs_df == 1 && dr == dir
            }
            PieceType::Knight => (abs_df == 2 && abs_dr == 1) || (abs_df == 1 && abs_dr == 2),
            PieceType::Bishop => abs_df == abs_dr && abs_df > 0 && self.path_clear(from, to),
            PieceType::Rook => (abs_df == 0 || abs_dr == 0) && (abs_df + abs_dr > 0) && self.path_clear(from, to),
            PieceType::Queen => (abs_df == abs_dr || abs_df == 0 || abs_dr == 0) && (abs_df + abs_dr > 0) && self.path_clear(from, to),
            PieceType::King => abs_df <= 1 && abs_dr <= 1 && (abs_df + abs_dr > 0),
        }
    }

    /// Check if the path between two squares is clear (exclusive endpoints).
    fn path_clear(&self, from: Square, to: Square) -> bool {
        let df = to.file as i8 - from.file as i8;
        let dr = to.rank as i8 - from.rank as i8;
        let step_f = df.signum();
        let step_r = dr.signum();
        let steps = df.abs().max(dr.abs());
        for i in 1..steps {
            let f = (from.file as i8 + step_f * i) as u8;
            let r = (from.rank as i8 + step_r * i) as u8;
            if self.board[r as usize][f as usize].is_some() {
                return false;
            }
        }
        true
    }

    /// Get all legal moves for a piece at the given square.
    fn legal_moves(&self, from: Square) -> Vec<Square> {
        let Some(piece) = self.piece_at(from) else {
            return vec![];
        };
        if piece.color != self.turn {
            return vec![];
        }
        let mut moves = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                let to = Square::new(file, rank);
                if self.is_legal_move(from, to) {
                    moves.push(to);
                }
            }
        }
        moves
    }

    /// Check if a move is legal (including check validation).
    fn is_legal_move(&self, from: Square, to: Square) -> bool {
        let Some(piece) = self.piece_at(from) else {
            return false;
        };
        if piece.color != self.turn {
            return false;
        }
        if let Some(target) = self.piece_at(to)
            && target.color == piece.color
        {
            return false;
        }
        if !self.is_pseudo_legal(from, to, piece) {
            return false;
        }
        // Simulate the move and check if own king is in check
        let mut sim = self.clone_board();
        sim.execute_move(from, to, piece);
        !sim.is_in_check(piece.color)
    }

    /// Check pseudo-legal move validity (without check validation).
    fn is_pseudo_legal(&self, from: Square, to: Square, piece: Piece) -> bool {
        let df = to.file as i8 - from.file as i8;
        let dr = to.rank as i8 - from.rank as i8;
        let abs_df = df.abs();
        let abs_dr = dr.abs();
        match piece.piece_type {
            PieceType::Pawn => {
                let dir = if piece.color == Color::White { 1 } else { -1 };
                let start_rank = if piece.color == Color::White { 1 } else { 6 };
                // Forward one square
                if df == 0 && dr == dir && self.piece_at(to).is_none() {
                    return true;
                }
                // Forward two squares from starting rank
                if df == 0 && dr == 2 * dir && from.rank == start_rank && self.piece_at(to).is_none() {
                    let mid_rank = (from.rank as i8 + dir) as u8;
                    let mid = Square::new(from.file, mid_rank);
                    if self.piece_at(mid).is_none() {
                        return true;
                    }
                }
                // Capture
                if abs_df == 1 && dr == dir {
                    if self.piece_at(to).map(|p| p.color != piece.color).unwrap_or(false) {
                        return true;
                    }
                    // En passant
                    if let Some(ep) = self.en_passant_target
                        && to == ep
                    {
                        return true;
                    }
                }
                false
            }
            PieceType::Knight => (abs_df == 2 && abs_dr == 1) || (abs_df == 1 && abs_dr == 2),
            PieceType::Bishop => abs_df == abs_dr && abs_df > 0 && self.path_clear(from, to),
            PieceType::Rook => (abs_df == 0 || abs_dr == 0) && (abs_df + abs_dr > 0) && self.path_clear(from, to),
            PieceType::Queen => (abs_df == abs_dr || abs_df == 0 || abs_dr == 0) && (abs_df + abs_dr > 0) && self.path_clear(from, to),
            PieceType::King => {
                // Normal king move
                if abs_df <= 1 && abs_dr <= 1 && (abs_df + abs_dr > 0) {
                    return true;
                }
                // Castling
                if abs_dr == 0 && abs_df == 2 && from.rank == if piece.color == Color::White { 0 } else { 7 } {
                    return self.can_castle(from, to, piece.color);
                }
                false
            }
        }
    }

    /// Check castling availability.
    fn can_castle(&self, from: Square, to: Square, color: Color) -> bool {
        let king_rank = if color == Color::White { 0 } else { 7 };
        if from.rank != king_rank || from.file != 4 {
            return false;
        }
        let kingside = to.file == 6;
        let queenside = to.file == 2;
        if !kingside && !queenside {
            return false;
        }
        // King hasn't moved (check history)
        for m in &self.history {
            if m.piece == PieceType::King && m.from == Square::new(4, king_rank) {
                return false;
            }
        }
        // Rook hasn't moved
        let rook_file = if kingside { 7 } else { 0 };
        for m in &self.history {
            if m.piece == PieceType::Rook && m.from == Square::new(rook_file, king_rank) {
                return false;
            }
        }
        // Squares between king and rook are empty
        if kingside {
            if self.piece_at(Square::new(5, king_rank)).is_some() || self.piece_at(Square::new(6, king_rank)).is_some() {
                return false;
            }
        } else {
            if self.piece_at(Square::new(1, king_rank)).is_some()
                || self.piece_at(Square::new(2, king_rank)).is_some()
                || self.piece_at(Square::new(3, king_rank)).is_some()
            {
                return false;
            }
        }
        // King is not in check, and doesn't pass through check
        let opponent = if color == Color::White { Color::Black } else { Color::White };
        if self.is_attacked(Square::new(4, king_rank), opponent) {
            return false;
        }
        let pass_sq = if kingside { Square::new(5, king_rank) } else { Square::new(3, king_rank) };
        if self.is_attacked(pass_sq, opponent) {
            return false;
        }
        if self.is_attacked(to, opponent) {
            return false;
        }
        true
    }

    /// Clone the board state (shallow copy of arrays).
    fn clone_board(&self) -> GameState {
        GameState {
            board: self.board,
            turn: self.turn,
            selected: self.selected,
            history: self.history.clone(),
            last_move: self.last_move,
            flipped: self.flipped,
            en_passant_target: self.en_passant_target,
            white_captured: self.white_captured.clone(),
            black_captured: self.black_captured.clone(),
            game_over: self.game_over,
            status_message: self.status_message.clone(),
        }
    }

    /// Execute a move on the board (no validation).
    fn execute_move(&mut self, from: Square, to: Square, piece: Piece) {
        let mut captured = self.piece_at(to).map(|p| p.piece_type);
        let mut is_en_passant = false;
        // En passant capture
        if piece.piece_type == PieceType::Pawn && to == self.en_passant_target.unwrap_or(Square::new(0, 9)) {
            let ep_rank = if piece.color == Color::White { to.rank - 1 } else { to.rank + 1 };
            captured = self.piece_at(Square::new(to.file, ep_rank)).map(|p| p.piece_type);
            self.board[ep_rank as usize][to.file as usize] = None;
            is_en_passant = true;
        }
        // Move the piece
        self.board[from.rank as usize][from.file as usize] = None;
        // Promotion to queen
        let moved_piece = if piece.piece_type == PieceType::Pawn {
            let promotion_rank = if piece.color == Color::White { 7 } else { 0 };
            if to.rank == promotion_rank {
                Some(Piece {
                    piece_type: PieceType::Queen,
                    color: piece.color,
                })
            } else {
                Some(piece)
            }
        } else {
            Some(piece)
        };
        self.board[to.rank as usize][to.file as usize] = moved_piece;
        // Castling: move the rook
        let is_castle = piece.piece_type == PieceType::King && (to.file as i8 - from.file as i8).abs() == 2;
        if is_castle {
            let king_rank = from.rank;
            if to.file == 6 {
                // Kingside
                let rook = self.board[king_rank as usize][7];
                self.board[king_rank as usize][7] = None;
                self.board[king_rank as usize][5] = rook;
            } else if to.file == 2 {
                // Queenside
                let rook = self.board[king_rank as usize][0];
                self.board[king_rank as usize][0] = None;
                self.board[king_rank as usize][3] = rook;
            }
        }
        // Update en passant target
        if piece.piece_type == PieceType::Pawn && (to.rank as i8 - from.rank as i8).abs() == 2 {
            let ep_rank = (from.rank + to.rank) / 2;
            self.en_passant_target = Some(Square::new(from.file, ep_rank));
        } else {
            self.en_passant_target = None;
        }
        // Record captured piece
        if let Some(cap) = captured {
            if piece.color == Color::White {
                self.white_captured.push(cap);
            } else {
                self.black_captured.push(cap);
            }
        }
        // Build notation
        let notation = self.build_notation(from, to, piece, captured.is_some(), is_castle, is_en_passant);
        self.history.push(Move {
            from,
            to,
            piece: piece.piece_type,
            captured,
            promotion: if moved_piece.map(|p| p.piece_type) == Some(PieceType::Queen) && piece.piece_type == PieceType::Pawn {
                Some(PieceType::Queen)
            } else {
                None
            },
            is_castle,
            is_en_passant,
            notation,
        });
        self.last_move = Some((from, to));
        self.turn = if self.turn == Color::White { Color::Black } else { Color::White };
    }

    /// Build algebraic notation for a move.
    fn build_notation(&self, from: Square, to: Square, piece: Piece, captured: bool, castle: bool, _ep: bool) -> String {
        if castle {
            if to.file == 6 {
                return "O-O".to_string();
            } else {
                return "O-O-O".to_string();
            }
        }
        let piece_letter = match piece.piece_type {
            PieceType::King => "K",
            PieceType::Queen => "Q",
            PieceType::Rook => "R",
            PieceType::Bishop => "B",
            PieceType::Knight => "N",
            PieceType::Pawn => "",
        };
        let capture = if captured { "x" } else { "" };
        let from_file = if piece.piece_type == PieceType::Pawn && captured {
            format!("{}", (b'a' + from.file) as char)
        } else {
            String::new()
        };
        let promotion = if piece.piece_type == PieceType::Pawn {
            let promo_rank = if piece.color == Color::White { 7 } else { 0 };
            if to.rank == promo_rank { "=Q" } else { "" }
        } else {
            ""
        };
        format!("{piece_letter}{from_file}{capture}{}{promotion}", to.to_algebraic())
    }

    /// Find the king of the given color.
    fn find_king(&self, color: Color) -> Option<Square> {
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(p) = self.board[rank][file]
                    && p.piece_type == PieceType::King
                    && p.color == color
                {
                    return Some(Square::new(file as u8, rank as u8));
                }
            }
        }
        None
    }

    /// Check if the given color's king is in check.
    fn is_in_check(&self, color: Color) -> bool {
        let Some(king_sq) = self.find_king(color) else {
            return false;
        };
        let opponent = if color == Color::White { Color::Black } else { Color::White };
        self.is_attacked(king_sq, opponent)
    }

    /// Check if the given color has any legal moves.
    fn has_legal_moves(&self, color: Color) -> bool {
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.board[rank][file]
                    && piece.color == color
                {
                    let from = Square::new(file as u8, rank as u8);
                    for tr in 0..8 {
                        for tf in 0..8 {
                            if self.is_legal_move(from, Square::new(tf as u8, tr as u8)) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Check game status (checkmate, stalemate, etc.).
    fn check_game_status(&mut self) {
        let in_check = self.is_in_check(self.turn);
        if !self.has_legal_moves(self.turn) {
            self.game_over = true;
            if in_check {
                let winner = if self.turn == Color::White { "Black" } else { "White" };
                self.status_message = format!("Checkmate! {} wins", winner);
            } else {
                self.status_message = "Stalemate! Draw".to_string();
            }
        } else if in_check {
            self.status_message = format!("{:?} in check", self.turn);
        } else {
            self.status_message = format!("{:?} to move", self.turn);
        }
    }
}

/// Map a piece to the Cuernavaca font character.
fn piece_to_char(piece: Piece) -> char {
    match (piece.color, piece.piece_type) {
        (Color::White, PieceType::Rook) => 'R',
        (Color::White, PieceType::Knight) => 'N',
        (Color::White, PieceType::Bishop) => 'B',
        (Color::White, PieceType::Queen) => 'Q',
        (Color::White, PieceType::King) => 'K',
        (Color::White, PieceType::Pawn) => 'P',
        (Color::Black, PieceType::Rook) => 'T',
        (Color::Black, PieceType::Knight) => 'M',
        (Color::Black, PieceType::Bishop) => 'V',
        (Color::Black, PieceType::Queen) => 'W',
        (Color::Black, PieceType::King) => 'L',
        (Color::Black, PieceType::Pawn) => 'O',
    }
}

/// Trace a glyph's outline curves onto the cairo context at (x, y) with given size.
/// Detects new contours by checking if the start point of a curve differs from
/// the end point of the previous curve.
fn trace_glyph_path(cr: &cairo::Context, font: &FontVec, ch: char, x: f64, y: f64, size: f64) -> bool {
    let glyph_id = font.glyph_id(ch);
    let outline = match font.outline(glyph_id) {
        Some(o) => o,
        None => return false,
    };
    let upem = font.units_per_em().unwrap_or(1000.0);
    let scale = size as f32 / upem;
    let scale_x = scale;
    let scale_y = -scale;
    let bounds = &outline.bounds;
    let offset_x = x - bounds.min.x as f64 * scale_x as f64;
    let offset_y = y - bounds.min.y as f64 * scale_y as f64;
    cr.translate(offset_x, offset_y);

    let sx = |p: &ab_glyph::Point| p.x as f64 * scale_x as f64;
    let sy = |p: &ab_glyph::Point| p.y as f64 * scale_y as f64;

    let mut started = false;
    let mut last_x = 0.0;
    let mut last_y = 0.0;

    for curve in outline.curves.iter() {
        let (p1_start, end_x, end_y, draw): (ab_glyph::Point, f64, f64, &dyn Fn(&cairo::Context)) = match curve {
            OutlineCurve::Line(p1, p2) => (*p1, sx(p2), sy(p2), &|cr: &cairo::Context| {
                cr.line_to(sx(p2), sy(p2));
            }),
            OutlineCurve::Quad(p1, p2, p3) => (*p1, sx(p3), sy(p3), &|cr: &cairo::Context| {
                let x1 = sx(p1);
                let y1 = sy(p1);
                let x2 = sx(p2);
                let y2 = sy(p2);
                let x3 = sx(p3);
                let y3 = sy(p3);
                let c1x = x1 + (2.0 / 3.0) * (x2 - x1);
                let c1y = y1 + (2.0 / 3.0) * (y2 - y1);
                let c2x = x3 + (2.0 / 3.0) * (x2 - x3);
                let c2y = y3 + (2.0 / 3.0) * (y2 - y3);
                cr.curve_to(c1x, c1y, c2x, c2y, x3, y3);
            }),
            OutlineCurve::Cubic(p1, p2, p3, p4) => (*p1, sx(p4), sy(p4), &|cr: &cairo::Context| {
                cr.curve_to(sx(p2), sy(p2), sx(p3), sy(p3), sx(p4), sy(p4));
            }),
        };

        let start_x = sx(&p1_start);
        let start_y = sy(&p1_start);

        if !started {
            cr.move_to(start_x, start_y);
            started = true;
        } else if (start_x - last_x).abs() > 0.01 || (start_y - last_y).abs() > 0.01 {
            cr.close_path();
            cr.move_to(start_x, start_y);
        }

        draw(cr);
        last_x = end_x;
        last_y = end_y;
    }

    if started {
        cr.close_path();
    }
    started
}

/// Draw a filled glyph using EvenOdd fill rule (correct for chess piece outlines with internal cutouts).
fn draw_glyph_with_outline(
    cr: &cairo::Context,
    font: &FontVec,
    ch: char,
    x: f64,
    y: f64,
    size: f64,
    fill_color: (f64, f64, f64),
    _outline_color: (f64, f64, f64),
) {
    cr.save().unwrap_or(());
    cr.set_fill_rule(cairo::FillRule::EvenOdd);
    if trace_glyph_path(cr, font, ch, x, y, size) {
        cr.set_source_rgb(fill_color.0, fill_color.1, fill_color.2);
        cr.fill().unwrap_or(());
    }
    cr.restore().unwrap_or(());
}

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    register_glyphs().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    Ok(app.run())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Chess Board — Cuernavaca")
        .default_width(900)
        .default_height(700)
        .build();

    let css = r#"
        .chess-title { font-size: 18px; font-weight: bold; }
        .chess-status { font-size: 14px; padding: 8px; }
        .chess-history { font-family: monospace; font-size: 12px; }
        .chess-btn { padding: 8px 16px; }
        .chess-panel { background-color: #f0f0f0; padding: 12px; }
        .chess-captured { font-size: 20px; }
    "#;
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let main_box = GtkBox::builder().orientation(Orientation::Horizontal).spacing(0).build();

    // --- Board area ---
    let drawing_area = DrawingArea::builder()
        .width_request(640)
        .height_request(640)
        .hexpand(true)
        .vexpand(true)
        .build();

    // --- Side panel ---
    let panel = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .css_classes(["chess-panel"])
        .width_request(260)
        .build();

    let title = Label::builder()
        .label("Chess — Cuernavaca")
        .css_classes(["chess-title"])
        .halign(Align::Center)
        .build();
    panel.append(&title);

    let status_label = Label::builder()
        .label("White to move")
        .css_classes(["chess-status"])
        .halign(Align::Center)
        .build();
    panel.append(&status_label);

    let captured_white = Label::builder().label("").css_classes(["chess-captured"]).halign(Align::Start).build();
    let captured_black = Label::builder().label("").css_classes(["chess-captured"]).halign(Align::Start).build();
    panel.append(&Label::builder().label("Captured by White:").halign(Align::Start).build());
    panel.append(&captured_white);
    panel.append(&Label::builder().label("Captured by Black:").halign(Align::Start).build());
    panel.append(&captured_black);

    let history_label = Label::builder()
        .label("Move History")
        .css_classes(["chess-title"])
        .halign(Align::Center)
        .build();
    panel.append(&history_label);

    let history_text = gtk4::TextView::builder().editable(false).css_classes(["chess-history"]).vexpand(true).build();
    let history_scroll = gtk4::ScrolledWindow::builder().vexpand(true).hexpand(true).child(&history_text).build();
    panel.append(&history_scroll);

    let btn_box = GtkBox::builder().orientation(Orientation::Horizontal).spacing(8).halign(Align::Center).build();
    let new_game_btn = Button::builder().label("New Game").css_classes(["chess-btn"]).build();
    let flip_btn = Button::builder().label("Flip Board").css_classes(["chess-btn"]).build();
    btn_box.append(&new_game_btn);
    btn_box.append(&flip_btn);
    panel.append(&btn_box);

    main_box.append(&drawing_area);
    main_box.append(&panel);
    window.set_child(Some(&main_box));

    // --- State ---
    let state: Rc<RefCell<GameState>> = Rc::new(RefCell::new(GameState::new()));
    let font = fonts::font().expect("Failed to load Cuernavaca font");

    // --- Drawing ---
    {
        let state_ref = state.clone();
        let status_ref = status_label.clone();
        let captured_white_ref = captured_white.clone();
        let captured_black_ref = captured_black.clone();
        let history_ref = history_text.clone();
        drawing_area.set_draw_func(move |_area, cr, width, height| {
            let s = state_ref.borrow();
            let board_size = width.min(height) as f64 - 40.0;
            let board_x = (width as f64 - board_size) / 2.0;
            let board_y = (height as f64 - board_size) / 2.0;
            let cell = board_size / 8.0;
            let piece_size = cell * 0.85;

            // Draw board squares
            for rank in 0..8u8 {
                for file in 0..8u8 {
                    let display_rank = if s.flipped { 7 - rank } else { rank };
                    let display_file = if s.flipped { 7 - file } else { file };
                    let x = board_x + display_file as f64 * cell;
                    let y = board_y + (7 - display_rank) as f64 * cell;
                    let is_light = (rank + file) % 2 != 0;
                    let color = if is_light { LIGHT_SQUARE } else { DARK_SQUARE };
                    cr.set_source_rgb(color.0, color.1, color.2);
                    cr.rectangle(x, y, cell, cell);
                    cr.fill().unwrap_or(());

                    // Highlight last move
                    if let Some((from, to)) = s.last_move {
                        let sq = Square::new(file, rank);
                        if sq == from || sq == to {
                            cr.set_source_rgba(LAST_MOVE.0, LAST_MOVE.1, LAST_MOVE.2, LAST_MOVE.3);
                            cr.rectangle(x, y, cell, cell);
                            cr.fill().unwrap_or(());
                        }
                    }

                    // Highlight selected square
                    if let Some(sel) = s.selected
                        && Square::new(file, rank) == sel
                    {
                        cr.set_source_rgba(HIGHLIGHT.0, HIGHLIGHT.1, HIGHLIGHT.2, HIGHLIGHT.3);
                        cr.rectangle(x, y, cell, cell);
                        cr.fill().unwrap_or(());
                    }

                    // Highlight legal moves
                    if let Some(sel) = s.selected {
                        let legal = s.legal_moves(sel);
                        for mv in &legal {
                            if *mv == Square::new(file, rank) {
                                if s.piece_at(*mv).is_some() {
                                    // Capture: draw ring
                                    cr.set_source_rgba(0.8, 0.2, 0.2, 0.5);
                                    cr.set_line_width(3.0);
                                    cr.arc(x + cell / 2.0, y + cell / 2.0, cell * 0.42, 0.0, std::f64::consts::TAU);
                                    cr.stroke().unwrap_or(());
                                } else {
                                    // Move: draw dot
                                    cr.set_source_rgba(0.2, 0.5, 0.2, 0.4);
                                    cr.arc(x + cell / 2.0, y + cell / 2.0, cell * 0.15, 0.0, std::f64::consts::TAU);
                                    cr.fill().unwrap_or(());
                                }
                            }
                        }
                    }

                    // Highlight king in check
                    if s.is_in_check(s.turn)
                        && let Some(king_sq) = s.find_king(s.turn)
                        && Square::new(file, rank) == king_sq
                    {
                        cr.set_source_rgba(CHECK_COLOR.0, CHECK_COLOR.1, CHECK_COLOR.2, CHECK_COLOR.3);
                        cr.rectangle(x, y, cell, cell);
                        cr.fill().unwrap_or(());
                    }
                }
            }

            // Draw coordinates
            cr.set_source_rgb(0.3, 0.3, 0.3);
            cr.set_font_size(11.0);
            for i in 0..8u8 {
                let file_label = if s.flipped { 7 - i } else { i };
                let rank_label = if s.flipped { i } else { 7 - i };
                let x_file = board_x + i as f64 * cell + cell / 2.0 - 4.0;
                cr.move_to(x_file, board_y + board_size + 15.0);
                cr.show_text(&format!("{}", (b'a' + file_label) as char)).unwrap_or(());
                let y_rank = board_y + i as f64 * cell + cell / 2.0 + 4.0;
                cr.move_to(board_x - 18.0, y_rank);
                cr.show_text(&format!("{}", rank_label + 1)).unwrap_or(());
            }

            // Draw pieces
            for rank in 0..8u8 {
                for file in 0..8u8 {
                    if let Some(piece) = s.board[rank as usize][file as usize] {
                        let display_rank = if s.flipped { 7 - rank } else { rank };
                        let display_file = if s.flipped { 7 - file } else { file };
                        let x = board_x + display_file as f64 * cell + (cell - piece_size) / 2.0;
                        let y = board_y + (7 - display_rank) as f64 * cell + (cell - piece_size) / 2.0;
                        let ch = piece_to_char(piece);
                        let color = if piece.color == Color::White {
                            (0.95, 0.95, 0.92)
                        } else {
                            (0.12, 0.12, 0.12)
                        };
                        // Draw outline (contrast color) then fill (piece color)
                        let outline_color = if piece.color == Color::White {
                            (0.12, 0.12, 0.12)
                        } else {
                            (0.95, 0.95, 0.92)
                        };
                        draw_glyph_with_outline(cr, font, ch, x, y, piece_size, color, outline_color);
                    }
                }
            }

            // Update side panel
            let status_text = s.status_message.clone();
            status_ref.set_label(&status_text);
            let white_caps: String = s
                .white_captured
                .iter()
                .map(|p| {
                    piece_to_char(Piece {
                        piece_type: *p,
                        color: Color::Black,
                    })
                })
                .collect();
            let black_caps: String = s
                .black_captured
                .iter()
                .map(|p| {
                    piece_to_char(Piece {
                        piece_type: *p,
                        color: Color::White,
                    })
                })
                .collect();
            captured_white_ref.set_label(&white_caps);
            captured_black_ref.set_label(&black_caps);

            // Update history
            let history = &s.history;
            let mut text = String::new();
            for (i, m) in history.iter().enumerate() {
                if i % 2 == 0 {
                    text.push_str(&format!("{}. {} ", i / 2 + 1, m.notation));
                } else {
                    text.push_str(&format!("{}  \n", m.notation));
                }
            }
            history_ref.buffer().set_text(&text);
        });
    }

    // --- Click handling ---
    {
        let state_ref = state.clone();
        let drawing_ref = drawing_area.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.connect_pressed(move |_gesture, _n, x, y| {
            let mut s = state_ref.borrow_mut();
            if s.game_over {
                return;
            }
            let width = drawing_ref.width() as f64;
            let height = drawing_ref.height() as f64;
            let board_size = width.min(height) - 40.0;
            let board_x = (width - board_size) / 2.0;
            let board_y = (height - board_size) / 2.0;
            let cell = board_size / 8.0;
            if x < board_x || x > board_x + board_size || y < board_y || y > board_y + board_size {
                return;
            }
            let display_file = ((x - board_x) / cell) as u8;
            let display_rank = 7 - ((y - board_y) / cell) as u8;
            let file = if s.flipped { 7 - display_file } else { display_file };
            let rank = if s.flipped { 7 - display_rank } else { display_rank };
            let clicked = Square::new(file, rank);

            if let Some(sel) = s.selected {
                if sel == clicked {
                    // Deselect
                    s.selected = None;
                } else if s.is_legal_move(sel, clicked) {
                    // Make the move
                    let piece = s.piece_at(sel).unwrap();
                    s.execute_move(sel, clicked, piece);
                    s.selected = None;
                    s.check_game_status();
                } else {
                    // Select new piece
                    if let Some(p) = s.piece_at(clicked) {
                        if p.color == s.turn {
                            s.selected = Some(clicked);
                        } else {
                            s.selected = None;
                        }
                    } else {
                        s.selected = None;
                    }
                }
            } else {
                // Select a piece
                if let Some(p) = s.piece_at(clicked)
                    && p.color == s.turn
                {
                    s.selected = Some(clicked);
                }
            }
            drawing_ref.queue_draw();
        });
        drawing_area.add_controller(gesture);
    }

    // --- New game button ---
    {
        let state_ref = state.clone();
        let drawing_ref = drawing_area.clone();
        new_game_btn.connect_clicked(move |_| {
            *state_ref.borrow_mut() = GameState::new();
            drawing_ref.queue_draw();
        });
    }

    // --- Flip board button ---
    {
        let state_ref = state.clone();
        let drawing_ref = drawing_area.clone();
        flip_btn.connect_clicked(move |_| {
            state_ref.borrow_mut().flipped = !state_ref.borrow().flipped;
            drawing_ref.queue_draw();
        });
    }

    window.present();
}
