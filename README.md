# Rustboard
Rustboard is a opensource project that i want to be a GUI app that uses the UCI protocol to draw the chessboard for your engine without being dependent on anything , that is by the way written in rust.

Rustboard uses the Raylib libray to provide simple Graphical user interface functionality to draw games of chess and allow to easy human-machine interactions.

This is meant to be a GUI interface that any chess engine can just plug right into using the UCI protocol allowing the user to test , play and create a chess engine programs.

# How to use 
You can start with this boilerplate code that looks very similar to a Raylib main file.
```rust
mod gui;
use crate::gui::{GUI,PIECES};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Rustboard")
        .build();

    // The API expects you have a texture map that dectates where your piece images are
    let mut texture_map = std::collections::HashMap::new();
    GUI::load_filepaths(&mut texture_map,thread.clone(),&mut rl);
	
		// It also expects you have a vector of 64 elements to store the pieces
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
```
The API always expects you to have some variables predefined and it will handle them for as long as the program runs.
# why ?
This was made just as an excuse for me to write more rust and i happen to be writting my own chess engine.


# Disclaimers
- This is not a chess engine , the game logic should be handled by a engine that checks for if moves are legal or not , this is simply providing functionality to see the game of chess and play it no matter how an engine works or was implimented.
- This crate is not done and still under development , things may change drastically , bear that in mind.

# Goals
- [x] Draw chessboard and load fen.
- [ ] drage and drop.
- [ ] UCI read STDOUT.
