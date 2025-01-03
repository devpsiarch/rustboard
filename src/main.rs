mod gui;
use crate::gui::{GUI,PIECES,algb_to_square};
use raylib::ffi::TraceLogLevel;

use std::process::{Command,Stdio};
use std::io::{BufRead, Write, BufReader};

use std::thread;
use std::time::Duration;

fn main() {
    let engine_location = "../rustmate/target/release/rustmate";
    let mut engine = Command::new(engine_location)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let engine_in = engine.stdin.as_mut().unwrap();
    let mut engine_out = BufReader::new(engine.stdout.as_mut().unwrap()); 
    // first loop to check if the engine supports the UCI protocol
    loop {
        let mut line = String::new();
        engine_out.read_line(&mut line).unwrap();
        println!("{}",line);
        if line == "uciok\n" {
            break;
        }
        line.clear();
    }
    println!("UCI SUPPORTED");
    let fen_board:&str = "4k3/8/5K2/2Q5/8/8/8/8 w - - 1 2";
    // Here goes the main game loop
    //let mut game_life:Vec<String> = vec!["position".to_string(),"startpos".to_string()];
    let mut game_life:Vec<String> = vec!["position".to_string(),
        "fen".to_string(),
        "4k3/8/5K2/2Q5/8/8/8/8 w - - 1 2".to_string()];
    let depth = 5;
    let mut moves = false;
    

    // GUI STUFF DO NOT TOUCH
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Rustboard")
        .build();
    rl.set_trace_log(TraceLogLevel::LOG_ERROR);
    // We define a map such that f:PIECE --> filepath    
    let mut texture_map = std::collections::HashMap::new();
    GUI::load_filepaths(&mut texture_map,thread.clone(),&mut rl);
    // well try to define a struct to store a board element
    let mut board:Vec<PIECES> = vec![PIECES::NONE;64];   
    GUI::load_fen("4k3/8/5K2/2Q5/8/8/8/8 w - - 1 2",&mut board); 
    
    // this halts the game when it is DONE
    let mut game_over = false;
    while game_over == false && !rl.window_should_close(){
        let mut d = rl.begin_drawing(&thread);
        let mut line = String::new();
        // Sending the game lifeline command , F UCI 
        let game_command = game_life.join(" ") + "\n";
        engine_in.write(game_command.as_bytes()).unwrap();
        println!("Game progressing ...");
        
        // Sending the seach command
        engine_in.write("go depth 6\n".as_bytes()).unwrap();
        println!("Searching for best move ...");
        
        // READING IS DONE HERE        
        engine_out.read_line(&mut line).unwrap();
        if line == "gameover\n" {
            println!("game is over ...");
            break;
        }

        print!("{line}");
        let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        if parts.len() <= 1 {
            println!("unexpected engine output, aborting...");
            break;
        }
        let best_move = parts[1].trim_end().to_string();
        // Indecate to the engine to make the moves after "moves"
        if !moves {
            game_life.push("moves".to_string());
            moves = true;
        }
        game_life.push(best_move);
        let result = game_life.join(" ");
        println!("{result}");
        // updating the board that will be drawn
        let mut src:usize = 0 ;
        let mut dst:usize = 0;
        let mut promo:PIECES = PIECES::NONE;
        if let Some(x) = algb_to_square(&parts[1][0..2])  {
            src = x as usize;
        }
        if let Some(x) = algb_to_square(&parts[1][2..4])  {
            dst = x as usize;
        }
        // am not handeing promos for now 
        GUI::move_piece(&mut board,src,dst,promo); 
        GUI::draw_chessboard(&board,&mut d,&texture_map);
    }




    // Turn off the engine
    match engine.kill() {
        Ok(_) => println!("Engine is offline , session ended gracefully"),
        Err(e) => eprintln!("Failed to turn the engine offline: {}", e),
    }

}

fn main_draw() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Rustboard")
        .build();
    rl.set_trace_log(TraceLogLevel::LOG_ERROR);
    // We define a map such that f:PIECE --> filepath    
    let mut texture_map = std::collections::HashMap::new();
    GUI::load_filepaths(&mut texture_map,thread.clone(),&mut rl);
    // well try to define a struct to store a board element
    let mut board:Vec<PIECES> = vec![PIECES::NONE;64]; 
    

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        GUI::draw_chessboard(&board,&mut d,&texture_map);
    }
}
