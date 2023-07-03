pub mod fizzbuzzfolder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_prints_number_if_not_divisible_by_3_or_5() {
        let expected = vec!["1"];
        assert_eq!(expected, fizzbuzzfolder::fizzbuzzfile::fizzbuzz(1));
    }

    #[test]
    fn fizz_works() {
        let expected = vec!["1", "2", "Fizz"];
        assert_eq!(expected, fizzbuzzfolder::fizzbuzzfile::fizzbuzz(3));
    }

    #[test]
    fn buzz_works() {
        let expected : Vec<String> = vec!["1".to_string(), "2".to_string(), "Fizz".to_string(), "4".to_string(), "Buzz".to_string()];
        assert_eq!(expected, fizzbuzzfolder::fizzbuzzfile::fizzbuzz(5));
    }

    #[test]
    #[should_panic]
    fn invalid_input() {
        fizzbuzzfolder::fizzbuzzfile::fizzbuzz(121);
    }
}
