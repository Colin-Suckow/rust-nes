use crate::memory::AddressSpace;
use bit_field::BitField;

pub struct ControllerState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
}

impl ControllerState {
    pub fn new_empty() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }

    pub fn new(
        up: bool,
        down: bool,
        left: bool,
        right: bool,
        a: bool,
        b: bool,
        start: bool,
        select: bool,
    ) -> Self {
        Self {
            up,
            down,
            left,
            right,
            a,
            b,
            start,
            select,
        }
    }

    fn as_byte(&self) -> u8 {
        let mut byte: u8 = 0;
        byte.set_bit(4, self.up);
        byte.set_bit(5, self.down);
        byte.set_bit(6, self.left);
        byte.set_bit(7, self.right);
        byte.set_bit(0, self.a);
        byte.set_bit(1, self.b);
        byte.set_bit(3, self.start);
        byte.set_bit(2, self.select);
        byte
    }
}

pub struct Controller {
    pub state: ControllerState,
    polls: u8,
    strobe: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: ControllerState::new_empty(),
            polls: 0,
            strobe: false,
        }
    }

    pub fn update_controller(&mut self, state: ControllerState) {
        self.state = state;
    }
}

impl AddressSpace for Controller {
    fn peek(&mut self, ptr: u16) -> u8 {
        if ptr == 0x4016 {
            if self.polls > 7 {
                2
            } else {
                let val = self.state.as_byte().get_bit(self.polls as usize) as u8;

                self.polls += 1;

                val
            }
        } else {
            3
        }
    }

    fn poke(&mut self, _ptr: u16, byte: u8) {
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
