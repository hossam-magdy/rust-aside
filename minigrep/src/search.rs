pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let needle = "q1";
        let haystack = "bla bla
foo bar 1 - q1
foo bar 2 - Q2
foo bar 3 - q1 & q3
foo bar 4 - Q4";

        assert_eq!(
            search(needle, haystack),
            vec!["foo bar 1 - q1", "foo bar 3 - q1 & q3"]
        );
    }
}
