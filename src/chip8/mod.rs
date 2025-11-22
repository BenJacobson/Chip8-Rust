mod bit_manipulation;
mod processor;
mod instructions;

pub use processor::Processor as Chip8;
pub use processor::DISPLAY_PIXELS_X as DISPLAY_PIXELS_X;
pub use processor::DISPLAY_PIXELS_Y as DISPLAY_PIXELS_Y;

#[cfg(test)]
mod test_processor;
