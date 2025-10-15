use leptos::prelude::*;

use uuid::Uuid;

#[component]
pub fn GridPattern(
    #[prop(into, optional)] width: i32,
    #[prop(into, optional)] height: i32,
    #[prop(into, optional)] x: i32,
    #[prop(into, optional)] y: i32,
    squares: Vec<(i32, i32)>,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let pattern_id = Uuid::new_v4();
    view! {
        <svg class={class} aria-hidden="true">
            <defs>
                <pattern
                    id={format!("{:?}", pattern_id)}
                    width={width}
                    height={height}
                    patternUnits="userSpaceOnUse"
                    x={x}
                    y={y}
                >
                    <path d={format!("M.5 {height}V.5H{width}")} fill="none"></path>
                </pattern>
            </defs>
            <rect
                width="100%"
                height="100%"
                stroke-width=0
                fill={format!("url(#{:?})", pattern_id)}
            ></rect>
            <svg x={x} y={y} class="overflow-visible">
                <For
                    each={move || squares.clone()}
                    key={|(x, y)| format!("{x}-{y}")}
                    children={move |(x, y)| {
                        view! {
                            <rect
                                stroke-width="0"
                                width={width + 1}
                                height={height + 1}
                                x={x * width}
                                y={y * height}
                            ></rect>
                        }
                    }}
                />

            </svg>
        </svg>
    }
}
