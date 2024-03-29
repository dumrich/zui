use std::io;
use std::process;
use zui_core::key::Key;
use zui_core::term::Terminal;

fn main() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    my_term.enter_raw_mode().unwrap();
    my_term.switch_screen().unwrap();

    let keys1 = my_term.keys(io::stdin());
    for k in keys1.clone() {
        my_term.set_cursor_to(1, my_term.y_pos).unwrap();
        my_term.clear_line().unwrap();

        println!("{:?}", k);

        if let Key::Ctrl('c') = k {
            my_term.switch_main().unwrap();
            process::exit(1);
        }
    }
}
