//TODO: move this to a new mod so we can have both tui and cli
//Ask user to confirm
pub fn confirm(msg: &str) -> bool {
    let mut input = String::new();
    println!("{}", msg);
    print!("press y to confirm: ");
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim() == "y"
}
