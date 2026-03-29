use std::time::{Duration, Instant};

use crate::statusbar::{Segment, StatusBar, StatusBarModel, Stopwatch, Tone};

#[derive(Debug, Clone)]
pub struct EditorBar {
	now: Instant,
	spinner_phase: usize,
	exec: LoopingJob,
}

#[derive(Debug, Clone)]
pub struct TableBar {
	now: Instant,
	spinner_phase: usize,
	fetch: LoopingJob,
}

#[derive(Debug, Clone)]
pub struct PlotBar {
	now: Instant,
	spinner_phase: usize,
	render: LoopingJob,
	export: LoopingJob,
}

#[derive(Debug, Clone)]
pub struct WorkspaceBar {
	now: Instant,
	spinner_phase: usize,
	workspace_io: LoopingJob,
}

#[derive(Debug, Clone)]
pub struct GlobalBar {
	now: Instant,
	spinner_phase: usize,
	cache_fill: LoopingJob,
}

#[derive(Debug, Clone)]
struct LoopingJob {
	timer: Stopwatch,
	work: Duration,
	pause: Duration,
}

impl EditorBar {
	pub fn new(now: Instant) -> Self {
		Self {
			now,
			spinner_phase: 0,
			exec: LoopingJob::new(now, Duration::from_secs(9), Duration::from_secs(3)),
		}
	}

}

impl StatusBarModel for EditorBar {
	fn tick(&mut self, now: Instant) {
		self.now = now;
		self.spinner_phase = self.spinner_phase.wrapping_add(1);
		self.exec.tick(now);
	}

	fn status_bar(&self) -> StatusBar {
		StatusBar::new()
			.left(Segment::toned_text("vim NORMAL", Tone::Accent))
			.left(Segment::text("wrap ON"))
			.left(Segment::text("UTF-8"))
			.left(Segment::label_value("Ln", "184, Col 19", Tone::Normal).reserve_chars(14))
			.right(
				Segment::label_value("sel", "3 rows / 92 chars", Tone::Normal)
					.max_chars(20)
					.reserve_chars(20),
			)
			.right(self.exec.timer_or_default(self.now, "exec", Tone::Accent).reserve_chars(12))
			.right(self.exec.progress_segment(self.now, "cpu", Tone::Accent, Tone::Normal))
			.right(
				self.exec
					.spinner_toggle(
						self.now,
						self.spinner_phase,
						"Query running",
						Tone::Accent,
						"Query idle",
						Tone::Success,
					)
					.reserve_chars(16),
			)
	}
}

impl TableBar {
	pub fn new(now: Instant) -> Self {
		Self {
			now,
			spinner_phase: 0,
			fetch: LoopingJob::new(now, Duration::from_secs(14), Duration::from_secs(2)),
		}
	}

}

impl StatusBarModel for TableBar {
	fn tick(&mut self, now: Instant) {
		self.now = now;
		self.spinner_phase = self.spinner_phase.wrapping_add(1);
		self.fetch.tick(now);
	}

	fn status_bar(&self) -> StatusBar {
		StatusBar::new()
			.left(Segment::label_value("rows", "18,420,114", Tone::Normal))
			.left(Segment::label_value("cols", "18", Tone::Normal))
			.left(Segment::label_value("loaded", "2.6 GiB", Tone::Normal).reserve_chars(16))
			.right(Segment::badge("sort ts desc", Tone::Accent).max_chars(12))
			.right(Segment::badge("filter region=us", Tone::Accent).max_chars(16))
			.right(self.fetch.progress_toggle(
				self.now,
				"fetch",
				Tone::Warning,
				"fetch complete",
				Tone::Success,
			))
			.right(
				self.fetch
					.spinner_toggle(
						self.now,
						self.spinner_phase,
						"virtualizing",
						Tone::Warning,
						"viewport warm",
						Tone::Normal,
					)
					.reserve_chars(14),
			)
	}
}

impl PlotBar {
	pub fn new(now: Instant) -> Self {
		Self {
			now,
			spinner_phase: 0,
			render: LoopingJob::new(
				now - Duration::from_secs(2),
				Duration::from_secs(8),
				Duration::from_secs(2),
			),
			export: LoopingJob::new(
				now - Duration::from_secs(5),
				Duration::from_secs(12),
				Duration::from_secs(5),
			),
		}
	}

}

impl StatusBarModel for PlotBar {
	fn tick(&mut self, now: Instant) {
		self.now = now;
		self.spinner_phase = self.spinner_phase.wrapping_add(1);
		self.render.tick(now);
		self.export.tick(now);
	}

	fn status_bar(&self) -> StatusBar {
		StatusBar::new()
			.left(self.render.spinner_toggle(
				self.now,
				self.spinner_phase,
				"rendering",
				Tone::Accent,
				"render cached",
				Tone::Success,
			))
			.left(self.render.timer_segment(self.now, "render", Tone::Accent).reserve_chars(12))
			.right(self.export.progress_toggle(
				self.now,
				"export svg",
				Tone::Accent,
				"export avif stalled",
				Tone::Danger,
			))
			.right(
				self.export
					.timer_with_tone(self.now, "export", Tone::Accent, Tone::Danger)
					.reserve_chars(12),
			)
	}
}

