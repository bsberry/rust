use std::io;

use dialoguer::Select;
use std::process::{Command, Stdio};
use std::borrow::BorrowMut;
use std::io::{Write, Stdin};
use isatty::stdout_isatty;

fn main() -> io::Result<()>{
    let mut lines: Vec<String> = Vec::new();
    let mut bytes_read: usize = 1;
    while bytes_read > 0 {
        let mut buffer = String::new();
        bytes_read = io::stdin().read_line(&mut buffer)?;
        lines.push(buffer);
    }

    let mut select = Select::new();
    select.with_prompt("Choose line");
        // .items(foo)
    select.default(0);
    select.clear(false);

    for line in &lines {
        select.item(line.trim());
    }

    let selection = select.interact().unwrap();

    let selected_line = lines.get(selection).unwrap().trim();

    if stdout_isatty() {
        let mut pbcopy = Command::new("pbcopy");
        pbcopy.stdin(Stdio::piped());
        let mut stdin = pbcopy.spawn()?.stdin.unwrap();
        let stdin_ref = stdin.borrow_mut();
        stdin_ref.write(selected_line.as_bytes());
        stdin_ref.flush();
    } else {
        print!("\"{}\"", selected_line); // Print to stdout in case you piped to something else
    }

    Ok(())
}
