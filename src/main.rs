use std::env;
use std::fs;
use std::io::Write;
use std::io::stdin;
use std::path;
use std::process;
use termion;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::style;

struct Doc {
    lines: Vec<String>
}

struct Coordinates {
    x: u16,
    y: u16
}

struct TerminalSize {
    x: u16,
    y: u16
}

struct TextViewer {
    doc: Doc,
    doc_length: u16,
    cur_pos: Coordinates,
    terminal_size: TerminalSize,
    file_name: String
}

impl TextViewer {
    fn new(file_name: &str) -> Self {
        let mut lines = Vec::new();
        for line in fs::read_to_string(file_name).unwrap().lines() {
            lines.push(line.to_string());
        }
        let doc = Doc { lines };
        let doc_length = doc.lines.len() as u16;
        let cur_pos = Coordinates { x: 1, y: doc_length };
        let terminal_size = termion::terminal_size().unwrap();
        let terminal_size = TerminalSize {
            x: terminal_size.0,
            y: terminal_size.1
        };
        let file_name = file_name.to_string();
        Self { doc, doc_length, cur_pos, terminal_size, file_name }
    }

    fn show_document(&mut self) {
        let pos = &self.cur_pos;
        let (old_x, old_y) = (pos.x, pos.y);

        // Display header
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!(
            "{}Welcome to Mini text viewer!{}",
            color::Fg(color::White),
            style::Reset
        );

        // Display text
        for line in &self.doc.lines {
            println!("{}\r", line);
        }

        // Display footer
        println!("{}", termion::cursor::Goto(0, &self.terminal_size.y - 2));
        println!(
            "{}{} line-count: {} Filename: {}{}",
            color::Fg(color::Red),
            style::Bold,
            &self.doc_length,
            &self.file_name,
            style::Reset
        );
        self.set_pos(old_x, old_y);
    }

    fn set_pos(&mut self, x: u16, y: u16) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;
        println!("{}", termion::cursor::Goto(x, y));
    }

    fn inc_x(&mut self) {
        if self.cur_pos.x < self.terminal_size.x {
            self.cur_pos.x += 1;
        }
        println!("{}", termion::cursor::Goto(self.cur_pos.x, self.cur_pos.y));
    }

    fn dec_x(&mut self) {
        if self.cur_pos.x > 0 {
            self.cur_pos.x -= 1;
        }
        println!("{}", termion::cursor::Goto(self.cur_pos.x, self.cur_pos.y));
    }

    fn inc_y(&mut self) {
        if self.cur_pos.y < self.terminal_size.y {
            self.cur_pos.y += 1;
        }
        println!("{}", termion::cursor::Goto(self.cur_pos.x, self.cur_pos.y));
    }

    fn dec_y(&mut self) {
        if self.cur_pos.y > 0 {
            self.cur_pos.y -= 1;
        }
        println!("{}", termion::cursor::Goto(self.cur_pos.x, self.cur_pos.y));
    }

    fn run(&mut self) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Ctrl('c') => break,
                Key::Left => {
                    self.dec_x();
                    self.show_document();
                },
                Key::Right => {
                    self.inc_x();
                    self.show_document();
                },
                Key::Up => {
                    self.dec_y();
                    self.show_document();
                },
                Key::Down => {
                    self.inc_y();
                    self.show_document();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide the file name as argumane:");
        println!("  example: mini-textviewer ./my_file.md");
        process::exit(0);
    }

    let path = &args[1];
    if !path::Path::new(path).exists() {
        println!("File path doesn't exist");
        process::exit(0);
    }

    println!("{}", termion::cursor::Show);
    let mut viewer = TextViewer::new(&args[1]);
    viewer.show_document();
    viewer.run();
}
