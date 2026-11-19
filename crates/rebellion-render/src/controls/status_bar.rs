//! Paint geometry for `CoolwinStatusBar` (`FUN_0060e150`, `WndProc`
//! `FUN_0060e460`). Colors and placement belong to each feature.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StatusBarLayout {
    pub fill_width: u32,
    pub ticks: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StatusBar {
    current: u32,
    maximum: u32,
    tick_interval: Option<u32>,
}

impl StatusBar {
    pub(crate) const fn new(current: u32, maximum: u32) -> Self {
        Self {
            current,
            maximum,
            tick_interval: None,
        }
    }

    pub(crate) const fn with_ticks(mut self, interval: u32) -> Self {
        self.tick_interval = Some(interval);
        self
    }

    pub(crate) fn layout(self, width: u32) -> StatusBarLayout {
        if self.maximum == 0 {
            return StatusBarLayout {
                fill_width: 0,
                ticks: Vec::new(),
            };
        }
        let fill_width = width.saturating_mul(self.current.min(self.maximum)) / self.maximum;
        let mut ticks = Vec::new();
        if let Some(interval) = self.tick_interval.filter(|interval| *interval > 0) {
            for value in (0..self.maximum).step_by(interval as usize) {
                ticks.push(value.saturating_mul(width) / self.maximum);
            }
        }
        StatusBarLayout { fill_width, ticks }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_bar_uses_the_original_integer_fill_geometry() {
        // FUN_0060e460: Rectangle right = width * current / maximum + 1;
        // the reusable geometry is the whole-pixel width before GDI's edge.
        assert_eq!(StatusBar::new(51, 100).layout(37).fill_width, 18);
        assert_eq!(StatusBar::new(150, 100).layout(37).fill_width, 37);
        assert_eq!(StatusBar::new(1, 0).layout(37).fill_width, 0);
    }

    #[test]
    fn status_bar_ticks_remain_visible_when_the_bar_is_empty() {
        // Tick painting is independent of the non-zero foreground branch in
        // FUN_0060e460, and iterates 0, interval, ... strictly below max.
        assert_eq!(
            StatusBar::new(0, 4).with_ticks(1).layout(12).ticks,
            vec![0, 3, 6, 9]
        );
        assert_eq!(
            StatusBar::new(50, 100).with_ticks(25).layout(40).ticks,
            vec![0, 10, 20, 30]
        );
        assert_eq!(
            StatusBar::new(50, 100).with_ticks(0).layout(40).ticks,
            Vec::<u32>::new()
        );
    }
}
