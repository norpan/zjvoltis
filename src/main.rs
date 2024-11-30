pub mod zjvoltis;
include!("minimax.rs");
use std::time::SystemTime;
fn main() {
    for depth in 1..10 {
        println!("Depth {}", depth);
        let mut game = Zjvoltis::new();
        game = game.make_move(&ZjvoltisMove::from_string("c31")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("i62")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("f32")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("i51")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("i32")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("h62")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("f32")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("i41")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("a21")).unwrap();
        //game = game.make_move(&ZjvoltisMove::from_string("e71")).unwrap();
        println!("{}", game.to_string());
        let now = SystemTime::now();
        let (score, best_move, nodes) = minimax(game, depth);
        let m = best_move.unwrap();
        println!("Score {}", score);
        println!("Best move {}", m.to_string());
        println!("Nodes visited {}", nodes);
        match now.elapsed() {
            Ok(elapsed) => {
                println!("Time taken {}ms", elapsed.as_millis());
                println!("Nodes per second {}", nodes as f64 / elapsed.as_secs_f64());
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
