trait Output<T> {
    fn render(&self) -> T {}
    fn calculate_save_location(&self) -> T {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
