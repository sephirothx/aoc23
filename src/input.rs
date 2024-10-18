use std::fs;

pub fn read_from_file(day: i32) -> String {
    fs::read_to_string(format!("./input/{day}.txt")).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_from_file() {
        assert_eq!("69", super::read_from_file(0));
    }
}
