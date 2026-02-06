pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = ruleset_playground_derive::add(2, 2);
        assert_eq!(result, 4);
    }
}
