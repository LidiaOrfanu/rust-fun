pub mod fizzbuzzfolder;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_does_not_work() {
        assert_eq!(4, fizzbuzzfolder::fizzbuzzfile::add_one_test(2));
    }

    #[test]
    fn it_works() {
        assert_eq!(3, fizzbuzzfolder::fizzbuzzfile::add_one_test(2));
    }
}