pub mod markdown;
pub mod html;

trait Convert<T> {
    fn convert(&self) -> T {}
}

trait SavePage {
    fn compute_output(&self) -> String {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
