use std::time::{Duration, Instant};

use iced::widget::{column, container, row, text};
use iced::{time, Background, Border, Element, Length, Subscription, Task, Theme};

use crate::app::demo::{EditorBar, GlobalBar, PlotBar, TableBar, WorkspaceBar};
use crate::colors;
use crate::statusbar::{StatusBar, StatusBarModel};

const TICK_RATE: Duration = Duration::from_millis(120);

#[derive(Debug, Clone)]
enum Message {
	Tick(Instant),
}

struct Demo {
	editor: EditorBar,
	table: TableBar,
	plot: PlotBar,
	workspace: WorkspaceBar,
	global: GlobalBar,
}

pub fn run() -> iced::Result {
	iced::application(boot, update, view)
		.title("Plinthon")
		.subscription(subscription)
		.theme(theme)
		.antialiasing(true)
		.run()
}

fn boot() -> (Demo, Task<Message>) {
	let now = Instant::now();
	(
		Demo {
			editor: EditorBar::new(now),
			table: TableBar::new(now),
			plot: PlotBar::new(now),
			workspace: WorkspaceBar::new(now),
			global: GlobalBar::new(now),
		},
		Task::none(),
	)
}

fn update(demo: &mut Demo, message: Message) -> Task<Message> {
	match message {
		Message::Tick(now) => {
			demo.editor.tick(now);
			demo.table.tick(now);
			demo.plot.tick(now);
			demo.workspace.tick(now);
			demo.global.tick(now);
		}
	}
	Task::none()
}

fn subscription(_demo: &Demo) -> Subscription<Message> {
	time::every(TICK_RATE).map(Message::Tick)
}

fn view(demo: &Demo) -> Element<'_, Message> {
	let editor = panel(
		"Code Editor",
		&[
			"Custom widget proving ground",
			"Modal editing, wrapping, diagnostics, execution context",
			"Status bar carries editor-specific execution state cleanly",
		],
		demo.editor.status_bar(),
	);
	let table = panel(
		"Data Table",
		&[
			"Large result virtualization",
			"Fetch progress, row counts, byte counts, sort/filter state",
			"Loading hints remain local to the table surface",
		],
		demo.table.status_bar(),
	);
	let plot = panel(
		"Plot Dashboard",
		&[
			"In-memory plotting for large series",
			"Separate render and export lifecycles",
			"Elapsed times stay visible without leaking concerns upward",
		],
		demo.plot.status_bar(),
	);
	let activity = panel(
		"Workspace Activity",
		&[
			"Notifications, background jobs, connection state",
			"Last operation summaries and failures",
			"One bar can summarize the whole workspace without hiding local bars",
		],
		demo.workspace.status_bar(),
	);

	let layout = column![
		header(),
		row![editor, table].spacing(16),
		row![plot, activity].spacing(16),
		demo.global.status_bar().view()
	]
	.spacing(16)
	.width(Length::Fill)
	.height(Length::Fill);

	container(layout)
		.padding(16)
		.width(Length::Fill)
		.height(Length::Fill)
		.style(|_theme| container::Style {
			background: Some(Background::Color(colors::APP_BACKGROUND)),
			..Default::default()
		})
		.into()
}

fn header<'a>() -> Element<'a, Message> {
	container(
		column![
			text("Plinthon").size(24).color(colors::APP_TEXT),
			text("Status instrumentation for local panes and workspace activity.")
				.size(13)
				.color(colors::STATUS_BAR_TEXT)
		]
		.spacing(4),
	)
	.padding([2, 2])
	.into()
}

fn panel<'a>(title: &'a str, lines: &'a [&'a str], status: StatusBar) -> Element<'a, Message> {
	let mut body = column![text(title).size(16).color(colors::APP_TEXT)].spacing(7);
	for line in lines {
		body = body.push(text(*line).size(13).color(colors::APP_TEXT_MUTED));
	}
	container(column![
		container(body).padding(14).height(Length::Fill),
		status.view()
	])
	.width(Length::FillPortion(1))
	.height(Length::FillPortion(1))
	.style(|_theme| container::Style {
		background: Some(Background::Color(colors::PANEL_BACKGROUND)),
		border: Border {
			color: colors::PANEL_BORDER,
			width: 1.0,
			radius: 0.0.into(),
		},
		..Default::default()
	})
	.into()
}

fn theme(_demo: &Demo) -> Theme {
	Theme::Oxocarbon
}
