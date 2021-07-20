pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let needle = needle.to_lowercase();

    for line in haystack.lines() {
        if line.to_lowercase().contains(&needle) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
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

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
