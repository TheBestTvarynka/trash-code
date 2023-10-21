
#[allow(dead_code)]
pub fn buffer_overflow_1(buffer: &mut [String]) {
    let _s = &buffer[10];
}

#[allow(dead_code)]
pub fn buffer_overflow_2(buffer: &mut [String]) {
    let s = buffer.get(1);

    match s {
        Some(s) => println!("present: {:?}", s),
        None => println!("missing"),
    }
}

#[allow(dead_code)]
pub fn dangling_pointers() {
    let _some_data = vec![1, 2, 3];
    // let reference;
    {
        // inner scope
        let mut _s = String::from("tbt");
        // reference = &mut s;
        // drop(s);
    }
    // println!("{:?}", reference);
    // drop(some_data);
}

#[cfg(test)]
mod tests {
    use crate::cool_part::memory_safety::{buffer_overflow_1, buffer_overflow_2};

    #[test]
    #[should_panic]
    fn test_buffer_overflow_1() {
        buffer_overflow_1(& mut []);
    }

    #[test]
    fn test_buffer_overflow_2() {
        buffer_overflow_2(& mut []);
    }
}