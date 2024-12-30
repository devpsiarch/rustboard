mod gui;
use crate::gui::{GUI,PIECES};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Rustboard")
        .build();
    // We define a map such that f:PIECE --> filepath    
    let mut texture_map = std::collections::HashMap::new();
    GUI::load_filepaths(&mut texture_map,thread.clone(),&mut rl);
    // well try to define a struct to store a board element
    let mut board:Vec<PIECES> = vec![PIECES::NONE;64]; 
    GUI::load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",&mut board);
    GUI::load_fen("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1",&mut board);
    GUI::move_piece(&mut board,0,1,PIECES::NONE); 
    GUI::move_piece(&mut board,7,63,PIECES::wq); 
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        GUI::draw_chessboard(&board,&mut d,&texture_map);
    }
}
