
#[derive(Debug, Clone)]
pub enum Value {
    None,
    False,
    True
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(true);
    }
}