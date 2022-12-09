use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use fltk::{app, frame::Frame, prelude::*, window::Window};
use fltk::app::{add_timeout3, App, event_key};
use fltk::draw::{draw_rect_fill, draw_rect_with_color, Offscreen};
use fltk::enums::{Color, Event, Key};
use crate::tetris::tetris::{Action};
use crate::tetris::tetris::Action::Down;

pub struct Gui {}

impl Gui {
    pub fn launch(action_sender: Sender<Action>, blocks_receiver: Receiver<[[bool; 20]; 10]>) {
        let app = App::default();
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

        let timer_sender = action_sender.clone();
        let key_event_sender = action_sender.clone();

        /* Event handling */
        let callback = move |handle| {
            timer_sender.send(Down).unwrap();
            app::repeat_timeout3(0.3, handle);
        };
        add_timeout3(0.3, callback);


        window.handle(move |_, event| {
            match event {
                Event::KeyDown => {
                    match event_key() {
                        Key::Up => {
                            key_event_sender.send(Action::Rotate).unwrap();
                            true
                        }
                        Key::Left => {
                            key_event_sender.send(Action::Left).unwrap();
                            true
                        }
                        Key::Right => {
                            key_event_sender.send(Action::Right).unwrap();
                            true
                        }
                        Key::Down => {
                            key_event_sender.send(Action::Drop).unwrap();
                            true
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        });
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

        while app.wait() {
            if let Ok(blocks) = blocks_receiver.try_recv() {
                let offs = offs.borrow_mut();
                    offs.begin();
                    for x in 0i8..10i8 {
                        for y in 0i8..20i8 {
                            let colour = if blocks[usize::from(x.unsigned_abs())][usize::from(y.unsigned_abs())] {
                                Color::Black
                            } else {
                                Color::White
                            };
                            draw_rect_fill(i32::from(x) * 20, i32::from(y) * 20, 20, 20, colour);
                            draw_rect_with_color(i32::from(x) * 20, i32::from(y) * 20, 20, 20, Color::White);
                        }
                    }
                    offs.end();
                    frame.redraw();
            }
        }
    }
}