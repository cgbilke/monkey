mod token;

struct Lexer {
    input: String,
    position: int,      //current position in input (points to current char)
    read_position: int, //current reading position in input (after current char)
    ch: byte,           //current char under examination
}

mod tests {

    #[test]
    fn test_next_token() {}
}
