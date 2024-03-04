// https://godbolt.org/z/T3dvscqvW

#[no_mangle]
pub fn third_1(arr: &[u8; 10]) -> u8 {
    arr[3]
}

#[no_mangle]
pub fn third_2(arr: &[u8]) -> u8 {
    arr[3]
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e9bcc37ea6929833156cde3a061d399f
use std::mem::size_of;

struct Initial;
struct Negotiate;
struct Completed;

struct Protocol<S> {
    id: u8,
    state: S,
}

impl Protocol<Initial> {
    fn new(id: u8) -> Protocol<Initial> {
        Protocol {
            id,
            state: Initial,
        }
    }
    
    fn negotiate(self) -> Protocol<Negotiate> {
        Protocol {
            id: self.id,
            state: Negotiate,
        }
    }
}

impl Protocol<Negotiate> {
    fn complete(self) -> Protocol<Completed> {
        Protocol {
            id: self.id,
            state: Completed,
        }
    }
}

fn main() {
    // ZST
    println!("Protocol<Initial>: {}", size_of::<Protocol<Initial>>());
    println!("Protocol<Negotiate>: {}", size_of::<Protocol<Negotiate>>());
    println!("Protocol<Completed>: {}", size_of::<Protocol<Completed>>());
    
    // ok
    let initial = Protocol::new(1);
    // err
    // let completed = initial.complete();
    // ok
    let negotiate = initial.negotiate();
    
    // err
    // let complete = negotiate.negotiate();
    // ok
    let complete = negotiate.complete();
}
