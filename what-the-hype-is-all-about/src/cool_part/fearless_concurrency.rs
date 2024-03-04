// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4452d2a0bb8288742eae560edd609408
use std::thread;

fn main() {
    let mut shared = [1, 2, 3, 4];
    
    let mut handles = Vec::new();
    
    let h_1 = thread::spawn(|| {
        shared[0] = 1;
    });
    handles.push(h_1);
    
    let h_2 = thread::spawn(|| {
        shared[0] = 1;
    });
    handles.push(h_1);
    
    for handle in handles {
        handle.join();
    }
}
