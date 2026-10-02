use leptos::prelude::*;

use crate::api::GetBibleApi;

mod api;

#[wasm_bindgen::prelude::wasm_bindgen(start, js_name = "appStart")]
fn main() {
    console_error_panic_hook::set_once();
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let _ = navigator.service_worker().register("/sw.js");
    }

    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (active_index, set_active_index) = signal(0);

    let date = jiff::Zoned::now().day();
    let offsets = [0, 30, 60, 90, 120];

    let psalms: Vec<_> = offsets
        .iter()
        .map(|offset| {
            let psalm_num = date + offset;
            LocalResource::new(move || async move {
                let api = GetBibleApi::new();
                api.get_psalm("web", psalm_num).await
            })
        })
        .collect();

    let next_psalm = move |_| {
        set_active_index.update(|i| {
            if *i < 4 {
                *i += 1;
            }
        });
    };

    let prev_psalm = move |_| {
        set_active_index.update(|i| {
            if *i > 0 {
                *i -= 1;
            }
        });
    };

    view! {
        <main class="min-h-screen bg-stone-950 text-stone-100 pt-8 px-4 flex flex-col items-center">
            <div class="w-full max-w-prose space-y-6">
                { move || RenderPsalm(RenderPsalmProps{ psalm: psalms[active_index.get()] }) }
                <footer class="flex justify-between items-center py-6">
                <button
                    class="px-4 py-2 bg-stone-900 hover:bg-stone-800 border border-stone-800 rounded-lg text-sm text-stone-300 transition"
                    on:click=prev_psalm
                >
                    Previous
                </button>
                <Indicator
                    active_index=active_index
                    callback=Callback::new(move |index| set_active_index.set(index))
                />
                <button
                    class="px-4 py-2 bg-stone-900 hover:bg-stone-800 border border-stone-800 rounded-lg text-sm text-stone-300 transition"
                    on:click=next_psalm
                >
                    Next
                </button>
                </footer>
            </div>
        </main>
    }
}

#[component]
fn RenderPsalm(psalm: LocalResource<String>) -> impl IntoView {
    view! {
        <p>
            <Suspense fallback=move || view! {"Loading..."}>
            { move || psalm.get().map(|p|  p.clone()) }
            </Suspense>
        </p>
    }
}

#[component]
fn Indicator(active_index: ReadSignal<usize>, callback: Callback<usize>) -> impl IntoView {
    view! {
        <p class="flex space-x-2">
            {move || {
                let current = active_index.get();
                (0..=4).map(move |i| {
                    view! {
                        <IndicatorButton
                            active=i == current
                            callback=Callback::new(move |_| callback.run(i))
                        />
                    }
                }).collect_view()
            }}
        </p>
    }
}

#[component]
fn IndicatorButton(active: bool, callback: Callback<()>) -> impl IntoView {
    view! {
        <button on:click=move |_| callback.run(())>
            { move || if active { "●" } else { "○" } }
        </button>
    }
}
