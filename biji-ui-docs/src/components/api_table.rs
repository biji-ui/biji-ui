use leptos::prelude::*;

#[derive(Copy, Clone)]
pub struct PropRow {
    pub name: &'static str,
    pub prop_type: &'static str,
    pub default: &'static str,
    pub description: &'static str,
}

#[derive(Copy, Clone)]
pub struct DataAttrRow {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Copy, Clone)]
pub struct KeyboardRow {
    pub key: &'static str,
    pub description: &'static str,
}

#[component]
pub fn SectionHeading(title: &'static str) -> impl IntoView {
    view! {
        <h2 class="mt-12 mb-4 text-lg font-semibold tracking-tight text-zinc-900 dark:text-white border-b border-border pb-2">
            {title}
        </h2>
    }
}

#[component]
pub fn PropsTable(title: &'static str, rows: &'static [PropRow]) -> impl IntoView {
    view! {
        <div class="mt-8">
            <h3 class="mb-3 text-sm font-semibold tracking-tight text-zinc-900 dark:text-white">
                {title}
            </h3>
            <div class="overflow-x-auto rounded-lg border border-border">
                <table class="w-full text-sm">
                    <thead class="bg-muted/50">
                        <tr>
                            <th class="px-4 py-2.5 text-left font-medium text-xs text-zinc-500 dark:text-zinc-400">
                                "Name"
                            </th>
                            <th class="px-4 py-2.5 text-left font-medium text-xs text-zinc-500 dark:text-zinc-400">
                                "Type"
                            </th>
                            <th class="px-4 py-2.5 text-left font-medium text-xs text-zinc-500 dark:text-zinc-400">
                                "Default"
                            </th>
                            <th class="px-4 py-2.5 text-left font-medium text-xs text-zinc-500 dark:text-zinc-400">
                                "Description"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {rows
                            .iter()
                            .map(|row| {
                                view! {
                                    <tr class="border-t border-border">
                                        <td class="px-4 py-2.5 font-mono text-xs text-violet-600 dark:text-violet-400">
                                            {row.name}
                                        </td>
                                        <td class="px-4 py-2.5 font-mono text-xs text-blue-600 dark:text-blue-400">
                                            {row.prop_type}
                                        </td>
                                        <td class="px-4 py-2.5 font-mono text-xs text-zinc-500 dark:text-zinc-400">
                                            {row.default}
                                        </td>
                                        <td class="px-4 py-2.5 text-xs text-zinc-600 dark:text-zinc-300">
                                            {row.description}
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn DataAttrsTable(rows: &'static [DataAttrRow]) -> impl IntoView {
    view! {
        <div class="mt-8">
            <h3 class="mb-3 text-sm font-semibold tracking-tight text-zinc-900 dark:text-white">
                "Data Attributes"
            </h3>
            <div class="overflow-x-auto rounded-lg border border-border">
                <table class="w-full text-sm">
                    <thead class="bg-muted/50">
                        <tr>
                            <th class="px-4 py-2.5 text-left font-medium text-xs text-zinc-500 dark:text-zinc-400">
                                "Attribute"
                            </th>
                            <th class="px-4 py-2.5 text-left font-medium text-xs text-zinc-500 dark:text-zinc-400">
                                "Description"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {rows
                            .iter()
                            .map(|row| {
                                view! {
                                    <tr class="border-t border-border">
                                        <td class="px-4 py-2.5 font-mono text-xs text-amber-600 dark:text-amber-400">
                                            {row.name}
                                        </td>
                                        <td class="px-4 py-2.5 text-xs text-zinc-600 dark:text-zinc-300">
                                            {row.description}
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn KeyboardTable(rows: &'static [KeyboardRow]) -> impl IntoView {
    view! {
        <div class="mt-8">
            <h3 class="mb-3 text-sm font-semibold tracking-tight text-zinc-900 dark:text-white">
                "Keyboard Navigation"
            </h3>
            <div class="overflow-x-auto rounded-lg border border-border">
                <table class="w-full text-sm">
                    <thead class="bg-muted/50">
                        <tr>
                            <th class="px-4 py-2.5 text-left font-medium text-xs text-zinc-500 dark:text-zinc-400">
                                "Key"
                            </th>
                            <th class="px-4 py-2.5 text-left font-medium text-xs text-zinc-500 dark:text-zinc-400">
                                "Description"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {rows
                            .iter()
                            .map(|row| {
                                view! {
                                    <tr class="border-t border-border">
                                        <td class="px-4 py-2.5">
                                            <kbd class="px-1.5 py-0.5 text-xs font-mono rounded border border-border bg-muted text-zinc-700 dark:text-zinc-300">
                                                {row.key}
                                            </kbd>
                                        </td>
                                        <td class="px-4 py-2.5 text-xs text-zinc-600 dark:text-zinc-300">
                                            {row.description}
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
