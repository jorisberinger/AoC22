use std::fs;

pub fn read_in_file(file_path: &str) -> String {
    let file = fs::read_to_string(file_path);
    return file.unwrap_or(String::new());
}

#[cfg(test)]
mod tests {
    use crate::read_in_file;

    #[test]
    fn read_file() {
        let file_path = "./src/test.txt";
        let result = read_in_file(file_path);
        assert_eq!(result, "1\n2\n3\n4\n");
    }
}
