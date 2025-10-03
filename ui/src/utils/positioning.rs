#[derive(Copy, Clone, Debug)]
pub enum Positioning {
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

impl Default for Positioning {
    fn default() -> Self {
        Positioning::Top
    }
}

impl Positioning {
    /// Calculate the position of content relative to a trigger element
    /// Returns (top, left) coordinates in pixels
    pub fn calculate_position(
        self,
        trigger_top: f64,
        trigger_left: f64,
        trigger_width: f64,
        trigger_height: f64,
        content_height: f64,
        content_width: f64,
        offset: f64,
    ) -> (f64, f64) {
        match self {
            Positioning::Top => {
                let top = trigger_top - content_height - offset;
                let left = trigger_left + (trigger_width / 2.0) - (content_width / 2.0);
                (top, left)
            }
            Positioning::TopStart => {
                let top = trigger_top - content_height - offset;
                (top, trigger_left)
            }
            Positioning::TopEnd => {
                let top = trigger_top - content_height - offset;
                let left = trigger_left + trigger_width - content_width;
                (top, left)
            }
            Positioning::Right => {
                let top = trigger_top + (trigger_height / 2.0) - (content_height / 2.0);
                let left = trigger_left + trigger_width + offset;
                (top, left)
            }
            Positioning::RightStart => {
                let left = trigger_left + trigger_width + offset;
                (trigger_top, left)
            }
            Positioning::RightEnd => {
                let top = trigger_top + trigger_height - content_height;
                let left = trigger_left + trigger_width + offset;
                (top, left)
            }
            Positioning::Bottom => {
                let top = trigger_top + trigger_height + offset;
                let left = trigger_left + (trigger_width / 2.0) - (content_width / 2.0);
                (top, left)
            }
            Positioning::BottomStart => {
                let top = trigger_top + trigger_height + offset;
                (top, trigger_left)
            }
            Positioning::BottomEnd => {
                let top = trigger_top + trigger_height + offset;
                let left = trigger_left + trigger_width - content_width;
                (top, left)
            }
            Positioning::Left => {
                let top = trigger_top + (trigger_height / 2.0) - (content_height / 2.0);
                let left = trigger_left - content_width - offset;
                (top, left)
            }
            Positioning::LeftStart => {
                let left = trigger_left - content_width - offset;
                (trigger_top, left)
            }
            Positioning::LeftEnd => {
                let left = trigger_left - content_width - offset;
                let top = trigger_top + trigger_height - content_height;
                (top, left)
            }
        }
    }

    /// Calculate the position and rotation for an arrow indicator
    /// Returns (top, left, rotation) where rotation is in degrees
    pub fn calculate_arrow_position(
        self,
        trigger_top: f64,
        trigger_left: f64,
        trigger_width: f64,
        trigger_height: f64,
        arrow_size: f64,
    ) -> (f64, f64, i32) {
        match self {
            Positioning::Top | Positioning::TopStart | Positioning::TopEnd => {
                let top = trigger_top - arrow_size - (arrow_size / 2.0);
                let left = trigger_left + (trigger_width / 2.0) - (arrow_size / 2.0);
                (top, left, 225)
            }
            Positioning::Right | Positioning::RightStart | Positioning::RightEnd => {
                let top = trigger_top + (trigger_height / 2.0) - (arrow_size / 2.0);
                let left = trigger_left + trigger_width + (arrow_size / 2.0);
                (top, left, 315)
            }
            Positioning::Bottom | Positioning::BottomStart | Positioning::BottomEnd => {
                let top = trigger_top + trigger_height + arrow_size - (arrow_size / 2.0);
                let left = trigger_left + (trigger_width / 2.0) - (arrow_size / 2.0);
                (top, left, 45)
            }
            Positioning::Left | Positioning::LeftStart | Positioning::LeftEnd => {
                let top = trigger_top + (trigger_height / 2.0) - (arrow_size / 2.0);
                let left = trigger_left - arrow_size - (arrow_size / 2.0);
                (top, left, 135)
            }
        }
    }

    /// Calculate position as a CSS style string with arrow CSS variables
    pub fn calculate_position_style(
        self,
        trigger_top: f64,
        trigger_left: f64,
        trigger_width: f64,
        trigger_height: f64,
        content_height: f64,
        content_width: f64,
        offset: f64,
        arrow_size: f64,
    ) -> String {
        let position = self.calculate_position(
            trigger_top,
            trigger_left,
            trigger_width,
            trigger_height,
            content_height,
            content_width,
            offset,
        );
        let arrow_position = self.calculate_arrow_position(
            trigger_top,
            trigger_left,
            trigger_width,
            trigger_height,
            arrow_size,
        );
        format!(
            "position: fixed; top: {}px; left: {}px; --biji-tooltip-arrow-top: {}px; --biji-tooltip-arrow-left: {}px; --biji-tooltip-arrow-rotation: {}deg;",
            position.0, position.1, arrow_position.0, arrow_position.1, arrow_position.2
        )
    }

    /// Calculate position as a simple CSS style string without arrow variables
    pub fn calculate_position_style_simple(
        self,
        trigger_top: f64,
        trigger_left: f64,
        trigger_width: f64,
        trigger_height: f64,
        content_height: f64,
        content_width: f64,
        offset: f64,
    ) -> String {
        let position = self.calculate_position(
            trigger_top,
            trigger_left,
            trigger_width,
            trigger_height,
            content_height,
            content_width,
            offset,
        );
        format!(
            "position: fixed; top: {}px; left: {}px;",
            position.0, position.1
        )
    }
}
