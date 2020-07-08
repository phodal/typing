/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::ime::event::{ActionEventReceiver, Event, KeyEventReceiver, SystemEventReceiver, KeyEvent};
use std::sync::mpsc::Receiver;

pub trait EventManager {
    fn eventloop(&self);
}

pub struct DefaultEventManager {
    receive_channel: Receiver<Event>,
}

impl DefaultEventManager {
    pub fn new(
        receive_channel: Receiver<Event>,
    ) -> DefaultEventManager {
        DefaultEventManager {
            receive_channel,
        }
    }
}

impl EventManager for DefaultEventManager {
    fn eventloop(&self) {
        loop {
            match self.receive_channel.recv() {
                Ok(event) => match event {
                    Event::Key(key_event) => match key_event {
                        KeyEvent::Char(c) => {
                            println!("{}", c);
                        }
                        KeyEvent::Modifier(m) => {}
                        KeyEvent::Other => {}
                    },
                    _ => {}
                },
                Err(e) => panic!("Broken event channel {}", e),
            }
        }
    }
}
