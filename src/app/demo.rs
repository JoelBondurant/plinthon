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
		let progress = self.exec.progress(self.now);
		let running = self.exec.running(self.now);
		let state = if running {
			Segment::spinner("Query running", self.spinner_phase, Tone::Accent)
		} else {
			Segment::toned_text("Query idle", Tone::Success)
		};
		let exec = if running {
			self.exec.timer.segment(self.now, "exec", Tone::Accent)
		} else {
			Segment::label_value("exec", "0.000s", Tone::Normal)
		};
		StatusBar::new()
			.left(Segment::toned_text("vim NORMAL", Tone::Accent))
			.left(Segment::text("wrap ON"))
			.left(Segment::text("UTF-8"))
			.left(Segment::label_value("Ln", "184, Col 19", Tone::Normal))
			.right(
				Segment::label_value("sel", "3 rows / 92 chars", Tone::Normal)
					.max_chars(20)
					.fixed_width(138),
			)
			.right(Segment::progress_percent(
				"cpu",
				progress,
				if running { Tone::Accent } else { Tone::Normal },
			))
			.right(exec)
			.right(state)
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
		let progress = self.fetch.progress(self.now);
		let running = self.fetch.running(self.now);
		StatusBar::new()
			.left(Segment::label_value("rows", "18,420,114", Tone::Normal))
			.left(Segment::label_value("cols", "18", Tone::Normal))
			.left(Segment::label_value("loaded", "2.6 GiB", Tone::Normal).fixed_width(118))
			.right(Segment::badge("sort ts desc", Tone::Accent).max_chars(12))
			.right(Segment::badge("filter region=us", Tone::Accent).max_chars(16))
			.right(if running {
				Segment::progress_percent("fetch", progress, Tone::Warning)
			} else {
				Segment::toned_text("fetch complete", Tone::Success)
			})
			.right(if running {
				Segment::spinner("virtualizing", self.spinner_phase, Tone::Warning)
			} else {
				Segment::text("viewport warm")
			})
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
		let render_running = self.render.running(self.now);
		let export_running = self.export.running(self.now);
		StatusBar::new()
			.left(if render_running {
				Segment::spinner("rendering", self.spinner_phase, Tone::Accent)
			} else {
				Segment::toned_text("render cached", Tone::Success)
			})
			.left(self.render.timer.segment(self.now, "render", Tone::Accent))
			.right(if export_running {
				let export = self.export.progress(self.now);
				Segment::progress_percent("export svg", export, Tone::Accent)
			} else {
				Segment::toned_text("export avif stalled", Tone::Danger)
			})
			.right(self.export.timer.segment(
				self.now,
				"export",
				if export_running { Tone::Accent } else { Tone::Danger },
			))
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
		let io = self.workspace_io.progress(self.now);
		StatusBar::new()
			.left(Segment::toned_text("postgres mainline", Tone::Success))
			.left(
				Segment::spinner("jobs 3 active", self.spinner_phase, Tone::Accent)
					.fill_portion(1),
			)
			.left(Segment::progress_percent(
				"workspace io",
				(io * 0.7 + 0.15).clamp(0.0, 1.0),
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
		let fill = self.cache_fill.progress(self.now);
		StatusBar::new()
			.left(Segment::toned_text("connected", Tone::Success))
			.left(Segment::spinner(
				"background reconcile",
				self.spinner_phase,
				Tone::Accent,
			).compact())
			.left(Segment::progress_percent(
				"cache fill",
				(fill * 0.6 + 0.2).clamp(0.0, 1.0),
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
}
