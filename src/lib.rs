pub fn answer() -> i32 {
    return 42;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_validity_unittest() {
        assert_eq!(answer(), 42);
    }
}
