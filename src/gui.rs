use raylib::prelude::*;

const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const SQUARE_SIZE:i32 = 100;
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq,Clone)]
pub enum PIECES {
    wk, // white king
    bk, // black king
    wq, // white queen
    bq, // black queen
    wr, // white rook
    br, // black rook
    wb, // white bishop
    bb, // black bishop
    wn, // white knight
    bn, // black knight
    wp, // white pawn
    bp, // black pawn
    NONE, // empty square
}

pub struct GUI;
impl GUI {
    // This does exacly what it says it does 
    // Note: this does not handle any error whatsoever 
    fn draw_board(handler:&mut RaylibDrawHandle) {
        let mut c:Color;
        for i in 0..8 {
            for j in 0..8 {
                if (i+j) % 2 == 0 {
                    c = Color::WHITE;
                } 
                else {
                    c = Color::BROWN;
                }
                handler.draw_rectangle(j*SQUARE_SIZE,i*SQUARE_SIZE,SQUARE_SIZE,SQUARE_SIZE,c);
            }
        }
    }

    // Given some stuff like thread and handle and a map of the filepaths , and a (row,col) this will
    // draw that piece in the chessboard
    fn draw_piece(row:i32,col:i32,drawer:&mut RaylibDrawHandle,piece:PIECES,texture_map: &std::collections::HashMap<PIECES, Texture2D>) {
        // Get the texture based on the PIECE enum
        if let Some(texture) = texture_map.get(&piece) {
            let position = Vector2::new(row as f32 * (SQUARE_SIZE as f32) + 4.0, col as f32 * (SQUARE_SIZE as f32) + 4.0);
            drawer.draw_texture_v(texture, position, Color::WHITE);
        }
    }

    pub fn load_filepaths(texture_map: &mut std::collections::HashMap<PIECES, Texture2D>,thread:RaylibThread,rl:&mut RaylibHandle) {
        texture_map.insert(PIECES::wk, rl.load_texture(&thread, "assets/wk.png").unwrap());
        texture_map.insert(PIECES::bk, rl.load_texture(&thread, "assets/bk.png").unwrap());
        texture_map.insert(PIECES::wq, rl.load_texture(&thread, "assets/wq.png").unwrap());
        texture_map.insert(PIECES::bq, rl.load_texture(&thread, "assets/bq.png").unwrap());
        texture_map.insert(PIECES::wr, rl.load_texture(&thread, "assets/wr.png").unwrap());
        texture_map.insert(PIECES::br, rl.load_texture(&thread, "assets/br.png").unwrap());
        texture_map.insert(PIECES::wb, rl.load_texture(&thread, "assets/wb.png").unwrap());
        texture_map.insert(PIECES::bb, rl.load_texture(&thread, "assets/bb.png").unwrap());
        texture_map.insert(PIECES::wn, rl.load_texture(&thread, "assets/wn.png").unwrap());
        texture_map.insert(PIECES::bn, rl.load_texture(&thread, "assets/bn.png").unwrap());
        texture_map.insert(PIECES::wp, rl.load_texture(&thread, "assets/wp.png").unwrap());
        texture_map.insert(PIECES::bp, rl.load_texture(&thread, "assets/bp.png").unwrap()); 
    } 

