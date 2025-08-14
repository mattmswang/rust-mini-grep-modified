pub fn search<'a> (query: &str, contents: &'a str) -> Vec<(&'a str, i32)> {
    let mut x: Vec<(&str, i32)> = Vec::new();
    let mut i: i32 = 1;
    for line in contents.lines() {
        if line.contains(query) {
            x.push((line,i));
        }
        i += 1;
    }
    x
}

pub fn search_case_insensitive <'a> (
    query: &str,
    contents: &'a str
) -> Vec<(&'a str, i32)> {
    let query = query.to_lowercase(); 
    let mut results: Vec<(&'a str, i32)> = Vec::new();
    let mut i: i32 = 1;
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push((line, i));
        }
        i += 1;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*; // access all of this libraries functions
    #[test] // mark as test
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        println!("{contents}");
        assert_eq!(vec![("safe, fast, productive.",2)], search(query, contents));
    }

    #[test] 
    fn case_insensitive (){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec![("Rust:",1),("Trust me.",4)],
            search_case_insensitive(query, contents)
        )
    }
}
