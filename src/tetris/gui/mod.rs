use std::cell::RefCell;
use std::rc::Rc;
use fltk::{app, button::Button, enums, frame::Frame, prelude::*, window::Window};
use fltk::app::{App, event_key};
use fltk::draw::{draw_pie, draw_rect_fill, draw_rect_with_color, Offscreen, set_draw_color};
use fltk::enums::{Color, Event, Key};
use crate::tetris::tetris::{Action, Tetris};

pub struct Gui {
    app: App,
    window: Window,
}

impl Gui {
    pub fn new(mut tetris: Tetris) -> Gui {
        let app = app::App::default();
        let mut window = Window::default()
            .with_size(200, 400)
            .center_screen()
            .with_label("Tetris");
        let mut frame = Frame::default()
            .with_size(200, 400)
            .center_of(&window);
        frame.set_color(Color::White);
        window.make_resizable(false);
        window.end();
        window.show();
        /* Event handling */
        let offs = Offscreen::new(frame.width(), frame.height()).unwrap();
            offs.begin();
            draw_rect_fill(0, 0, 200, 400, Color::White);
            offs.end();

        let offs = Rc::from(RefCell::from(offs));

        frame.draw({
            let offs = offs.clone();
            move |_| {
                let mut offs = offs.borrow_mut();
                if offs.is_valid() {
                    offs.rescale();
                    offs.copy(0, 0, 200, 400, 0, 0);
                } else {
                    offs.begin();
                    draw_rect_fill(0, 0, 200, 400, Color::White);
                    offs.copy(0, 0, 200, 400, 0, 0);
                    offs.end();
                }
            }
        });

        window.handle(move |w, event| {
            let offs = offs.borrow_mut();
            offs.begin();
            let handled = match event {
                Event::KeyDown => {
                    match event_key() {
                        Key::Up => {
                            tetris.input(Action::Rotate);
                            true
                        }
                        Key::Left => {
                            tetris.input(Action::Left);
                            true
                        }
                        Key::Right => {
                            tetris.input(Action::Right);
                            true
                        }
                        Key::Down => {
                            tetris.input(Action::Drop);
                            true
                        }
                        _ => false,
                    }
                }
                _ => false,
            };
            for x in 0..10 {
                for y in 0..20 {
                    let colour = if tetris.block_at(x, y) {
                        enums::Color::Black
                    } else {
                        enums::Color::White
                    };
                    draw_rect_fill(i32::from(x) * 20, i32::from(y) * 20, 20, 20, colour);
                    draw_rect_with_color(i32::from(x) * 20, i32::from(y) * 20, 20, 20, enums::Color::White);
                }
            }
            offs.end();
            w.redraw();
            handled
        });
        let mut gui = Gui {
            app,
            window,
        };
        gui
    }

    pub fn run(&mut self) {
        self.app.run().unwrap();
    }
}