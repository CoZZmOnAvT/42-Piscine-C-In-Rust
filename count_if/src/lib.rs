#[allow(dead_code)]
fn count_if(tab: Vec<&'static str>, func: fn(&'static str) -> i8) -> usize {
    let mut ret = 0;
    for value in &tab {
        if func(value) == 1 {
            ret += 1;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    fn test_func(str: &'static str) -> i8 {
        if str.len() == 5 {
            1
        } else {
            42
        }
    }

    #[test]
    fn test_count_if() {
        let tab: Vec<&'static str> = vec!["Hello", "World", "Folks", "!"];
        assert_eq!(crate::count_if(tab, test_func), 3);
    }
}
