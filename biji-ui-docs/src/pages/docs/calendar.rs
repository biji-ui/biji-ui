use leptos::prelude::*;

use crate::components::{
    api_table::{
        DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable,
        SectionHeading,
    },
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"calendar\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::calendar;

#[component]
pub fn MyCalendar() -> impl IntoView {
    view! {
        <calendar::Root
            selection_type={calendar::SelectionType::Single}
            months=1
            week_starts_on={calendar::WeekStartsOn::Sunday}
        >
            <calendar::Header class="flex justify-between items-center mb-3">
                <calendar::PrevButton class="flex justify-center items-center w-7 h-7 text-sm rounded-md hover:bg-muted">
                    "‹"
                </calendar::PrevButton>
                <calendar::Heading class="text-sm font-medium cursor-pointer" />
                <calendar::NextButton class="flex justify-center items-center w-7 h-7 text-sm rounded-md hover:bg-muted">
                    "›"
                </calendar::NextButton>
            </calendar::Header>
            <calendar::Grid>
                <calendar::GridHead class="grid grid-cols-7 mb-1 text-xs text-center text-muted-foreground" />
                <calendar::GridBody
                    day_class="grid grid-cols-7 gap-y-1 [&_button]:aspect-square"
                    month_class="grid grid-cols-4 gap-1 [&_button]:py-2"
                    year_class="grid grid-cols-5 gap-1 [&_button]:py-2"
                />
            </calendar::Grid>
        </calendar::Root>
    }
}"#;

const RANGE_CODE: &str = r#"// Use CalendarValue::Range as the initial value.
let value = RwSignal::new(calendar::CalendarValue::Range {
    start: None,
    end: None,
});

view! {
    <calendar::Root
        value={value}
        selection_type={calendar::SelectionType::Range}
    >
        // ... (same Header and Grid structure as a single calendar)
    </calendar::Root>
}"#;

const MULTI_MONTH_CODE: &str = r#"// Set months=2 on Root, then render one Grid per month.
<calendar::Root months=2 selection_type={calendar::SelectionType::Range}>
    <calendar::Header class="flex justify-between items-center mb-3">
        <calendar::PrevButton>{"‹"}</calendar::PrevButton>
        // Heading automatically shows "March – April 2026" when months > 1.
        <calendar::Heading />
        <calendar::NextButton>{"›"}</calendar::NextButton>
    </calendar::Header>
    <div class="flex gap-6">
        <calendar::Grid month_offset=0>
            <calendar::GridHead />
            <calendar::GridBody />
        </calendar::Grid>
        <calendar::Grid month_offset=1>
            <calendar::GridHead />
            <calendar::GridBody />
        </calendar::Grid>
    </div>
</calendar::Root>"#;

const CONTROLLED_CODE: &str = r#"// The parent owns the value signal and writes to it directly.
let today = chrono::Local::now().date_naive();
let value = RwSignal::new(calendar::CalendarValue::Single(None));

// NavButtons must live inside <calendar::Root> to access CalendarContext,
// which lets it also navigate the displayed month when setting the value.
#[component]
fn NavButtons(value: RwSignal<calendar::CalendarValue>) -> impl IntoView {
    use biji_ui::components::calendar::{CalendarContext, CalendarValue};
    use chrono::Datelike;
    let ctx = expect_context::<CalendarContext>();
    let today = chrono::Local::now().date_naive();
    let ly = today.with_year(today.year() - 1).unwrap_or(today);
    let lw = today.checked_sub_signed(chrono::Duration::weeks(1)).unwrap_or(today);
    let nw = today.checked_add_signed(chrono::Duration::weeks(1)).unwrap_or(today);
    let ny = today.with_year(today.year() + 1).unwrap_or(today);
    view! {
        <button on:click=move |_| {
            value.set(CalendarValue::Single(Some(ly)));
            ctx.placeholder.set(ly.with_day(1).unwrap_or(ly));
        }>"Last Year"</button>
        // ... Last Week, Today, Next Week, Next Year follow the same pattern
    }
}

view! {
    <calendar::Root value={value} selection_type={calendar::SelectionType::Single}>
        <NavButtons value={value} />
        // ... header and grid
    </calendar::Root>
}"#;

const MIN_MAX_CODE: &str = r#"use chrono::Duration;

let today = chrono::Local::now().date_naive();
let week_ago = today.checked_sub_signed(Duration::weeks(1)).unwrap_or(today);

view! {
    <calendar::Root
        min_date={week_ago}
        max_date={today}
        selection_type={calendar::SelectionType::Single}
    >
        // ... header and grid
    </calendar::Root>
}"#;

const CUSTOM_DISABLED_CODE: &str = r#"// Disable weekends (Saturday and Sunday).
<calendar::Root
    selection_type={calendar::SelectionType::Single}
    is_date_disabled={Box::new(|date: chrono::NaiveDate| {
        use chrono::Datelike;
        matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
    })}
>
    // ... header and grid
</calendar::Root>"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the root element.",
    },
    PropRow {
        name: "selection_type",
        prop_type: "SelectionType",
        default: "Single",
        description: "Controls whether a single date, multiple dates, or a date range can be selected.",
    },
    PropRow {
        name: "months",
        prop_type: "usize",
        default: "1",
        description: "Number of month grids to display side-by-side. All grids share the same navigation.",
    },
    PropRow {
        name: "week_starts_on",
        prop_type: "WeekStartsOn",
        default: "Sunday",
        description: "Which day is treated as the first column in the week grid.",
    },
    PropRow {
        name: "value",
        prop_type: "Option<RwSignal<CalendarValue>>",
        default: "None",
        description: "Controlled value signal. When provided the calendar is controlled externally.",
    },
    PropRow {
        name: "default_value",
        prop_type: "Option<CalendarValue>",
        default: "None",
        description: "Initial value for uncontrolled mode.",
    },
    PropRow {
        name: "placeholder",
        prop_type: "Option<NaiveDate>",
        default: "today",
        description: "Overrides the initially displayed month. Pass this from the server to avoid SSR hydration mismatches near timezone boundaries.",
    },
    PropRow {
        name: "min_date",
        prop_type: "Option<NaiveDate>",
        default: "None",
        description: "Dates before this are disabled and unselectable.",
    },
    PropRow {
        name: "max_date",
        prop_type: "Option<NaiveDate>",
        default: "None",
        description: "Dates after this are disabled and unselectable.",
    },
    PropRow {
        name: "is_date_disabled",
        prop_type: "Option<Box<dyn Fn(NaiveDate) -> bool + Send + Sync>>",
        default: "None",
        description: "Custom predicate — return true to disable a specific date. Must be Send + Sync.",
    },
    PropRow {
        name: "on_change",
        prop_type: "Option<Callback<CalendarValue>>",
        default: "None",
        description: "Called whenever the selection changes. In controlled mode the external value signal is already updated before this fires — prefer reacting to the signal directly to avoid double-notification. Use on_change for out-of-band side effects such as persisting to a server.",
    },
];

