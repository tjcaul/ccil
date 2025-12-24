#[cfg(test)]
mod test {
    use ccil::*;

    // TODO: Add actual tests
    #[test]
    fn dummy_test() {
        assert_eq!(constants::CCIL_MAGIC_BYTE_0, 0xCC);
        assert_eq!(constants::CCIL_MAGIC_BYTE_1, 0x17);
    }
}