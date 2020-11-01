use crate::AddressSpace;
use bit_field::BitField;
use winit::event::{ElementState, VirtualKeyCode};

pub struct Controller {
    pub state: u8,
    polls: u8,
    strobe: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: 0,
            polls: 0,
            strobe: false,
        }
    }

    pub fn update(&mut self, keycode: &VirtualKeyCode, state: &ElementState) {
        let pressed = match state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };

        match keycode {
            VirtualKeyCode::W => {
                self.state.set_bit(4, pressed);
            }
            VirtualKeyCode::A => {
                self.state.set_bit(6, pressed);
            }
            VirtualKeyCode::S => {
                self.state.set_bit(5, pressed);
            }
            VirtualKeyCode::D => {
                self.state.set_bit(7, pressed);
            }
            VirtualKeyCode::Semicolon => {
                self.state.set_bit(0, pressed);
            }
            VirtualKeyCode::Apostrophe => {
                self.state.set_bit(1, pressed);
            }
            VirtualKeyCode::Return => {
                self.state.set_bit(3, pressed);
            }
            VirtualKeyCode::RShift => {
                self.state.set_bit(2, pressed);
            }
            _ => (),
        };
    }
}

impl AddressSpace for Controller {
    fn peek(&mut self, ptr: u16) -> u8 {
        if ptr == 0x4016 {
            //println!("poll: {}", self.polls);

            if self.polls > 7 {
                2
            } else {
                let val = (self.state.get_bit(self.polls as usize) as u8);

                self.polls += 1;

                val
            }
        } else {
            3
        }
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        if byte > 0 {
            self.strobe = true;
            self.polls = 0;
        } else {
            self.strobe = false;
            self.polls = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_all_state() {
        let mut controller = Controller::new();
        controller.poke(0x4016, 1);
        controller.poke(0x4016, 0);

        assert_eq!(0, controller.peek(0x4016));
        assert_eq!(0, controller.peek(0x4016));
        assert_eq!(0, controller.peek(0x4016));
        assert_eq!(0, controller.peek(0x4016));
        assert_eq!(0, controller.peek(0x4016));
        assert_eq!(0, controller.peek(0x4016));
        assert_eq!(0, controller.peek(0x4016));
        assert_eq!(0, controller.peek(0x4016));

        assert_eq!(2, controller.peek(0x4016));
    }
}
