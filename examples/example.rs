use crossterm_simple_event::CrosstermSimpleEvent;
use ratatui::{TerminalOptions, Viewport, widgets::Widget};

pub fn main() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        hook(info);
    }));

    fn run() {
        let mut term = ratatui::init_with_options(TerminalOptions {
            viewport: Viewport::Inline(1),
        });
        let mut print = |msg: &str| {
            _ = term.insert_before(1, |buf| {
                msg.render(*buf.area(), buf);
            });
        };
        print("press b to say hi.");
        print("press ctrl+c to exit.");
        loop {
            // read event from crossterm
            let ev = crossterm::event::read().unwrap();
            // call `simple` and turn into &str
            match ev.simple().as_str() {
                "ctrl+c" => {
                    print("done.");
                    break;
                }
                "b" => {
                    print("hello world!");
                }
                "p" => {
                    panic!("at the disco");
                }
                _ => print("naughty naughty"),
            }
        }
    }
    run();

    ratatui::restore();
}
