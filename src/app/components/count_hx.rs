use std::sync::atomic::{AtomicU64, Ordering};

use crate::app::Counter;

use topcoat::{
    Result,
    context::{Cx, app_context},
    htmx::HxResponseTrigger,
    router::{href, route},
    view::{View, component, view},
};

// struct Counter(AtomicU64);

#[component]
pub async fn count_hx() -> Result {
    view! {


        // Swaps the returned fragment into #count.

        <div class="flex w-full">
            <h1 class="grow place-items-center">
                "Count: "
                <span id="count">"0"</span>
            </h1>
            <div
                class="card bg-base-300 rounded-box grid h-20 grow place-items-center"
            >
            <button class="btn btn-primary" hx-post=(href!(inc_hx)) hx-target="#count" hx-swap="innerHTML">
                "Increment"
            </button>
            </div>
            <div class="divider divider-horizontal">"OR"</div>
            <div
                class="card bg-base-300 rounded-box grid h-20 grow place-items-center"
            >
            <button class="btn btn-primary" hx-post=(href!(dec_hx)) hx-target="#count" hx-swap="innerHTML">
                "Decrement"
            </button>
            </div>
        </div>
    }
}

#[route(POST "/inc_hx")]
async fn inc_hx(cx: &Cx) -> Result<(HxResponseTrigger, View)> {
    let count = app_context::<Counter>(cx).0.fetch_add(1, Ordering::Relaxed) + 1;
    let fragment = view! { <span id="count">(count)</span> }?;

    // The trigger becomes an `HX-Trigger: counted` response header, which
    // fires a `counted` event in the browser.
    Ok((HxResponseTrigger::receive(["counted"]), fragment))
}

#[route(POST "/dec_hx")]
async fn dec_hx(cx: &Cx) -> Result<(HxResponseTrigger, View)> {
    let count = app_context::<Counter>(cx).0.fetch_sub(1, Ordering::Relaxed) - 1;
    let fragment = view! { <span id="count">(count)</span> }?;

    // The trigger becomes an `HX-Trigger: counted` response header, which
    // fires a `counted` event in the browser.
    Ok((HxResponseTrigger::receive(["counted"]), fragment))
}
