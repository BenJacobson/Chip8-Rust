mod bit_manipulation;
mod display;
mod instructions;
mod key;
pub mod logger;
mod processor;

pub use key::Key as Chip8Key;
pub use key::Keys as Chip8Keys;
pub use processor::Processor as Chip8;

#[cfg(test)]
mod test_display;

#[cfg(test)]
mod test_execute_digits;

#[cfg(test)]
mod test_execute_flags;

#[cfg(test)]
mod test_execute_key_press;

#[cfg(test)]
mod test_instructions;

#[cfg(test)]
mod test_key;

#[cfg(test)]
mod test_processor;
