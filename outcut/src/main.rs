use dialoguer::Select;
use isatty::stdout_isatty;
use std::borrow::BorrowMut;
use std::io;
use std::process::{Command,Stdio};
use std::io::{Write, Stdin};

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
    // select.clear(false);

    for line in &lines {
        select.item(line.trim());
    }

    let selection = select.interact().unwrap();

    let selected_line = lines.get(selection).unwrap().trim();

    if stdout_isatty() {
        copy_to_clipboard(selected_line);
    } else {
        print!("\"{}\"", selected_line); // Print to stdout in case you piped to something else
    }

    Ok(())
}

fn copy_to_clipboard(line: &str) {
    let commands = ["xclip", "pbcopy"];
    for command in commands {
        let mut which = Command::new("which");
        which
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg(command);
        let success = which.status().unwrap().success();
        if (success) {
            println!("Command found: {}", command);
            do_copy_to_clipboard(command, line);
            return;
        }
    }

    println!("No commands of list [{}] found installed in system to copy to clipboard.", commands.join(", "));
}

fn do_copy_to_clipboard(command: &str, line: &str) {
    let mut cmd = Command::new(command);
    cmd.stdin(Stdio::piped());
    let mut stdin = cmd.spawn().expect("error running command").stdin.unwrap();
    let stdin_ref = stdin.borrow_mut();
    stdin_ref.write(line.as_bytes()).expect("error writing lines to be copied");
    stdin_ref.flush().expect("error flushing to command");
}