const GRID_PROPS: &[PropRow] = &[
    PropRow {
        name: "month_offset",
        prop_type: "usize",
        default: "0",
        description: "Which month to display, offset from the anchor month. Use 0, 1, 2, … for multi-month layouts.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the grid wrapper.",
    },
];

const GRID_BODY_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class always applied to the body container (all views).",
    },
    PropRow {
        name: "day_class",
        prop_type: "String",
        default: "\"\"",
        description: "Additional class applied when showing the day grid (view = Day). Apply grid grid-cols-7 here. Each week row is wrapped in a role=\"row\" element with display:contents so column layout is inherited from this class.",
    },
    PropRow {
        name: "month_class",
        prop_type: "String",
        default: "\"\"",
        description: "Additional class applied when showing the month picker (view = Month). Apply grid grid-cols-4 here.",
    },
    PropRow {
        name: "year_class",
        prop_type: "String",
        default: "\"\"",
        description: "Additional class applied when showing the year picker (view = Year). Apply grid grid-cols-5 here for the 5-column decade grid.",
    },
];

const HEADER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the header wrapper div.",
}];

const PREV_BUTTON_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the previous-navigation button.",
}];

const NEXT_BUTTON_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the next-navigation button.",
}];

const HEADING_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the heading button. Clicking it cycles through Day → Month → Year views.",
}];

