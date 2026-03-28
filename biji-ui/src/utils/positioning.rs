/// Controls how positioned overlays react when they would overflow the viewport.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum AvoidCollisions {
    /// Keeps the preferred side. Flips to the opposite side when there is not
    /// enough space. If neither side fits, uses whichever has more available space.
    #[default]
    Flip,
    /// Always places the overlay on the side with the most available space,
    /// regardless of the preferred positioning.
    AutoPlace,
    /// No collision detection. Always uses the exact `Positioning` specified.
    None,
}

#[derive(Copy, Clone, Debug)]
enum MainSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
enum Alignment {
    Center,
    Start,
    End,
}

/// Specifies where content should be positioned relative to a trigger element.
///
/// Each variant places the content along one of the four edges of the trigger,
/// with optional alignment (`Start` / `End`) along the perpendicular axis.
/// When no alignment suffix is given, the content is centered along that edge.
///
/// ```text
///         TopStart    Top    TopEnd
///            ┌─────────────────┐
/// LeftStart  │                 │  RightStart
///       Left │     trigger     │  Right
///  LeftEnd   │                 │  RightEnd
///            └─────────────────┘
///      BottomStart  Bottom  BottomEnd
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub enum Positioning {
    /// Centered above the trigger.
    #[default]
    Top,
    /// Above the trigger, aligned to the start (left) edge.
    TopStart,
    /// Above the trigger, aligned to the end (right) edge.
    TopEnd,
    /// Centered to the right of the trigger.
    Right,
    /// To the right of the trigger, aligned to the start (top) edge.
    RightStart,
    /// To the right of the trigger, aligned to the end (bottom) edge.
    RightEnd,
    /// Centered below the trigger.
    Bottom,
    /// Below the trigger, aligned to the start (left) edge.
    BottomStart,
    /// Below the trigger, aligned to the end (right) edge.
    BottomEnd,
    /// Centered to the left of the trigger.
    Left,
    /// To the left of the trigger, aligned to the start (top) edge.
    LeftStart,
    /// To the left of the trigger, aligned to the end (bottom) edge.
    LeftEnd,
}

impl Positioning {
    /// Calculate the position of content relative to a trigger element.
    ///
    /// Returns `(top, left)` coordinates in pixels, suitable for use with
    /// `position: fixed` CSS styling.
    ///
    /// # Arguments
    ///
    /// * `trigger_top` / `trigger_left` – the trigger element's viewport coordinates.
    /// * `trigger_width` / `trigger_height` – the trigger element's dimensions.
    /// * `content_height` / `content_width` – the content element's dimensions.
    /// * `offset` – additional spacing (in pixels) between the trigger and content.
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

    /// Calculate the position and rotation for an arrow indicator.
    ///
    /// Returns `(top, left, rotation)` where `top` and `left` are pixel
    /// coordinates and `rotation` is in degrees. The arrow is intended to
    /// be a small square element rotated so that one corner points toward
    /// the trigger.
    ///
    /// # Arguments
    ///
    /// * `trigger_top` / `trigger_left` – the trigger element's viewport coordinates.
    /// * `trigger_width` / `trigger_height` – the trigger element's dimensions.
    /// * `arrow_size` – the width/height of the square arrow element in pixels.
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

