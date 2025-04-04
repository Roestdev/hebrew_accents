#![doc = include_str!("../README.md")]

// TODO :  remove
pub fn dummy_function(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = dummy_function(2, 2);
        assert_eq!(result, 4);
    }
}
