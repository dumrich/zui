//use std::io;
//use std::sync::{Arc, Mutex};
//use std::thread;
//use zui::key;
//
//fn main() {
//    let mut output = io::stdout();
//    let my_term = Arc::new(Mutex::new(Terminal::new(&mut output).unwrap()));
//
//    let term = Arc::clone(&my_term);
//    let handle = thread::spawn(move || {
//        let mut term = term.lock().unwrap();
//        for k in term.keys() {
//            let k = k.lock().unwrap();
//            handle_key(k);
//        }
//    });
//}

use std::io::stdin;
use std::io::{self, Read};
use std::process;
use std::thread;
use std::time::Duration;
use zui::key::Key;
use zui::term::clear::TClear;
use zui::term::cursor::TCursor;
use zui::term::Terminal;

fn main() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    my_term.enter_raw_mode().unwrap();
    my_term.switch_screen().unwrap();

    for k in my_term.keys(io::stdin()) {
        my_term.set_cursor_to(1, my_term.y_pos).unwrap();
        my_term.clear_line().unwrap();

        println!("{:?}", k);

        if let Key::Ctrl('c') = k {
            my_term.switch_main().unwrap();
            process::exit(1);
        }
    }

    //for k in io::stdin().bytes() {
    //    my_term.set_cursor_to(1, my_term.y_pos + 1).unwrap();
    //    my_term.clear_line().unwrap();

    //    println!("{:?}", k);

    //    if let Ok(3) = k {
    //        process::exit(1);
    //    }
    //}
}
