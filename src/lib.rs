#![feature(fmt_as_str)]
#![cfg_attr(not(test), no_std)]

use iter_state_machine::examples::{Average, Pi};
use iter_state_machine::{Instruction, StateMachine};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_str, peek};
use pc_keyboard::{DecodedKey, KeyCode};
use lazy_static::lazy_static;
use core::fmt::Write;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Choice {
    Average, Pi
}

impl Choice {
    fn start(&self) -> Process {
        match self {
            Choice::Average => Process::Average(Average::new()),
            Choice::Pi => Process::Pi(Pi::new())
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Choice::Average => "Average",
            Choice::Pi => "Pi"
        }
    }
}

const NUM_PROGRAMS: usize = 2;
static PROGRAMS: [Choice; NUM_PROGRAMS] = [Choice::Average, Choice::Pi];

#[derive(Copy,Clone)]
enum Process {
    Average(Average),
    Pi(Pi)
}

impl StateMachine for Process {
    fn receive_input(&mut self, value: &str) -> Option<&'static str> {
        match self {
            Process::Average(avg) => avg.receive_input(value),
            Process::Pi(pi) => pi.receive_input(value)
        }
    }

    fn update(&mut self) {
        match self {
            Process::Average(avg) => avg.update(),
            Process::Pi(pi) => pi.update()
        }
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        match self {
            Process::Average(avg) => avg.next_instruction(),
            Process::Pi(pi) => pi.next_instruction()
        }
    }
}

lazy_static! {
    static ref TEXT_COLOR: ColorCode = ColorCode::new(Color::White, Color::Black);
    static ref REVERSE_COLOR: ColorCode = ColorCode::new(Color::Black, Color::White);
}

pub struct Kernel {

}

impl Kernel {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn draw(&mut self) {
        unimplemented!()
    }

    pub fn run_one_instruction(&mut self) {
        unimplemented!()
    }

    pub fn draw_proc_status(&mut self) {
        unimplemented!()
    }

    pub fn key(&mut self, key: DecodedKey) {
        unimplemented!()
    }
}
