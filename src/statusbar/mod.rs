mod core;
mod spinner;

#[allow(unused_imports)] // public widget surface in a binary demo crate
pub use core::{
	Segment, SegmentWidth, StatusBar, StatusBarModel, StatusBarStyle, Stopwatch, Tone,
};
