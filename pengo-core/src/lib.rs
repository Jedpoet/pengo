pub mod error;
mod novel;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::novel::new_novel;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let novel_name = "test_novel";
        let _ = new_novel(novel_name);

        let mut novel = "";

        if std::path::Path::new(novel_name).exists() {
            println!("{} has existed！", novel_name);
            novel = novel_name;
        }
        assert_eq!(novel, novel_name);
    }
}
