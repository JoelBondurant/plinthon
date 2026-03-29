use std::time::{Duration, Instant};

use iced::widget::{column, container, progress_bar, row, text, Space};
use iced::{Alignment, Background, Border, Element, Length, Shadow};

use crate::colors;
use crate::statusbar::spinner;

const STATUS_BAR_HEIGHT: f32 = 26.0;
const SEGMENT_HEIGHT: f32 = 18.0;
const STATUS_BAR_INSET_X: f32 = 4.0;
const STATUS_BAR_INSET_BOTTOM: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
	#[default]
	Normal,
	Accent,
	Success,
	Warning,
	Danger,
}

#[derive(Debug, Clone)]
pub enum Segment {
	Text {
		value: String,
		tone: Tone,
	},
	Spinner {
		label: String,
		phase: usize,
		tone: Tone,
	},
	Progress {
		label: String,
		value: f32,
		value_text: String,
		tone: Tone,
	},
}

#[derive(Debug, Clone)]
pub struct StatusBar {
	left: Vec<Segment>,
	right: Vec<Segment>,
}

#[derive(Debug, Clone, Copy)]
pub struct Stopwatch {
	started_at: Instant,
}

impl Segment {
	pub fn text(value: impl Into<String>) -> Self {
		Self::Text {
			value: value.into(),
			tone: Tone::Normal,
		}
	}

	pub fn toned_text(value: impl Into<String>, tone: Tone) -> Self {
		Self::Text {
			value: value.into(),
			tone,
		}
	}

	pub fn spinner(label: impl Into<String>, phase: usize, tone: Tone) -> Self {
		Self::Spinner {
			label: label.into(),
			phase,
			tone,
		}
	}

	pub fn progress(
		label: impl Into<String>,
		value: f32,
		value_text: impl Into<String>,
		tone: Tone,
	) -> Self {
		Self::Progress {
			label: label.into(),
			value: value.clamp(0.0, 1.0),
			value_text: value_text.into(),
			tone,
		}
	}
}

impl StatusBar {
	pub fn new() -> Self {
		Self {
			left: Vec::new(),
			right: Vec::new(),
		}
	}

	pub fn left(mut self, segment: Segment) -> Self {
		self.left.push(segment);
		self
	}

	pub fn right(mut self, segment: Segment) -> Self {
		self.right.push(segment);
		self
	}

	pub fn view<Message: 'static>(&self) -> Element<'static, Message> {
		let left = container(lane::<Message>(&self.left))
			.width(Length::FillPortion(3))
			.height(Length::Fixed(STATUS_BAR_HEIGHT))
			.align_y(Alignment::Center)
			.clip(true);
		let right = container(lane::<Message>(&self.right))
			.width(Length::FillPortion(2))
			.height(Length::Fixed(STATUS_BAR_HEIGHT))
			.align_right(Length::Fill)
			.align_y(Alignment::Center)
			.clip(true);
		let rail = column![
			container(Space::new())
				.height(Length::Fixed(1.0))
				.style(|_theme| container::Style {
					background: Some(Background::Color(colors::BORDER_SUBTLE)),
					..Default::default()
				}),
			container(
				row![left, Space::new().width(Length::Fixed(4.0)), right]
					.align_y(Alignment::Center)
					.spacing(0)
					.clip(true),
			)
			.width(Length::Fill)
			.height(Length::Fixed(STATUS_BAR_HEIGHT))
			.padding([3, 5])
			.clip(true)
			.style(|_theme| container::Style {
				background: Some(Background::Color(colors::BG_STATUS)),
				..Default::default()
			})
		]
		.width(Length::Fill);
		column![
			container(rail)
				.padding([0, STATUS_BAR_INSET_X as u16])
				.width(Length::Fill),
			Space::new().height(Length::Fixed(STATUS_BAR_INSET_BOTTOM))
		]
		.width(Length::Fill)
		.into()
	}
}

impl Stopwatch {
	pub fn start(now: Instant) -> Self {
		Self { started_at: now }
	}

	pub fn restart(&mut self, now: Instant) {
		self.started_at = now;
	}

	pub fn elapsed(&self, now: Instant) -> Duration {
		now.saturating_duration_since(self.started_at)
	}

	pub fn segment(&self, now: Instant, label: impl Into<String>, tone: Tone) -> Segment {
		Segment::toned_text(
			format!("{} {}", label.into(), format_elapsed(self.elapsed(now))),
			tone,
		)
	}
}

fn lane<Message: 'static>(segments: &[Segment]) -> Element<'static, Message> {
	let mut content = row![].align_y(Alignment::Center).spacing(4);
	for segment in segments.iter().cloned() {
		content = content.push(segment_view(segment));
	}
	content
		.height(Length::Fixed(SEGMENT_HEIGHT))
		.clip(true)
		.into()
}

fn segment_view<Message: 'static>(segment: Segment) -> Element<'static, Message> {
	match segment {
		Segment::Text { value, tone } => {
			segment_shell(text(value).size(12).color(tone.text()), tone)
		}
		Segment::Spinner { label, phase, tone } => segment_shell(
			row![
				spinner::view(phase, tone),
				text(label).size(12).color(tone.text())
			]
			.spacing(5)
			.align_y(Alignment::Center),
			tone,
		),
		Segment::Progress {
			label,
			value,
			value_text,
			tone,
		} => segment_shell(
			row![
				text(label).size(12).color(tone.text()),
				container(
					progress_bar(0.0..=1.0, value)
						.length(Length::Fixed(74.0))
						.girth(Length::Fixed(10.0))
						.style(|_theme| iced::widget::progress_bar::Style {
							background: Background::Color(colors::BG_STATUS_BAR),
							bar: Background::Color(colors::STATUS_BAR),
							border: Border {
								color: colors::BORDER_SUBTLE,
								width: 1.0,
								radius: 2.0.into(),
							},
						}),
				)
				.width(74)
				.align_y(Alignment::Center),
				text(value_text).size(12).color(tone.text())
			]
			.spacing(6)
			.align_y(Alignment::Center),
			tone,
		),
	}
}

fn segment_shell<Message: 'static>(
	content: impl Into<Element<'static, Message>>,
	tone: Tone,
) -> Element<'static, Message> {
	container(content)
		.height(Length::Fixed(SEGMENT_HEIGHT))
		.padding([1, 6])
		.align_y(Alignment::Center)
		.clip(true)
		.style(move |_theme| container::Style {
			background: Some(Background::Color(tone.background())),
			border: Border {
				color: tone.border(),
				width: 1.0,
				radius: 3.0.into(),
			},
			shadow: Shadow::default(),
			..Default::default()
		})
		.into()
}

impl Tone {
	pub(crate) fn text(self) -> iced::Color {
		match self {
			Self::Normal => colors::TEXT_STATUS,
			Self::Accent => colors::TEXT_STATUS,
			Self::Success => colors::SUCCESS,
			Self::Warning => colors::WARNING,
			Self::Danger => colors::DANGER,
		}
	}

	fn border(self) -> iced::Color {
		let _ = self;
		colors::BORDER_SUBTLE
	}

	fn background(self) -> iced::Color {
		let _ = self;
		colors::BG_SEGMENT
	}
}

pub fn format_elapsed(duration: Duration) -> String {
	let total_ms = duration.as_millis() as u64;
	let millis = total_ms % 1000;
	let seconds = total_ms / 1000;
	let minutes = seconds / 60;
	let seconds = seconds % 60;
	if minutes > 0 {
		format!("{minutes:02}:{seconds:02}.{millis:03}")
	} else {
		format!("{seconds}.{millis:03}s")
	}
}