const GRID_HEAD_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the weekday header row. Invisible in Month and Year views (visibility: hidden) so it still occupies space and prevents layout shift.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-view",
        description: "On Root: \"day\", \"month\", or \"year\" — the current picker view.",
    },
    DataAttrRow {
        name: "data-today",
        description: "On day buttons: present when the date is today.",
    },
    DataAttrRow {
        name: "data-selected",
        description: "On day / month / year buttons: present when that cell is part of the current selection.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "On day buttons: present when the date is disabled via min_date, max_date, or is_date_disabled.",
    },
    DataAttrRow {
        name: "data-in-range",
        description: "On day buttons (Range mode): present for dates between the range start and end, including hover preview.",
    },
    DataAttrRow {
        name: "data-range-start",
        description: "On day buttons (Range mode): present on the selected range start date.",
    },
    DataAttrRow {
        name: "data-range-end",
        description: "On day buttons (Range mode): present on the selected range end date.",
    },
    DataAttrRow {
        name: "data-current-month",
        description: "On month buttons (Month view): present for the current calendar month.",
    },
    DataAttrRow {
        name: "data-current-year",
        description: "On year buttons (Year view): present for the current calendar year.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowLeft / ArrowRight",
        description: "Day: ±1 day. Month: ±1 month (wraps into adjacent year). Year: ±1 year (wraps into adjacent decade).",
    },
    KeyboardRow {
        key: "ArrowUp / ArrowDown",
        description: "Day: ±1 week. Month: ±4 months (one row). Year: ±5 years (one row).",
    },
    KeyboardRow {
        key: "Home",
        description: "Day: first day of month. Month: January of displayed year. Year: first year of the decade window.",
    },
    KeyboardRow {
        key: "End",
        description: "Day: last day of month. Month: December of displayed year. Year: last year of the decade window.",
    },
    KeyboardRow {
        key: "PageUp / PageDown",
        description: "Day: previous/next month. Month: same month, previous/next year. Year: previous/next decade.",
    },
    KeyboardRow {
        key: "Enter / Space",
        description: "Day: select the focused date. Month: drill into the focused month (switches to Day view). Year: drill into the focused year (switches to Month view).",
    },
];

