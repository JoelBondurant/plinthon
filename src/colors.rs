use iced::Color;

pub const BG_APP: Color = rgb(1, 1, 1);
pub const BG_PANEL: Color = rgb(4, 4, 4);
pub const BG_STATUS: Color = rgb(4, 4, 8);
pub const BG_SEGMENT: Color = rgb(8, 8, 16);
pub const STATUS_BAR: Color = rgb(150, 4, 250);
pub const BG_STATUS_BAR: Color = rgb(12, 8, 24);
pub const BORDER: Color = rgb(60, 8, 100);
pub const BORDER_SUBTLE: Color = rgb(24, 2, 32);
pub const TEXT: Color = rgb(230, 230, 230);
pub const TEXT_MUTED: Color = rgb(200, 180, 200);
pub const TEXT_STATUS: Color = rgb(210, 200, 220);
pub const SUCCESS: Color = rgb(40, 120, 40);
pub const WARNING: Color = rgb(180, 40, 40);
pub const DANGER: Color = rgb(200, 80, 80);
pub const ACCENT: Color = rgb(150, 4, 250);

const fn rgb(r: u8, g: u8, b: u8) -> Color {
	Color::from_rgb8(r, g, b)
}
