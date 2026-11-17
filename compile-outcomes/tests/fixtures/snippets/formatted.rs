pub fn formatted() -> String {
    itoa::Buffer::new().format(42).to_owned()
}