impl WorkspaceBar {
	pub fn new(now: Instant) -> Self {
		Self {
			now,
			spinner_phase: 0,
			workspace_io: LoopingJob::new(
				now - Duration::from_secs(3),
				Duration::from_secs(11),
				Duration::from_secs(4),
			),
		}
	}

}

impl StatusBarModel for WorkspaceBar {
	fn tick(&mut self, now: Instant) {
		self.now = now;
		self.spinner_phase = self.spinner_phase.wrapping_add(1);
		self.workspace_io.tick(now);
	}

	fn status_bar(&self) -> StatusBar {
		StatusBar::new()
			.left(Segment::toned_text("postgres mainline", Tone::Success))
			.left(
				Segment::spinner("jobs 3 active", self.spinner_phase, Tone::Accent)
					.fill_portion(1),
			)
			.left(self.workspace_io.scaled_progress_segment(
				self.now,
				"workspace io",
				0.7,
				0.15,
				Tone::Accent,
			))
			.right(Segment::badge("warning temp schema drift", Tone::Warning).max_chars(20))
			.right(Segment::label_value("last", "imported 18.4M rows", Tone::Normal).max_chars(24))
	}
}

impl GlobalBar {
	pub fn new(now: Instant) -> Self {
		Self {
			now,
			spinner_phase: 0,
			cache_fill: LoopingJob::new(
				now - Duration::from_secs(1),
				Duration::from_secs(10),
				Duration::from_secs(3),
			),
		}
	}

}

impl StatusBarModel for GlobalBar {
	fn tick(&mut self, now: Instant) {
		self.now = now;
		self.spinner_phase = self.spinner_phase.wrapping_add(1);
		self.cache_fill.tick(now);
	}

	fn status_bar(&self) -> StatusBar {
		StatusBar::new()
			.left(Segment::toned_text("connected", Tone::Success))
			.left(Segment::spinner(
				"background reconcile",
				self.spinner_phase,
				Tone::Accent,
			).compact())
			.left(self.cache_fill.scaled_progress_segment(
				self.now,
				"cache fill",
				0.6,
				0.2,
				Tone::Accent,
			))
			.right(Segment::badge("1 error export quota exceeded", Tone::Danger).max_chars(24))
			.right(Segment::badge("2 notices", Tone::Warning))
	}
}

impl LoopingJob {
	fn new(started_at: Instant, work: Duration, pause: Duration) -> Self {
		Self {
			timer: Stopwatch::start(started_at),
			work,
			pause,
		}
	}

	fn tick(&mut self, now: Instant) {
		if self.timer.elapsed(now) >= self.work + self.pause {
			self.timer.restart(now);
		}
	}

	fn running(&self, now: Instant) -> bool {
		self.timer.elapsed(now) < self.work
	}

	fn progress(&self, now: Instant) -> f32 {
		if !self.running(now) {
			return 1.0;
		}
		let elapsed = self.timer.elapsed(now).as_secs_f32();
		let total = self.work.as_secs_f32().max(0.001);
		(elapsed / total).clamp(0.0, 1.0)
	}

	fn spinner_toggle(
		&self,
		now: Instant,
		phase: usize,
		running_label: impl Into<String>,
		running_tone: Tone,
		idle_label: impl Into<String>,
		idle_tone: Tone,
	) -> Segment {
		Segment::spinner_toggle(
			self.running(now),
			running_label,
			phase,
			running_tone,
			idle_label,
			idle_tone,
		)
	}

	fn progress_toggle(
		&self,
		now: Instant,
		progress_label: impl Into<String>,
		progress_tone: Tone,
		idle_label: impl Into<String>,
		idle_tone: Tone,
	) -> Segment {
		Segment::progress_percent_toggle(
			self.running(now),
			progress_label,
			self.progress(now),
			progress_tone,
			idle_label,
			idle_tone,
		)
	}

	fn progress_segment(&self, now: Instant, label: impl Into<String>, active: Tone, idle: Tone) -> Segment {
		let tone = if self.running(now) { active } else { idle };
		Segment::progress_percent(label, self.progress(now), tone)
	}

	fn scaled_progress_segment(
		&self,
		now: Instant,
		label: impl Into<String>,
		scale: f32,
		offset: f32,
		tone: Tone,
	) -> Segment {
		Segment::progress_percent(label, (self.progress(now) * scale + offset).clamp(0.0, 1.0), tone)
	}

	fn timer_segment(&self, now: Instant, label: impl Into<String>, tone: Tone) -> Segment {
		self.timer.segment(now, label, tone)
	}

	fn timer_with_tone(
		&self,
		now: Instant,
		label: impl Into<String>,
		running_tone: Tone,
		idle_tone: Tone,
	) -> Segment {
		self.timer.segment(now, label, if self.running(now) { running_tone } else { idle_tone })
	}

	fn timer_or_default(
		&self,
		now: Instant,
		label: impl Into<String>,
		running_tone: Tone,
	) -> Segment {
		let label = label.into();
		if self.running(now) {
			self.timer.segment(now, label, running_tone)
		} else {
			Segment::label_value(label, "0.000s", Tone::Normal)
		}
	}
}
