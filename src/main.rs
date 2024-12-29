use raylib::prelude::*;

const SQUARE_SIZE:i32 = 100;
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq)]
enum PIECES {
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
}

// This does exacly what it says it does 
// Note: this does not handle any error whatsoever 
fn draw_chessboard(handler:&mut RaylibDrawHandle) {
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

fn load_filepaths(texture_map: &mut std::collections::HashMap<PIECES, Texture2D>,thread:RaylibThread,rl:&mut RaylibHandle) {
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

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Rustboard")
        .build();
    // We define a map such that f:PIECE --> filepath    
    let mut texture_map = std::collections::HashMap::new();
    load_filepaths(&mut texture_map,thread.clone(),&mut rl);
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        draw_chessboard(&mut d);
        draw_piece(0,5,&mut d,PIECES::wq,&texture_map);
        draw_piece(0,1,&mut d,PIECES::wk,&texture_map);
        draw_piece(5,5,&mut d,PIECES::bp,&texture_map);
    }
}
