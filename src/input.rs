

use std::sync::mpsc;

#[derive(Eq, PartialEq, Clone)]
pub enum InputEvent {
    ConsoleEvent(::unsegen::Event),
    PseudoTerminalEvent(::unsegen::Event),
    Quit,
}

pub trait InputSource {
    fn start_loop(event_sink: mpsc::Sender<InputEvent>) -> Self;
}

#[derive(Clone, Copy)]
enum Mode {
    Console,
    PTY,
}

pub struct ViKeyboardInput {
    _thread: ::std::thread::JoinHandle<()>,
}

impl ViKeyboardInput {
    fn input_loop(output: mpsc::Sender<InputEvent>) {
        use termion::input::TermRead;

        let mut mode = Mode::Console;
        let stdin = ::std::io::stdin(); //TODO lock outside of thread
        let stdin = stdin.lock();
        for c in stdin.keys() {
            let c = c.expect("key");
            match c {
                ::termion::event::Key::F(1) => {
                    mode = match mode {
                        Mode::Console => Mode::PTY,
                        Mode::PTY => Mode::Console,
                    }
                },
                c => {
                    let event = match mode {
                        Mode::Console => { InputEvent::ConsoleEvent(::unsegen::Event::Key(c)) },
                        Mode::PTY => { InputEvent::PseudoTerminalEvent(::unsegen::Event::Key(c)) },
                    };
                    output.send(event).expect("send event");
                },
            }
        }
        output.send(InputEvent::Quit).expect("send quit");
    }
}

impl InputSource for ViKeyboardInput {
    fn start_loop(event_sink: mpsc::Sender<InputEvent>) -> Self {
        let input_thread = ::std::thread::spawn(move || {
            Self::input_loop(event_sink);
        });
        ViKeyboardInput {
            _thread: input_thread,
        }
    }
}