    // This draws the whole thing , it basicly draws what is in the vec that represent the board
    pub fn draw_chessboard(board_arr:&Vec<PIECES>,drawer:&mut RaylibDrawHandle,texture_map: &std::collections::HashMap<PIECES, Texture2D>) {
        Self::draw_board(drawer);
        let mut row:i32;
        let mut col:i32;
        for (sqr,b) in board_arr.iter().enumerate() {
            if *b != PIECES::NONE {
                row = (sqr as i32) / 8;
                col = (sqr as i32) % 8;
                Self::draw_piece(col,row,drawer,b.clone(),&texture_map);
            } 
        }
    } 
    // This reads the a fen notation
    pub fn load_fen(fen_str:&str,board_arr:&mut Vec<PIECES>) -> bool{
        let parts:Vec<&str> = fen_str.split(" ").collect();
        let mut result_code = true;
        // we will only handle the first part of the fen, the others are the engines job
        let mut rank:u32= 0;
        let mut file:u32 = 0;
        for c in parts[0].chars() {
            let square = rank*8+file;
            match c {
                'P' =>  board_arr[square as usize] = PIECES::wp, 
                'R' =>  board_arr[square as usize] = PIECES::wr, 
                'N' =>  board_arr[square as usize] = PIECES::wn, 
                'B' =>  board_arr[square as usize] = PIECES::wb, 
                'Q' =>  board_arr[square as usize] = PIECES::wq, 
                'K' =>  board_arr[square as usize] = PIECES::wk, 
                'p' =>  board_arr[square as usize] = PIECES::bp,
                'r' =>  board_arr[square as usize] = PIECES::br, 
                'n' =>  board_arr[square as usize] = PIECES::bn, 
                'b' =>  board_arr[square as usize] = PIECES::bb, 
                'q' =>  board_arr[square as usize] = PIECES::bq, 
                'k' =>  board_arr[square as usize] = PIECES::bk, 
                '1'..='8' => {
                    if let Some(x) = c.to_digit(10) {
                        for i in 0..x {
                            board_arr[(square + i)as usize] = PIECES::NONE;
                        }
                        file += x;
                    }
                }
                '/' => {
                    result_code = file == 8; 
                    rank += 1;
                    file = 0;
                }
                _ => result_code = false,
            }
            if LIST_OF_PIECES.contains(c) {
                file += 1;
            }
            if !result_code {
                break;
            }
        } 
        result_code
    }

    // This is prolly very usefull , a methode to move a pieces , i think that all what are we gonne
    // need here
    pub fn move_piece(board_arr:&mut Vec<PIECES>,old:usize,new:usize,promo:PIECES) -> bool {
        // if we already want to move a NONE piece then we gotta aleart the user 
        if board_arr[old] == PIECES::NONE {
            return false;
        }
        // we save the piece we want to move
        let saved = board_arr[old].clone();
        // pop it
        board_arr[old] = PIECES::NONE;
        // check for promotion
        if promo != PIECES::NONE {
            board_arr[new] = promo; 
        }else{
            board_arr[new] = saved; 
        } 
        return true;
    }
}
// stolen just to test shit out
pub fn algb_to_square(square: &str) -> Option<u8> {
    match square {
        "a8" => Some(0), "b8" => Some(1), "c8" => Some(2), "d8" => Some(3),
        "e8" => Some(4), "f8" => Some(5), "g8" => Some(6), "h8" => Some(7),
        "a7" => Some(8), "b7" => Some(9), "c7" => Some(10), "d7" => Some(11),
        "e7" => Some(12), "f7" => Some(13), "g7" => Some(14), "h7" => Some(15),
        "a6" => Some(16), "b6" => Some(17), "c6" => Some(18), "d6" => Some(19),
        "e6" => Some(20), "f6" => Some(21), "g6" => Some(22), "h6" => Some(23),
        "a5" => Some(24), "b5" => Some(25), "c5" => Some(26), "d5" => Some(27),
        "e5" => Some(28), "f5" => Some(29), "g5" => Some(30), "h5" => Some(31),
        "a4" => Some(32), "b4" => Some(33), "c4" => Some(34), "d4" => Some(35),
        "e4" => Some(36), "f4" => Some(37), "g4" => Some(38), "h4" => Some(39),
        "a3" => Some(40), "b3" => Some(41), "c3" => Some(42), "d3" => Some(43),
        "e3" => Some(44), "f3" => Some(45), "g3" => Some(46), "h3" => Some(47),
        "a2" => Some(48), "b2" => Some(49), "c2" => Some(50), "d2" => Some(51),
        "e2" => Some(52), "f2" => Some(53), "g2" => Some(54), "h2" => Some(55),
        "a1" => Some(56), "b1" => Some(57), "c1" => Some(58), "d1" => Some(59),
        "e1" => Some(60), "f1" => Some(61), "g1" => Some(62), "h1" => Some(63),
        _ => None,  // Return None for invalid input
    }
}
