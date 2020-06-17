use rand;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn simple() {
        assert_eq!(4, add_one(3));
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
