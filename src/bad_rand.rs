pub fn rand_index(size: usize) -> usize {
    let index = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time is going backwards!")
        .as_millis() as usize;
    index % size
}

#[cfg(test)]
mod tests {
    use super::rand_index;

    #[test]
    fn test_rand_index() {
        (0..100).for_each(|i| {
            assert_eq!(rand_index(0), 0);
            assert!(rand_index(i) <= i);
        })
    }
}