    /// Calculate position as a CSS `style` attribute string including arrow CSS custom properties.
    ///
    /// The returned string sets `position: fixed`, `top`, `left`, and three
    /// CSS custom properties consumed by the arrow element:
    ///
    /// * `--biji-tooltip-arrow-top`
    /// * `--biji-tooltip-arrow-left`
    /// * `--biji-tooltip-arrow-rotation`
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
            "position: fixed; top: {}px; left: {}px; --biji-transform-origin: {}; --biji-anchor-width: {}px; --biji-tooltip-arrow-top: {}px; --biji-tooltip-arrow-left: {}px; --biji-tooltip-arrow-rotation: {}deg;",
            position.0, position.1, self.transform_origin(), trigger_width, arrow_position.0, arrow_position.1, arrow_position.2
        )
    }

    fn main_side(self) -> MainSide {
        match self {
            Positioning::Top | Positioning::TopStart | Positioning::TopEnd => MainSide::Top,
            Positioning::Right | Positioning::RightStart | Positioning::RightEnd => MainSide::Right,
            Positioning::Bottom | Positioning::BottomStart | Positioning::BottomEnd => {
                MainSide::Bottom
            }
            Positioning::Left | Positioning::LeftStart | Positioning::LeftEnd => MainSide::Left,
        }
    }

    fn alignment(self) -> Alignment {
        match self {
            Positioning::TopStart
            | Positioning::RightStart
            | Positioning::BottomStart
            | Positioning::LeftStart => Alignment::Start,
            Positioning::TopEnd
            | Positioning::RightEnd
            | Positioning::BottomEnd
            | Positioning::LeftEnd => Alignment::End,
            _ => Alignment::Center,
        }
    }

    fn with_main_side(self, side: MainSide) -> Positioning {
        match (side, self.alignment()) {
            (MainSide::Top, Alignment::Center) => Positioning::Top,
            (MainSide::Top, Alignment::Start) => Positioning::TopStart,
            (MainSide::Top, Alignment::End) => Positioning::TopEnd,
            (MainSide::Right, Alignment::Center) => Positioning::Right,
            (MainSide::Right, Alignment::Start) => Positioning::RightStart,
            (MainSide::Right, Alignment::End) => Positioning::RightEnd,
            (MainSide::Bottom, Alignment::Center) => Positioning::Bottom,
            (MainSide::Bottom, Alignment::Start) => Positioning::BottomStart,
            (MainSide::Bottom, Alignment::End) => Positioning::BottomEnd,
            (MainSide::Left, Alignment::Center) => Positioning::Left,
            (MainSide::Left, Alignment::Start) => Positioning::LeftStart,
            (MainSide::Left, Alignment::End) => Positioning::LeftEnd,
        }
    }

    fn opposite(side: MainSide) -> MainSide {
        match side {
            MainSide::Top => MainSide::Bottom,
            MainSide::Bottom => MainSide::Top,
            MainSide::Left => MainSide::Right,
            MainSide::Right => MainSide::Left,
        }
    }

    /// Return the effective `Positioning` to use after applying collision avoidance.
    ///
    /// Computes which side actually has enough room given the current viewport,
    /// trigger rect, and content dimensions. The result can then be passed to
    /// `calculate_position_style` or `calculate_position_style_simple`.
    pub fn effective_positioning(
        self,
        content_width: f64,
        content_height: f64,
        trigger_top: f64,
        trigger_left: f64,
        trigger_width: f64,
        trigger_height: f64,
        offset: f64,
        viewport_width: f64,
        viewport_height: f64,
        avoid: AvoidCollisions,
    ) -> Positioning {
        if avoid == AvoidCollisions::None {
            return self;
        }

        let space_top = trigger_top - offset;
        let space_bottom = viewport_height - (trigger_top + trigger_height) - offset;
        let space_left = trigger_left - offset;
        let space_right = viewport_width - (trigger_left + trigger_width) - offset;

        let space_for = |side: MainSide| match side {
            MainSide::Top => space_top,
            MainSide::Bottom => space_bottom,
            MainSide::Left => space_left,
            MainSide::Right => space_right,
        };

        let required_for = |side: MainSide| match side {
            MainSide::Top | MainSide::Bottom => content_height,
            MainSide::Left | MainSide::Right => content_width,
        };

        match avoid {
            AvoidCollisions::Flip => {
                let preferred = self.main_side();
                if space_for(preferred) >= required_for(preferred) {
                    return self;
                }
                let opposite = Self::opposite(preferred);
                if space_for(opposite) >= required_for(opposite) {
                    return self.with_main_side(opposite);
                }
                // Neither fits — pick whichever has more space.
                if space_for(opposite) > space_for(preferred) {
                    self.with_main_side(opposite)
                } else {
                    self
                }
            }
            AvoidCollisions::AutoPlace => {
                let sides = [
                    MainSide::Top,
                    MainSide::Bottom,
                    MainSide::Left,
                    MainSide::Right,
                ];
                // Pick the side with the greatest surplus (available minus needed).
                let best = sides
                    .iter()
                    .copied()
                    .max_by(|&a, &b| {
                        let surplus_a = space_for(a) - required_for(a);
                        let surplus_b = space_for(b) - required_for(b);
                        surplus_a
                            .partial_cmp(&surplus_b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap_or(self.main_side());
                self.with_main_side(best)
            }
            AvoidCollisions::None => unreachable!("None is handled by early return above"),
        }
    }

    /// Calculate position as a simple CSS `style` attribute string without arrow variables.
    ///
    /// Returns a string containing only `position: fixed`, `top`, and `left`.
    /// Use this when an arrow indicator is not needed (e.g., dropdown menus).
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
            "position: fixed; top: {}px; left: {}px; --biji-transform-origin: {}; --biji-anchor-width: {}px;",
            position.0, position.1, self.transform_origin(), trigger_width
        )
    }

    /// Returns the CSS `transform-origin` value that points toward the trigger.
    ///
    /// Use via `origin-[var(--biji-transform-origin)]` (Tailwind arbitrary value)
    /// or `transform-origin: var(--biji-transform-origin)` in plain CSS to make
    /// scale animations emanate from the trigger side of the overlay.
    pub fn transform_origin(self) -> &'static str {
        match (self.main_side(), self.alignment()) {
            (MainSide::Top, Alignment::Start) => "bottom left",
            (MainSide::Top, Alignment::End) => "bottom right",
            (MainSide::Top, Alignment::Center) => "bottom center",
            (MainSide::Bottom, Alignment::Start) => "top left",
            (MainSide::Bottom, Alignment::End) => "top right",
            (MainSide::Bottom, Alignment::Center) => "top center",
            (MainSide::Left, Alignment::Start) => "right top",
            (MainSide::Left, Alignment::End) => "right bottom",
            (MainSide::Left, Alignment::Center) => "right center",
            (MainSide::Right, Alignment::Start) => "left top",
            (MainSide::Right, Alignment::End) => "left bottom",
            (MainSide::Right, Alignment::Center) => "left center",
        }
    }
}
