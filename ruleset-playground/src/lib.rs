pub fn add(left: u64, right: u64) -> u64 {
    ruleset_playground_derive::add(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(1, 1);
        assert_eq!(result, 2);
    }
}
