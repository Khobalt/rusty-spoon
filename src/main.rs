#![forbid(unused_imports)]
use std::thread;

fn main() {
    play_with_threads();
}


fn play_with_threads(){
    let mut joinhandles = Vec::new();
    
    for offset in 0..8 {
        joinhandles.push(thread::spawn(move || {
            for i in 0..10000 {
                println!("I'm thread {}, spitting out the number {}", offset, i);
            }
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }

}