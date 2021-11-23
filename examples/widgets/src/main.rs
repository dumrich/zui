use std::io::{self, Read};
use std::io::{stdin, Write};
use std::process;
use std::thread;
use std::time::Duration;
use zui_core::term::Terminal;
use zui_core::widgets::popup::Popup;
use zui_core::widgets::{Position, Widget};

fn main() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    my_term.enter_raw_mode().unwrap();
    my_term.clear_screen().unwrap();

    let position = ui(&mut my_term);

    println!("position: {:?}", position.starting_pos);
    println!("position: {:?}", position.ending_pos);
    my_term
        .set_cursor_to(position.starting_pos.0, position.starting_pos.1)
        .unwrap();
    my_term.show_cursor().unwrap();
    thread::sleep(Duration::from_secs(9));
}

fn ui<T: Write>(term: &mut Terminal<T>) -> Position {
    let x = Popup::new(term)
        .title("Find")
        .width(60)
        .height(2)
        .y_offset(14);
    x.render(term).unwrap();

    let p = Popup::new(term).title("").width(60).height(25);
    p.render(term).unwrap()
}
