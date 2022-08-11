#![allow(unused)]

use std::io::{stdout, Write};
use std::thread;
use std::panic::{set_hook, take_hook};
use std::time::Duration;
use random::{Default, Source};

use crossterm::{
    cursor::{MoveLeft, MoveTo, Hide, Show },
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode, size},
};

struct Pos(u16, u16);

impl Pos {
    fn random(source: &mut Default, &Pos(max_x, max_y): &Pos) -> Pos {
        let x = source.read::<u16>() % max_x;
        let y = source.read::<u16>() % max_y;
        Pos(x, y)
    }
}

fn setup() {
    stdout().execute(Hide).unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
}

fn cleanup() {
    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
    stdout().execute(Show).unwrap();
}

fn main() {
    // Safely setup terminal
    let panic_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        cleanup();
        panic_hook(panic_info);
    }));
    setup();

//        .execute(SetBackgroundColor(Color::Red)).unwrap();
//        .execute(Clear(ClearType::All)).unwrap()
//    println!("this is in the alternate screen");
//    thread::sleep(Duration::from_secs(1));

    let mut pos = Pos(15,15);
    let (max_x, max_y) = size().unwrap();
    let max_pos = Pos(max_x, max_y);
    let mut source = random::default();
    loop {
        match read().unwrap() {
            Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL } ) => {
                cleanup();
                return;
            }
            Key(KeyEvent { code, modifiers }) => {
//                write!(stdout(), "{:?} {:?}", code, modifiers);
//                stdout().execute(MoveLeft(1));
                let pos = Pos::random(&mut source, &max_pos);
                stdout().execute(MoveTo(pos.0, pos.1));
                write!(stdout(), "*");
                stdout().flush();
            }
            _ => println!("other event"),
        }
    }
}
