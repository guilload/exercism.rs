pub fn lsp(string: &str, n: usize) -> Result<u32, ()> {
    if n == 0 {
        Ok(1)
    }

    else if string.len() < n || !string.chars().all(|c| c.is_digit(10)) {
        Err(())
    }

    else {
        string
            .chars()
            .flat_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()
            .windows(n)
            .map(|w| w.iter().product())
            .max()
            .ok_or(())
    }
}
