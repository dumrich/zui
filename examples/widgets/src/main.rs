use std::io::{self, Read};
use std::io::{stdin, Write};
use std::process;
use std::thread;
use std::time::Duration;
use zui_core::term::Terminal;
use zui_core::widgets::popup::Popup;
use zui_core::widgets::Widget;

fn main() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    my_term.enter_raw_mode().unwrap();
    my_term.clear_screen().unwrap();

    ui(&mut my_term);

    thread::sleep(Duration::from_secs(9));

    my_term.show_cursor().unwrap();
}

fn ui<T: Write>(term: &mut Terminal<T>) {
    let p = Popup::new(term).title("Find Files").width(60).height(25);
    p.render(term).unwrap();
}
