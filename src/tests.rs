

#[cfg(test)]
mod tests {
    use crate::operations;
    use crate::hash;

    #[test]
    fn test1() {
        let data = b"a";
        let mut input = data.to_vec();
        let result = hash::sha512(&mut input);
        let result_string = operations::get_hex(&result);

        assert_eq!("1f40fc92da241694750979ee6cf582f2d5d7d28e18335de05abc54d0560e0f5302860c652bf08d560252aa5e74210546f369fbbbce8c12cfc7957b2652fe9a75", result_string)
    }

    #[test]
    fn test2() {
        let data = b"Hello World!";
        let mut input = data.to_vec();
        let result = hash::sha512(&mut input);
        let result_string = operations::get_hex(&result);

        assert_eq!("861844d6704e8573fec34d967e20bcfef3d424cf48be04e6dc08f2bd58c729743371015ead891cc3cf1c9d34b49264b510751b1ff9e537937bc46b5d6ff4ecc8", result_string)
    }
}