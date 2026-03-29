use iced::Color;

pub const APP_BACKGROUND: Color = rgb(1, 1, 1);
pub const PANEL_BACKGROUND: Color = rgb(4, 4, 4);
pub const PANEL_BORDER: Color = rgb(60, 8, 100);
pub const APP_TEXT: Color = rgb(230, 230, 230);
pub const APP_TEXT_MUTED: Color = rgb(200, 180, 200);

pub const STATUS_BAR_RAIL_BACKGROUND: Color = rgb(10, 10, 12);
pub const STATUS_BAR_RAIL_SEPARATOR: Color = rgb(24, 2, 32);
pub const STATUS_BAR_SEGMENT_BACKGROUND: Color = rgb(8, 8, 16);
pub const STATUS_BAR_SEGMENT_BORDER: Color = rgb(24, 2, 32);
pub const STATUS_BAR_TEXT: Color = rgb(200, 190, 210);
pub const STATUS_BAR_TEXT_ACCENT: Color = rgb(200, 190, 210);
pub const STATUS_BAR_TEXT_SUCCESS: Color = rgb(40, 120, 40);
pub const STATUS_BAR_TEXT_WARNING: Color = rgb(180, 40, 40);
pub const STATUS_BAR_TEXT_DANGER: Color = rgb(200, 80, 80);

pub const PROGRESS_BAR_TRACK_BACKGROUND: Color = rgb(1, 1, 1);
pub const PROGRESS_BAR_FILL: Color = rgb(150, 4, 250);

const fn rgb(r: u8, g: u8, b: u8) -> Color {
	Color::from_rgb8(r, g, b)
}
