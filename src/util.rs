pub fn read_input(day: usize) -> String {
    let path = format!("src/d{}.in", day);
    std::fs::read_to_string(path).expect("missing input")
}
