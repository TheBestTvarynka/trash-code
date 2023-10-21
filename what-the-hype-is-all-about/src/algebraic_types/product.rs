use time::OffsetDateTime;

struct User {
    pub name: String,
    pub email: String,
}

struct Age(OffsetDateTime);

type Point = (u32, u32);

#[derive(Debug)]
struct Flags {
    pub f1: bool,
    pub f2: bool
}

#[cfg(test)]
mod tests {
    use crate::algebraic_types::product::Flags;

    #[test]
    fn possible_flags_values() {
        println!("{:?}", vec![
            Flags { f1: true, f2: true },
            Flags { f1: true, f2: false },
            Flags { f1: false, f2: true },
            Flags { f1: false, f2: false },
        ]);
    }
}
