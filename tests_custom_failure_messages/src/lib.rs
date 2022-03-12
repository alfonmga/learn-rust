pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Alfonso");
        assert!(result.contains("Alfonso"), "Greeting did not contain name, value was `{}`", result);
    }
}