#[component]
pub fn CalendarDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    let code_class =
        "[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm";

    view! {
        <DocPage title="Calendar">
            <p class="mt-3 mb-11 text-base text-balance">
                "A headless date picker with single, multiple, and range selection. Supports day, month, and year views with full keyboard navigation."
            </p>
            <DocPreview>
                <SingleCalendar />
            </DocPreview>
            <SectionHeading title="Installation" />
            <Code
                class={code_class}
                code={INSTALL_CODE}
                language="toml"
            />
            <SectionHeading title="Usage" />
            <Code class={code_class} code={USAGE_CODE} language="rust" />
            <SectionHeading title="Examples" />
            <h3 class="mt-8 mb-2 text-base font-semibold">"Range"</h3>
            <p class="mb-5 text-sm text-muted-foreground">
                "Pass "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "SelectionType::Range"
                </code>
                " to enable range selection. Clicking once sets the start date; clicking again sets the end. A hover preview highlights the candidate range before the second click."
            </p>
            <DocPreview>
                <RangeCalendar />
            </DocPreview>
            <Code class={code_class} code={RANGE_CODE} language="rust" />
            <h3 class="mt-8 mb-2 text-base font-semibold">"Multi-month"</h3>
            <p class="mb-5 text-sm text-muted-foreground">
                "Set "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"months=2"</code>
                " on " <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"Root"</code>
                " and render one "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"Grid"</code>
                " per month with "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"month_offset"</code>
                ". All grids share a single navigation, and "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"Heading"</code>
                " automatically shows the full span (e.g. \"March – April 2026\")."
            </p>
            <DocPreview>
                <MultiMonthCalendar />
            </DocPreview>
            <Code class={code_class} code={MULTI_MONTH_CODE} language="rust" />
            <h3 class="mt-8 mb-2 text-base font-semibold">"Controlled"</h3>
            <p class="mb-5 text-sm text-muted-foreground">
                "The " <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"value"</code>
                " signal is owned by the parent and can be written to at any time. Components inside "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"Root"</code>
                " can also access "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "CalendarContext"
                </code> " directly to navigate the displayed month alongside the value change."
            </p>
            <DocPreview>
                <ControlledCalendar />
            </DocPreview>
            <Code class={code_class} code={CONTROLLED_CODE} language="rust" />
            <h3 class="mt-8 mb-2 text-base font-semibold">"Date constraints"</h3>
            <p class="mb-5 text-sm text-muted-foreground">
                "Use "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"min_date"</code>
                " and "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"max_date"</code>
                " to restrict the selectable range. Dates outside the range are rendered with "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"data-disabled"</code>
                " and cannot be clicked or keyboard-navigated to."
            </p>
            <DocPreview>
                <MinMaxCalendar />
            </DocPreview>
            <Code class={code_class} code={MIN_MAX_CODE} language="rust" />
            <h3 class="mt-8 mb-2 text-base font-semibold">"Custom disabled dates"</h3>
            <p class="mb-5 text-sm text-muted-foreground">
                "For arbitrary rules use "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "is_date_disabled"
                </code> ". The predicate receives each date and returns "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"true"</code>
                " to disable it. Here weekends (Saturday and Sunday) are disabled."
            </p>
            <DocPreview>
                <CustomDisabledCalendar />
            </DocPreview>
            <Code class={code_class} code={CUSTOM_DISABLED_CODE} language="rust" />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="Header" rows={HEADER_PROPS} />
            <PropsTable title="PrevButton" rows={PREV_BUTTON_PROPS} />
            <PropsTable title="NextButton" rows={NEXT_BUTTON_PROPS} />
            <PropsTable title="Heading" rows={HEADING_PROPS} />
            <PropsTable title="Grid" rows={GRID_PROPS} />
            <PropsTable title="GridHead" rows={GRID_HEAD_PROPS} />
            <PropsTable title="GridBody" rows={GRID_BODY_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
fn SingleCalendar() -> impl IntoView {
    use biji_ui::components::calendar;

    let value = RwSignal::new(calendar::CalendarValue::Single(None));

    view! {
        <div class="flex flex-col gap-2">
            <p class="text-xs font-medium text-muted-foreground">"Single"</p>
            <calendar::Root value={value} selection_type={calendar::SelectionType::Single}>
                <CalendarShell />
            </calendar::Root>
        </div>
    }
}

#[component]
fn RangeCalendar() -> impl IntoView {
    use biji_ui::components::calendar;

    let value = RwSignal::new(calendar::CalendarValue::Range {
        start: None,
        end: None,
    });

    view! {
        <div class="flex flex-col gap-2">
            <p class="text-xs font-medium text-muted-foreground">"Range"</p>
            <calendar::Root value={value} selection_type={calendar::SelectionType::Range}>
                <CalendarShell />
            </calendar::Root>
        </div>
    }
}

#[component]
fn MultiMonthCalendar() -> impl IntoView {
    use biji_ui::components::calendar;

    let value = RwSignal::new(calendar::CalendarValue::Range {
        start: None,
        end: None,
    });

    let btn = "min-h-[253px] [&_button]:w-full [&_button]:text-sm [&_button]:rounded-md [&_button]:transition-colors [&_button:hover]:bg-muted [&_button[data-selected]]:bg-primary [&_button[data-selected]]:text-primary-foreground [&_button[data-disabled]]:opacity-30 [&_button[data-disabled]]:pointer-events-none";
    let day = "grid grid-cols-7 gap-y-1 [&_button]:aspect-square [&_button[data-today]:not([data-selected])]:font-bold [&_button[data-in-range]]:bg-primary/20 [&_button[data-range-start]]:bg-primary [&_button[data-range-start]]:text-primary-foreground [&_button[data-range-end]]:bg-primary [&_button[data-range-end]]:text-primary-foreground";
    let month = "content-start grid grid-cols-4 gap-1 [&_button]:py-2 [&_button]:text-center [&_button[data-current-month]]:font-bold";
    let year = "content-start grid grid-cols-5 gap-1 [&_button]:py-2 [&_button]:text-center [&_button[data-current-year]]:font-bold";
    let head = "grid grid-cols-7 text-center text-xs text-muted-foreground mb-1 [&>div]:py-1";

    view! {
        <div class="flex flex-col gap-2">
            <p class="text-xs font-medium text-muted-foreground">"Multi-month"</p>
            <calendar::Root value={value} selection_type={calendar::SelectionType::Range} months=2>
                <div class="p-3 mx-auto rounded-lg border select-none border-border w-fit">
                    <calendar::Header class="flex justify-between items-center mb-3">
                        <calendar::PrevButton class="flex justify-center items-center w-7 h-7 text-sm rounded-md transition-colors hover:bg-muted">
                            "\u{2039}"
                        </calendar::PrevButton>
                        <calendar::Heading class="py-1 px-2 text-sm font-medium rounded-md transition-colors cursor-pointer hover:bg-muted" />
                        <calendar::NextButton class="flex justify-center items-center w-7 h-7 text-sm rounded-md transition-colors hover:bg-muted">
                            "\u{203a}"
                        </calendar::NextButton>
                    </calendar::Header>
                    <div class="flex flex-col gap-4 sm:flex-row sm:gap-6">
                        <calendar::Grid month_offset=0 class="w-[272px]">
                            <calendar::GridHead class={head} />
                            <calendar::GridBody
                                class={btn}
                                day_class={day}
                                month_class={month}
                                year_class={year}
                            />
                        </calendar::Grid>
                        <DayOnlyGrid>
                            <calendar::Grid month_offset=1 class="w-[272px]">
                                <calendar::GridHead class={head} />
                                <calendar::GridBody
                                    class={btn}
                                    day_class={day}
                                    month_class={month}
                                    year_class={year}
                                />
                            </calendar::Grid>
                        </DayOnlyGrid>
                    </div>
                </div>
            </calendar::Root>
        </div>
    }
}

#[component]
fn ControlledCalendar() -> impl IntoView {
    use biji_ui::components::calendar;

    let value = RwSignal::new(calendar::CalendarValue::Single(None));

    view! {
        <div class="flex flex-col gap-2">
            <p class="text-xs font-medium text-muted-foreground">"Controlled"</p>
            <calendar::Root value={value} selection_type={calendar::SelectionType::Single}>
                <NavButtons value={value} />
                <CalendarShell />
            </calendar::Root>
        </div>
    }
}

#[component]
fn NavButtons(value: RwSignal<biji_ui::components::calendar::CalendarValue>) -> impl IntoView {
    use biji_ui::components::calendar::{CalendarContext, CalendarValue};
    use chrono::Datelike;

    let ctx = expect_context::<CalendarContext>();
    let today = chrono::Local::now().date_naive();
    let ly = today.with_year(today.year() - 1).unwrap_or(today);
    let lw = today
        .checked_sub_signed(chrono::Duration::weeks(1))
        .unwrap_or(today);
    let nw = today
        .checked_add_signed(chrono::Duration::weeks(1))
        .unwrap_or(today);
    let ny = today.with_year(today.year() + 1).unwrap_or(today);

    let btn = "px-3 py-1.5 text-xs font-medium rounded-full border border-border bg-background hover:bg-muted transition-colors cursor-pointer";

    view! {
        <div class="flex flex-wrap gap-2 justify-center mb-4 w-[272px]">
            <button
                class={btn}
                on:click={move |_| {
                    value.set(CalendarValue::Single(Some(ly)));
                    ctx.placeholder.set(ly.with_day(1).unwrap_or(ly));
                }}
            >
                "Last Year"
            </button>
            <button
                class={btn}
                on:click={move |_| {
                    value.set(CalendarValue::Single(Some(lw)));
                    ctx.placeholder.set(lw.with_day(1).unwrap_or(lw));
                }}
            >
                "Last Week"
            </button>
            <button
                class={btn}
                on:click={move |_| {
                    value.set(CalendarValue::Single(Some(today)));
                    ctx.placeholder.set(today.with_day(1).unwrap_or(today));
                }}
            >
                "Today"
            </button>
            <button
                class={btn}
                on:click={move |_| {
                    value.set(CalendarValue::Single(Some(nw)));
                    ctx.placeholder.set(nw.with_day(1).unwrap_or(nw));
                }}
            >
                "Next Week"
            </button>
            <button
                class={btn}
                on:click={move |_| {
                    value.set(CalendarValue::Single(Some(ny)));
                    ctx.placeholder.set(ny.with_day(1).unwrap_or(ny));
                }}
            >
                "Next Year"
            </button>
        </div>
    }
}

#[component]
fn CustomDisabledCalendar() -> impl IntoView {
    use biji_ui::components::calendar;

    let value = RwSignal::new(calendar::CalendarValue::Single(None));

    view! {
        <div class="flex flex-col gap-2">
            <p class="text-xs font-medium text-muted-foreground">"Custom disabled dates"</p>
            <calendar::Root
                value={value}
                selection_type={calendar::SelectionType::Single}
                is_date_disabled={Box::new(|date: chrono::NaiveDate| {
                    use chrono::Datelike;
                    matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
                })}
            >
                <CalendarShell />
            </calendar::Root>
        </div>
    }
}

#[component]
fn MinMaxCalendar() -> impl IntoView {
    use biji_ui::components::calendar;
    use chrono::Duration;

    let today = chrono::Local::now().date_naive();
    let week_ago = today
        .checked_sub_signed(Duration::weeks(1))
        .unwrap_or(today);

    let value = RwSignal::new(calendar::CalendarValue::Single(None));

    view! {
        <div class="flex flex-col gap-2">
            <p class="text-xs font-medium text-muted-foreground">"Date constraints"</p>
            <calendar::Root
                value={value}
                selection_type={calendar::SelectionType::Single}
                min_date={week_ago}
                max_date={today}
            >
                <CalendarShell />
            </calendar::Root>
        </div>
    }
}

/// Renders children only when the calendar is in Day view.
/// Used to hide secondary grids in month/year picker mode.
#[component]
fn DayOnlyGrid(children: ChildrenFn) -> impl IntoView {
    use biji_ui::components::calendar::{CalendarContext, CalendarView};
    let ctx = expect_context::<CalendarContext>();
    view! { <Show when={move || ctx.view.get() == CalendarView::Day}>{children()}</Show> }
}

/// Shared calendar layout used by both single and range examples.
#[component]
fn CalendarShell() -> impl IntoView {
    use biji_ui::components::calendar;

    // Common button styles shared across all views.
    let btn = "min-h-[253px] [&_button]:w-full [&_button]:text-sm [&_button]:rounded-md [&_button]:transition-colors [&_button:hover]:bg-muted [&_button[data-selected]]:bg-primary [&_button[data-selected]]:text-primary-foreground [&_button[data-disabled]]:opacity-30 [&_button[data-disabled]]:pointer-events-none";

    view! {
        <div class="p-3 rounded-lg border select-none border-border w-[272px]">
            <calendar::Header class="flex justify-between items-center mb-3">
                <calendar::PrevButton class="flex justify-center items-center w-7 h-7 text-sm rounded-md transition-colors hover:bg-muted">
                    "\u{2039}"
                </calendar::PrevButton>
                <calendar::Heading class="py-1 px-2 text-sm font-medium rounded-md transition-colors cursor-pointer hover:bg-muted" />
                <calendar::NextButton class="flex justify-center items-center w-7 h-7 text-sm rounded-md transition-colors hover:bg-muted">
                    "\u{203a}"
                </calendar::NextButton>
            </calendar::Header>
            <calendar::Grid>
                <calendar::GridHead class="grid grid-cols-7 text-center text-xs text-muted-foreground mb-1 [&>div]:py-1" />
                <calendar::GridBody
                    class={btn}
                    day_class="grid grid-cols-7 gap-y-1 [&_button]:aspect-square [&_button[data-today]:not([data-selected])]:font-bold [&_button[data-in-range]]:bg-primary/20 [&_button[data-range-start]]:bg-primary [&_button[data-range-start]]:text-primary-foreground [&_button[data-range-end]]:bg-primary [&_button[data-range-end]]:text-primary-foreground"
                    month_class="content-start grid grid-cols-4 gap-1 [&_button]:py-2 [&_button]:text-center [&_button[data-current-month]]:font-bold"
                    year_class="content-start grid grid-cols-5 gap-1 [&_button]:py-2 [&_button]:text-center [&_button[data-current-year]]:font-bold"
                />
            </calendar::Grid>
        </div>
    }
}
