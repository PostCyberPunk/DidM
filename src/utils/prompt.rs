//TODO: move this to a new mod so we can have both tui and cli
//Ask user to confirm
pub fn confirm(msg: &str) -> bool {
    let mut input = String::new();
    println!("{}\npress y to confirm:", msg);
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim() == "y"
}
