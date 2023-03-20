use rand::Rng;
use std::cmp;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub x:usize,
    pub y:usize
}

impl Piece {

    pub fn new(x_pos:usize, y_pos:usize) -> Piece {

        Piece {
            x:x_pos,
            y:y_pos
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub x:usize,
    pub y:usize,
    pub piece:Option<Piece>
}

impl Pos {

    pub fn new(x_pos:usize, y_pos:usize, curr_piece:Option<Piece>) -> Pos {

        Pos {
            x:x_pos,
            y:y_pos,
            piece:curr_piece
        }
    }
}

pub struct Board {

    pub row:usize,
    pub col:usize,

    pub my_side:i32,
    pub op_side:i32,

    pub curr_player:i32,

    pub state:Vec<Vec<i32>>,

    pub my_pieces:Vec<Pos>,
    pub op_pieces:Vec<Pos>,

    pub cache_table:HashMap<String, Vec<usize>>,
    pub minax_cache:HashMap<String, i32>,

    pub quix:usize
}

impl Board {

    pub fn new() -> Board {

        Board {
            row:8,
            col:8,

            my_side:1,
            op_side:-1,

            curr_player:1,

            state:Vec::new(),

            my_pieces:Vec::new(),
            op_pieces:Vec::new(),

            cache_table:HashMap::new(),
            minax_cache:HashMap::new(),

            quix:0
        }
    }

    pub fn init(&mut self) {

        for i in 0..self.row {

            let mut row:Vec<i32> = Vec::new();

            for j in 0..self.col {

                let even:bool = (i % 2 == 0) && (j % 2 == 0);
                let odd:bool = (i % 2 != 0) && (j % 2 != 0);

                if even || odd || i == 3 || i == 4 {
                    row.push(0);
                } else {

                    let mut _piece:Pos = Pos::new(i, j, None);

                    let x:i32 = i as i32;

                    if 0 <= x && x <= 2 {
                        row.push(self.op_side);
                        self.op_pieces.push(_piece);
                    } else {
                        row.push(self.my_side);
                        self.my_pieces.push(_piece);
                    }
                }
            }

            self.state.push(row);
        }
    }

    fn clone_board(&mut self) -> Board {
        
        let mut _board:Board = Board::new();

        _board.init();

        _board.state = self.state.clone();
        _board.curr_player = self.curr_player;

        _board.my_pieces = self.my_pieces.clone();
        _board.op_pieces = self.op_pieces.clone();

        return _board;
    }

    pub fn do_move(&mut self, mut piece:Pos, next_pos:Pos) {

        let checkas_len:i32;

        if self.state[piece.x][piece.y] == self.my_side {
            checkas_len = self.my_pieces.len() as i32;
        } else {
            checkas_len = self.op_pieces.len() as i32;
        }

        let (legal, took) = self.is_move_legal(piece, next_pos);

        if legal {

            let checkas_new_len:i32;

            if self.state[piece.x][piece.y] == self.my_side {
                checkas_new_len = self.my_pieces.len() as i32;
            } else {
                checkas_new_len = self.op_pieces.len() as i32;
            }

            if checkas_len == checkas_new_len {
                self.scan_board(piece);
            }

            if next_pos.x == 0 && self.state[piece.x][piece.y] == self.my_side {
                self.state[next_pos.x][next_pos.y] = 2;
            } else if next_pos.x == 7 && self.state[piece.x][piece.y] == self.op_side {
                self.state[next_pos.x][next_pos.y] = -2;
            } else {
                self.state[next_pos.x][next_pos.y] = self.state[piece.x][piece.y];
            }

            self.state[piece.x][piece.y] = 0;

            if took {

                let poss_moves:Vec<Pos> = self.possible_moves(next_pos, None);

                let mut new_pos:Pos = Pos::new(0, 0, None);

                let mut found:bool = false;

                for i in 0..poss_moves.len() {

                    if let Some(p) = poss_moves[i].piece {

                        found = true;

                        piece = next_pos;

                        new_pos = Pos::new(p.x, p.y, None);

                        break;
                    }
                }

                if found {
                    self.do_move(piece, new_pos);
                }
            }
        }

        self.curr_player = -self.curr_player;
    }

    fn take_piece(&mut self, x_pos:usize, y_pos:usize) {
        self.state[x_pos][y_pos] = 0;
    }

    fn is_move_legal(&mut self, piece:Pos, next_pos:Pos) -> (bool, bool) {

        let mut legal:bool = false;
        let mut took_piece:bool = false;

        let poss_moves:Vec<Pos> = self.possible_moves(piece, None);

        if poss_moves.len() > 0 {

            for i in 0..poss_moves.len() {

                let x_pos:usize = poss_moves[i].x;
                let y_pos:usize = poss_moves[i].y;

                if x_pos == next_pos.x && y_pos == next_pos.y {

                    if let Some(p) = poss_moves[i].piece {

                        self.take_piece(p.x, p.y);
                        took_piece = true;
                    }

                    legal = true;
                    break;
                }
            }
        }

        return (legal, took_piece);
    }

    fn scan_board(&mut self, piece:Pos) {

        let all_pieces:Vec<Pos> = self.all_movable_pieces(self.curr_player);

        let mut found:bool = false;

        for i in 0..all_pieces.len() {

            if found {
                break;
            }

            if all_pieces[i].x != piece.x && all_pieces[i].y != piece.y {

                let curr_piece:Pos = Pos::new(all_pieces[i].x, all_pieces[i].y, None);

                let moves:Vec<Pos> = self.possible_moves(curr_piece, None);

                for j in 0..moves.len() {

                    if let Some(_p) = moves[j].piece {

                        found = true;

                        self.take_piece(all_pieces[i].x, all_pieces[i].y);

                        break;
                    }
                }
            }
        }
    }

    fn check_all_pieces(&mut self) {

        self.op_pieces = Vec::new();
        self.my_pieces = Vec::new();

        for i in 0..self.row {
            for j in 0..self.col {

                if self.state[i][j] >= self.my_side {

                    let _piece:Pos = Pos::new(i, j, None);
                    self.my_pieces.push(_piece);

                } else if self.state[i][j] <= self.op_side {

                    let _piece:Pos = Pos::new(i, j, None);
                    self.op_pieces.push(_piece);
                }
            }
        }
    }

    pub fn all_movable_pieces(&mut self, side:i32) -> Vec<Pos> {

        self.check_all_pieces();

        let mut all_pieces:Vec<Pos> = Vec::new();

        if side == self.my_side {

            for i in 0..self.my_pieces.len() {

                let poss_moves:Vec<Pos> = self.possible_moves(self.my_pieces[i], None);

                if poss_moves.len() > 0 {
                    all_pieces.push(self.my_pieces[i]);
                }
            }

        } else {

            for i in 0..self.op_pieces.len() {

                let poss_moves:Vec<Pos> = self.possible_moves(self.op_pieces[i], None);

                if poss_moves.len() > 0 {
                    all_pieces.push(self.op_pieces[i]);
                }
            }
        }

        return all_pieces;
    }

    fn check_piece_history(&mut self, piece:Pos, history:&Vec<(usize, usize)>) -> bool {

        let mut been:bool = false;

        for i in 0..history.len() {

            let x_pos = history[i].0;
            let y_pos = history[i].1;

            if x_pos == piece.x && piece.y == y_pos {
                been = true;
                break;
            }
        }

        return been;
    }

    pub fn possible_moves(&mut self, piece:Pos, prev_pos:Option<&Vec<(usize, usize)>>) -> Vec<Pos> {

        let mut moves:Vec<Pos> = Vec::new();

        let x_pos = piece.x;
        let y_pos = piece.y;

        let mut empty:bool = true;

        let mut h:Vec<(usize, usize)> = Vec::new();

        if let Some(history) = prev_pos {
            empty = false;
            h = history.to_vec();
        }

        if self.check_upper_left(x_pos, y_pos).1 {

            if !empty {

                let new_pos:Pos = Pos::new(x_pos - 2, y_pos - 2, None);

                if !self.check_piece_history(new_pos, &h) {
                    if self.state[x_pos - 2][y_pos - 2] == 0 && self.state[x_pos - 1][y_pos - 1] != 0 && self.state[x_pos - 1][y_pos - 1] != self.state[x_pos][y_pos] {

                        let mut _x = (x_pos - 1) as i32;
                        let mut _y = (y_pos - 1) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let take_piece:Piece = Piece::new(_x as usize, _y as usize);

                        _x = (x_pos - 2) as i32;
                        _y = (y_pos - 2) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let poss_move:Pos = Pos::new(_x as usize, _y as usize, Some(take_piece));

                        moves.push(poss_move);
                        h.push((x_pos, y_pos));

                        let new_moves:Vec<Pos> = self.possible_moves(poss_move, Some(&h));

                        if new_moves.len() > 0 {
                            for i in 0..new_moves.len() {
                                moves.push(new_moves[i]);
                            }
                        }
                    }
                }
            } else {

                if self.state[x_pos - 2][y_pos - 2] == 0 && self.state[x_pos - 1][y_pos - 1] != 0 && self.state[x_pos - 1][y_pos - 1] != self.state[x_pos][y_pos] {
                    
                    let mut _x = (x_pos - 1) as i32;
                        let mut _y = (y_pos - 1) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let take_piece:Piece = Piece::new(_x as usize, _y as usize);

                        _x = (x_pos - 2) as i32;
                        _y = (y_pos - 2) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let poss_move:Pos = Pos::new(_x as usize, _y as usize, Some(take_piece));

                    moves.push(poss_move);

                    let mut his:Vec<(usize, usize)> = Vec::new();

                    his.push((x_pos, y_pos));

                    let new_moves:Vec<Pos> = self.possible_moves(poss_move, Some(&his));

                    if new_moves.len() > 0 {
                        for i in 0..new_moves.len() {
                            moves.push(new_moves[i]);
                        }
                    }
                }
            }
        }

        if self.check_upper_right(x_pos, y_pos).1 {

            if !empty {

                let new_pos:Pos = Pos::new(x_pos - 2, y_pos + 2, None);

                if !self.check_piece_history(new_pos, &h) {
                    if self.state[x_pos - 2][y_pos + 2] == 0 && self.state[x_pos - 1][y_pos + 1] != 0 && self.state[x_pos - 1][y_pos + 1] != self.state[x_pos][y_pos] {

                        let mut _x = (x_pos - 1) as i32;
                        let mut _y = (y_pos + 1) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let take_piece:Piece = Piece::new(_x as usize, _y as usize);

                        _x = (x_pos - 2) as i32;
                        _y = (y_pos + 2) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let poss_move:Pos = Pos::new(_x as usize, _y as usize, Some(take_piece));

                        moves.push(poss_move);
                        h.push((x_pos, y_pos));

                        let new_moves:Vec<Pos> = self.possible_moves(poss_move, Some(&h));

                        if new_moves.len() > 0 {
                            for i in 0..new_moves.len() {
                                moves.push(new_moves[i]);
                            }
                        }
                    }
                }
            } else {

                if self.state[x_pos - 2][y_pos + 2] == 0 && self.state[x_pos - 1][y_pos + 1] != 0 && self.state[x_pos - 1][y_pos + 1] != self.state[x_pos][y_pos] {

                    let mut _x = (x_pos - 1) as i32;
                    let mut _y = (y_pos + 1) as i32;

                    _x = _x.abs();
                    _y = _y.abs();

                    let take_piece:Piece = Piece::new(_x as usize, _y as usize);

                    _x = (x_pos - 2) as i32;
                    _y = (y_pos + 2) as i32;

                    _x = _x.abs();
                    _y = _y.abs();

                    let poss_move:Pos = Pos::new(_x as usize, _y as usize, Some(take_piece));

                    moves.push(poss_move);

                    let mut his:Vec<(usize, usize)> = Vec::new();

                    his.push((x_pos, y_pos));

                    let new_moves:Vec<Pos> = self.possible_moves(poss_move, Some(&his));

                    if new_moves.len() > 0 {
                        for i in 0..new_moves.len() {
                            moves.push(new_moves[i]);
                        }
                    }
                }
            }
        }

        if self.check_lower_left(x_pos, y_pos).1 {

            if !empty {

                let new_pos:Pos = Pos::new(x_pos + 2, y_pos - 2, None);

                if !self.check_piece_history(new_pos, &h) {
                    if self.state[x_pos + 2][y_pos - 2] == 0 && self.state[x_pos + 1][y_pos - 1] != 0 && self.state[x_pos + 1][y_pos - 1] != self.state[x_pos][y_pos] {

                        let mut _x = (x_pos + 1) as i32;
                        let mut _y = (y_pos - 1) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let take_piece:Piece = Piece::new(_x as usize, _y as usize);

                        _x = (x_pos + 2) as i32;
                        _y = (y_pos - 2) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let poss_move:Pos = Pos::new(_x as usize, _y as usize, Some(take_piece));

                        moves.push(poss_move);
                        h.push((x_pos, y_pos));

                        let new_moves:Vec<Pos> = self.possible_moves(poss_move, Some(&h));

                        if new_moves.len() > 0 {
                            for i in 0..new_moves.len() {
                                moves.push(new_moves[i]);
                            }
                        }
                    }
                }
            } else {

                if self.state[x_pos + 2][y_pos - 2] == 0 && self.state[x_pos + 1][y_pos - 1] != 0 && self.state[x_pos + 1][y_pos - 1] != self.state[x_pos][y_pos] {

                    let mut _x = (x_pos + 1) as i32;
                    let mut _y = (y_pos - 1) as i32;

                    _x = _x.abs();
                    _y = _y.abs();

                    let take_piece:Piece = Piece::new(_x as usize, _y as usize);

                    _x = (x_pos + 2) as i32;
                    _y = (y_pos - 2) as i32;

                    _x = _x.abs();
                    _y = _y.abs();

                    let poss_move:Pos = Pos::new(_x as usize, _y as usize, Some(take_piece));

                    moves.push(poss_move);

                    let mut his:Vec<(usize, usize)> = Vec::new();

                    his.push((x_pos, y_pos));

                    let new_moves:Vec<Pos> = self.possible_moves(poss_move, Some(&his));

                    if new_moves.len() > 0 {
                        for i in 0..new_moves.len() {
                            moves.push(new_moves[i]);
                        }
                    }
                }
            }
        }

        if self.check_lower_right(x_pos, y_pos).1 {

            if !empty {

                let new_pos:Pos = Pos::new(x_pos + 2, y_pos + 2, None);

                if !self.check_piece_history(new_pos, &h) {
                    if self.state[x_pos + 2][y_pos + 2] == 0 && self.state[x_pos + 1][y_pos + 1] != 0 && self.state[x_pos + 1][y_pos + 1] != self.state[x_pos][y_pos] {

                        let mut _x = (x_pos + 1) as i32;
                        let mut _y = (y_pos + 1) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let take_piece:Piece = Piece::new(_x as usize, _y as usize);

                        _x = (x_pos + 2) as i32;
                        _y = (y_pos + 2) as i32;

                        _x = _x.abs();
                        _y = _y.abs();

                        let poss_move:Pos = Pos::new(_x as usize, _y as usize, Some(take_piece));

                        moves.push(poss_move);
                        h.push((x_pos, y_pos));

                        let new_moves:Vec<Pos> = self.possible_moves(poss_move, Some(&h));

                        if new_moves.len() > 0 {
                            for i in 0..new_moves.len() {
                                moves.push(new_moves[i]);
                            }
                        }
                    }
                }
            } else {

                if self.state[x_pos + 2][y_pos + 2] == 0 && self.state[x_pos + 1][y_pos + 1] != 0 && self.state[x_pos + 1][y_pos + 1] != self.state[x_pos][y_pos] {

                    let mut _x = (x_pos + 1) as i32;
                    let mut _y = (y_pos + 1) as i32;

                    _x = _x.abs();
                    _y = _y.abs();

                    let take_piece:Piece = Piece::new(_x as usize, _y as usize);

                    _x = (x_pos + 2) as i32;
                    _y = (y_pos + 2) as i32;

                    _x = _x.abs();
                    _y = _y.abs();

                    let poss_move:Pos = Pos::new(_x as usize, _y as usize, Some(take_piece));

                    moves.push(poss_move);

                    let mut his:Vec<(usize, usize)> = Vec::new();

                    his.push((x_pos, y_pos));

                    let new_moves:Vec<Pos> = self.possible_moves(poss_move, Some(&his));

                    if new_moves.len() > 0 {
                        for i in 0..new_moves.len() {
                            moves.push(new_moves[i]);
                        }
                    }
                }
            }
        }

        if moves.len() == 0 {

            if self.check_lower_right(x_pos, y_pos).0 && empty {
                if self.state[x_pos + 1][y_pos + 1] == 0 {
                    if self.state[x_pos][y_pos] == self.op_side || self.state[x_pos][y_pos] > self.my_side {
                        
                        let poss_move:Pos = Pos::new(x_pos + 1, y_pos + 1, None);
                        moves.push(poss_move);
                    }
                }
            }

            if self.check_upper_right(x_pos, y_pos).0 && empty {
                if self.state[x_pos - 1][y_pos + 1] == 0 {
                    if self.state[x_pos][y_pos] == self.my_side || self.state[x_pos][y_pos] < self.op_side {
                        
                        let poss_move:Pos = Pos::new(x_pos - 1, y_pos + 1, None);
                        moves.push(poss_move);
                    }
                }
            }

            if self.check_upper_left(x_pos, y_pos).0 && empty {
                if self.state[x_pos - 1][y_pos - 1] == 0 {
                    if self.state[x_pos][y_pos] == self.my_side || self.state[x_pos][y_pos] < self.op_side {
                        
                        let poss_move:Pos = Pos::new(x_pos - 1, y_pos - 1, None);
                        moves.push(poss_move);
                    }
                }
            }

            if self.check_lower_left(x_pos, y_pos).0 && empty {
                if self.state[x_pos + 1][y_pos - 1] == 0 {
                    if self.state[x_pos][y_pos] == self.op_side || self.state[x_pos][y_pos] > self.my_side {
                        
                        let poss_move:Pos = Pos::new(x_pos + 1, y_pos - 1, None);
                        moves.push(poss_move);
                    }
                }
            }
        }

        return moves;
    }

    fn check_upper_left(&mut self, x:usize, y:usize) -> (bool, bool) {

        let mut one_sq:bool = false;
        let mut two_sq:bool = false;

        let x_pos:i32 = x as i32;
        let y_pos:i32 = y as i32;

        if (0 <= x_pos - 2) && (0 <= y_pos - 2) && (x_pos - 2 <= 7) && (y_pos - 2 <= 7) {
            two_sq = true;
        }

        if (0 <= x_pos - 1) && (0 <= y_pos - 1) && (x_pos - 1 <= 7) && (y_pos - 1 <= 7) {
            one_sq = true;
        }

        return (one_sq, two_sq);
    }

    fn check_upper_right(&mut self, x:usize, y:usize) -> (bool, bool) {

        let mut one_sq:bool = false;
        let mut two_sq:bool = false;

        let x_pos:i32 = x as i32;
        let y_pos:i32 = y as i32;

        if (0 <= x_pos - 2) && (0 <= y_pos + 2) && (x_pos - 2 <= 7) && (y_pos + 2 <= 7) {
            two_sq = true;
        }

        if (0 <= x_pos - 1) && (0 <= y_pos + 1) && (x_pos - 1 <= 7) && (y_pos + 1 <= 7) {
            one_sq = true;
        }

        return (one_sq, two_sq);
    }

    fn check_lower_left(&mut self, x:usize, y:usize) -> (bool, bool) {

        let mut one_sq:bool = false;
        let mut two_sq:bool = false;

        let x_pos:i32 = x as i32;
        let y_pos:i32 = y as i32;

        if (0 <= x_pos + 2) && (0 <= y_pos - 2) && (x_pos + 2 <= 7) && (y_pos - 2 <= 7) {
            two_sq = true;
        }

        if (0 <= x_pos + 1) && (0 <= y_pos - 1) && (x_pos + 1 <= 7) && (y_pos - 1 <= 7) {
            one_sq = true;
        }

        return (one_sq, two_sq);
    }
    
    fn check_lower_right(&mut self, x:usize, y:usize) -> (bool, bool) {

        let mut one_sq:bool = false;
        let mut two_sq:bool = false;

        let x_pos:i32 = x as i32;
        let y_pos:i32 = y as i32;

        if (0 <= x_pos + 2) && (0 <= y_pos + 2) && (x_pos + 2 <= 7) && (y_pos + 2 <= 7) {
            two_sq = true;
        }

        if (0 <= x_pos + 1) && (0 <= y_pos + 1) && (x_pos + 1 <= 7) && (y_pos + 1 <= 7) {
            one_sq = true;
        }

        return (one_sq, two_sq);
    }

    pub fn finish_state(&mut self) -> (bool, i32) {

        let mut finished:bool = false;
        let mut winner:i32 = 0;

        let my_pieces:Vec<Pos> = self.all_movable_pieces(self.my_side);
        let op_pieces:Vec<Pos> = self.all_movable_pieces(self.op_side);

        if self.my_pieces.len() < self.op_pieces.len() && self.op_pieces.len() == 0 {
            finished = true;
            winner = self.my_side;
        } else if self.my_pieces.len() > self.op_pieces.len() && self.my_pieces.len() == 0 {
            finished = true;
            winner = self.op_side;
        } else if my_pieces.len() == 0 {
            finished = true;
            winner = self.op_side;
        } else if op_pieces.len() == 0 {
            finished = true;
            winner = self.my_side;
        }

        return (finished, winner);
    }

    fn first_prior(&mut self) -> (Option<Pos>, Option<Pos>) {

        let mut _piece:Option<Pos> = None;
        let mut next_pos:Option<Pos> = None;

        let all_pieces:Vec<Pos> = self.all_movable_pieces(self.curr_player);

        for i in 0..all_pieces.len() {

            if let Some(_p) = _piece {
                break;
            }

            let poss_moves:Vec<Pos> = self.possible_moves(all_pieces[i], None);

            for j in 0..poss_moves.len() {

                if let Some(_m) = poss_moves[j].piece {

                    _piece = Some(all_pieces[i]);
                    next_pos = Some(poss_moves[j]);
                    break;
                }
            }
        }

        return (_piece, next_pos);
    }

    fn second_prior(&mut self) -> (Option<Pos>, Option<Pos>) {

        let mut _piece:Option<Pos> = None;
        let mut next_pos:Option<Pos> = None;

        let my_pieces:Vec<Pos> = self.all_movable_pieces(self.my_side);
        let op_pieces:Vec<Pos> = self.all_movable_pieces(self.op_side);

        for i in 0..my_pieces.len() {
            for j in 0..op_pieces.len() {

                if let Some(_p) = _piece {
                    break;
                }

                let my_moves:Vec<Pos> = self.possible_moves(my_pieces[i], None);
                let op_moves:Vec<Pos> = self.possible_moves(op_pieces[j], None);

                for x in 0..my_moves.len() {
                    for y in 0..op_moves.len() {

                        if my_moves[x].x == op_moves[y].x && my_moves[x].y == op_moves[y].y {

                            if self.curr_player == self.op_side {

                                if let Some(_m) = my_moves[x].piece {

                                    if let Some(_n) = op_moves[y].piece {

                                        _piece = Some(op_pieces[j]);
                                        next_pos = Some(op_moves[y]);

                                        break;
                                    }
                                }

                            } else if self.curr_player == self.my_side {

                                if let Some(_m) = op_moves[y].piece {

                                    if let Some(_n) = my_moves[x].piece {

                                        _piece = Some(my_pieces[i]);
                                        next_pos = Some(my_moves[x]);

                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return (_piece, next_pos);
    }

    fn side_prior(&mut self) -> (Option<Pos>, Option<Pos>) {

        let mut _piece:Option<Pos> = None;
        let mut next_pos:Option<Pos> = None;

        let all_pieces:Vec<Pos> = self.all_movable_pieces(self.curr_player);

        if all_pieces.len() > 0 {

            for i in 0..all_pieces.len() {

                if let Some(_p) = _piece {
                    break;
                }

                let poss_moves:Vec<Pos> = self.possible_moves(all_pieces[i], None);

                for j in 0..poss_moves.len() {

                    if poss_moves[j].y == 0 || poss_moves[j].y == 7 {

                        _piece = Some(all_pieces[i]);
                        next_pos = Some(poss_moves[j]);
                        break;
                    }
                }
            }
        }

        return (_piece, next_pos);
    }


    fn third_prior(&mut self) -> (Pos, Pos) {

        let mut _piece:Pos = Pos::new(0, 0, None);
        let mut next_pos:Pos = Pos::new(0, 0, None);

        let mut count:usize = 1;

        let mut rng = rand::thread_rng();

        loop {

            let mut found:bool = false;

            let all_pieces:Vec<Pos> = self.all_movable_pieces(self.curr_player);

            if all_pieces.len() > 0 {

                let idx:usize = rng.gen_range(0, all_pieces.len()) as usize;

                _piece = all_pieces[idx];

                let all_moves:Vec<Pos> = self.possible_moves(_piece, None);

                let mv_idx:usize = rng.gen_range(0, all_moves.len()) as usize;

                next_pos = all_moves[mv_idx];

                let op_pieces:Vec<Pos> = self.all_movable_pieces(-self.curr_player);

                for i in 0..op_pieces.len() {

                    let poss_moves:Vec<Pos> = self.possible_moves(op_pieces[i], None); 

                    for j in 0..poss_moves.len() {

                        if poss_moves[j].x == next_pos.x && poss_moves[j].y == next_pos.y {
                            found = true;
                        }
                    }
                
                }
                
            }

            if !found || count > all_pieces.len() || all_pieces.len() == 0 {
                break;
            }

            count += 1;
        }

        return (_piece, next_pos);
    }

    pub fn best_move(&mut self) -> (Pos, Pos) {

        let mut _piece:Pos = Pos::new(0, 0, None);
        let mut next_pos:Pos = Pos::new(0, 0, None);

        let (_p1, _n1) = self.first_prior();

        if let Some(_p) = _p1 {
            if let Some(_n) = _n1 {
                _piece = _p;
                next_pos = _n;
            }
        } else {

            let (_p2, _n2) = self.second_prior();

            if let Some(_p) = _p2 {
                if let Some(_n) = _n2 {
                    _piece = _p;
                    next_pos = _n;
                }
            } else {

                let (_p3, _n3) = self.side_prior();

                if let Some(_p) = _p3 {
                    if let Some(_n) = _n3 {
                        _piece = _p;
                        next_pos = _n;
                    }
                } else {

                    let (_p4, _n4) = self.third_prior();

                    _piece = _p4;
                    next_pos = _n4;
                }
            }
        }

        return (_piece, next_pos);
    }

    pub fn ultimate_move(&mut self) -> (Pos, Pos) {

        let mut best_score:i32 = -999999;

        let mut _piece:Pos = Pos::new(0, 0, None);
        let mut next_pos:Pos = Pos::new(0, 0, None);

        let all_pieces:Vec<Pos> = self.all_movable_pieces(self.curr_player);

        for i in 0..all_pieces.len() {

            let moves:Vec<(i32, Pos, Pos)> = self.get_poss_moves(all_pieces[i]);

            for j in 0..moves.len() {

                if moves[j].0 > best_score {
                    best_score = moves[j].0;
                    _piece = moves[j].1;
                    next_pos = moves[j].2;
                }
            }
        }

        //println!("Computer Move Score: {}", best_score);

        self.set_cache_state(_piece, next_pos);

        return (_piece, next_pos);
    }

    fn get_poss_moves(&mut self, _piece:Pos) -> Vec<(i32, Pos, Pos)> {

        let alpha:i32 = 999999;
        let beta:i32 = -999999;

        let mut all_scores:Vec<(i32, Pos, Pos)> = Vec::new();

        let poss_moves:Vec<Pos> = self.possible_moves(_piece, None);

        for i in 0..poss_moves.len() {

            let mut _score:i32 = 0;

            let score:Option<i32> = self.get_cache_score(&self.state.clone(), _piece, poss_moves[i]);

            match score {

                Some(_s) => {

                    all_scores.push((_s, _piece, poss_moves[i]));
                },

                None => {

                    let mut temp_board:Board = self.clone_board();

                    temp_board.do_move(_piece, poss_moves[i]);

                    _score = self.minimax(temp_board, 9, false, alpha, beta);

                    all_scores.push((_score, _piece, poss_moves[i]));

                    self.set_cache_score(&self.state.clone(), _piece, poss_moves[i], _score);
                },
            }

            //println!("Move Score: {}", _score);
        }

        return all_scores;
    }

    fn set_cache_state(&mut self, _piece:Pos, next_pos:Pos) {

        let flat_state:Vec<i32> = self.state.iter()
                                            .flat_map(|array| array.iter())
                                            .cloned()
                                            .collect();

        let mut hasher = DefaultHasher::new();

        flat_state.hash(&mut hasher);

        let str_hash = hasher.finish().to_string();

        self.cache_table.insert(str_hash, vec![_piece.x, _piece.y, next_pos.x, next_pos.y]);
    }

    pub fn get_cache_state(&mut self) -> (Option<Pos>, Option<Pos>) {

        let mut _piece:Option<Pos> = None;
        let mut next_pos:Option<Pos> = None;

        let flat_state:Vec<i32> = self.state.iter()
                                            .flat_map(|array| array.iter())
                                            .cloned()
                                            .collect();
        
        let mut hasher = DefaultHasher::new();

        flat_state.hash(&mut hasher);

        let hash_str = hasher.finish().to_string();

        match self.cache_table.get(&hash_str) {

            Some(_v) => {

                let v_arr:Vec<usize> = _v.to_vec();

                _piece = Some(Pos::new(v_arr[0], v_arr[1], None));
                next_pos = Some(Pos::new(v_arr[2], v_arr[3], None));
            },

            None => {
                self.do_nothing();
            }
        }

        return (_piece, next_pos);
    }

    fn get_cache_score(&mut self, state:&Vec<Vec<i32>>, _piece:Pos, next_pos:Pos) -> Option<i32> {

        let mut score:Option<i32> = None;

        let moves:Vec<i32> = vec![_piece.x as i32, _piece.y as i32, next_pos.x as i32, next_pos.y as i32];

        let mut flat_state:Vec<i32> = state.iter()
                                            .flat_map(|array| array.iter())
                                            .cloned()
                                            .collect();

        for i in 0..moves.len() {
            flat_state.push(moves[i]);
        }

        let mut hasher = DefaultHasher::new();

        flat_state.hash(&mut hasher);

        let hash_str = hasher.finish().to_string();

        match self.minax_cache.get(&hash_str) {

            Some(_v) => {
                score = Some(*_v);
            },

            None => {
                self.do_nothing();
            }
        }
        
        return score;
    }

    fn do_nothing(&mut self) {
        return;
    }

    fn set_cache_score(&mut self, state:&Vec<Vec<i32>>, _piece:Pos, next_pos:Pos, score:i32) {

        let mut flat_state:Vec<i32> = state.iter()
                                            .flat_map(|array| array.iter())
                                            .cloned()
                                            .collect();
        
        let moves:Vec<i32> = vec![_piece.x as i32, _piece.y as i32, next_pos.x as i32, next_pos.y as i32];

        for i in 0..moves.len() {
            flat_state.push(moves[i]);
        }

        let mut hasher = DefaultHasher::new();

        flat_state.hash(&mut hasher);

        let hash_str = hasher.finish().to_string();

        self.minax_cache.insert(hash_str, score);
    }


    pub fn minimax(&mut self, mut temp_board:Board, depth:i32, is_max:bool, mut alpha:i32, mut beta:i32) -> i32 {

        let (done, _winner) = temp_board.finish_state();

        let mut initial:i32 = 0;

        if done || depth == 0 {
            //println!("Depth: {}", depth);
            initial = self.board_heuristics(temp_board);

        } else {

            if is_max {

                initial = -999999;
    
                let all_pieces:Vec<Pos> = temp_board.all_movable_pieces(temp_board.curr_player);
    
                for i in 0..all_pieces.len() {
    
                    let poss_moves:Vec<Pos> = temp_board.possible_moves(all_pieces[i], None);
    
                    for j in 0..poss_moves.len() {

                        let mut _result:i32 = 0;

                        let result:Option<i32> = self.get_cache_score(&temp_board.state.clone(), all_pieces[i], poss_moves[j]);
                        
                        match result {

                            None => {

                                let mut sim_board:Board = temp_board.clone_board();

                                sim_board.do_move(all_pieces[i], poss_moves[j]);

                                _result = self.minimax(sim_board, depth - 1, !is_max, alpha, beta);

                                self.set_cache_score(&temp_board.state.clone(), all_pieces[i], poss_moves[j], _result)
                            },

                            Some(_r) => {

                                self.quix += 1;

                                _result = _r;
                            }
                        }
    
                        initial = cmp::max(_result, initial);
    
                        alpha = cmp::max(alpha, initial);
    
                        if beta <= alpha {
                            break;
                        }
                    }
                }
    
            } else {
    
                initial = 999999;
    
                let all_pieces:Vec<Pos> = temp_board.all_movable_pieces(temp_board.curr_player);
    
                for i in 0..all_pieces.len() {
    
                    let poss_moves:Vec<Pos> = temp_board.possible_moves(all_pieces[i], None);
    
                    for j in 0..poss_moves.len() {
    
                        let mut _result:i32 = 0;

                        let result:Option<i32> = self.get_cache_score(&temp_board.state.clone(), all_pieces[i], poss_moves[j]);
                        
                        match result {

                            None => {

                                let mut sim_board:Board = temp_board.clone_board();

                                sim_board.do_move(all_pieces[i], poss_moves[j]);

                                _result = self.minimax(sim_board, depth - 1, !is_max, alpha, beta);

                                self.set_cache_score(&temp_board.state.clone(), all_pieces[i], poss_moves[j], _result)
                            },

                            Some(_r) => {

                                self.quix += 1;

                                _result = _r;
                            }
                        }
    
                        initial = cmp::min(_result, initial);
    
                        beta = cmp::min(beta, initial);
    
                        if beta <= alpha {
                            break;
                        }
                    }
                }
            }
        }

        return initial;
    }

    fn board_heuristics(&mut self, mut board:Board) -> i32 {

        let mut score:i32 = 0;

        let mut op_kings:i32 = 0;
        let mut op_pieces:i32 = 0;
        let mut op_poss_moves:i32 = 0;
        let mut op_taken:i32 = 0;

        let mut my_kings:i32 = 0;
        let mut my_pieces:i32 = 0;
        let mut my_poss_moves:i32 = 0;
        let mut my_taken:i32 = 0;

        for x in 0..board.row {
            for y in 0..board.col {

                if board.state[x][y] == -2 {
                    op_kings += 1;
                } else if board.state[x][y] == -1 {
                    op_pieces += 1;
                } else if board.state[x][y] == 1 {
                    my_pieces += 1;
                } else if board.state[x][y] == 2 {
                    my_kings += 1
                }
            }
        }

        let mut all_pieces:Vec<Pos> = board.all_movable_pieces(-1);

        for i in 0..all_pieces.len() {

            let poss_moves:Vec<Pos> = board.possible_moves(all_pieces[i], None);

            for j in 0..poss_moves.len() {

                op_poss_moves += 1;

                if let Some(_p) = poss_moves[j].piece {
                    op_taken += 1;
                }
            }
        }

        all_pieces = board.all_movable_pieces(1);

        for i in 0..all_pieces.len() {

            let poss_moves:Vec<Pos> = board.possible_moves(all_pieces[i], None);

            for j in 0..poss_moves.len() {

                my_poss_moves += 1;

                if let Some(_p) = poss_moves[j].piece {
                    my_taken += 1;
                }
            }
        }

        let king_diff:i32 = op_kings - my_kings;
        let piece_diff:i32 = op_pieces - my_pieces;
        let move_diff:i32 = op_poss_moves - my_poss_moves;
        let take_diff:i32 = op_taken - my_taken;

        let weights:Vec<i32> = vec![1000, 100, 10, 1];
        let factors:Vec<i32> = vec![king_diff, take_diff, move_diff, piece_diff];

        for w in 0..weights.len() {
            score += factors[w] * weights[w];
        }

        return score;
    }
}