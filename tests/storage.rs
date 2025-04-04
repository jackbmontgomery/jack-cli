#[cfg(test)]
mod tests {
    use jack::storage;

    #[test]
    fn it_works() {
        let result = storage::read();
        assert_eq!(result, vec!["hello"]);
    }
}
