use futures_core::Stream;
use futures_util::stream;
use serde::{Deserialize, Serialize};
use topcoat::{
    Result,
    context::Cx,
    datastar::{ElementPatchMode, PatchElements, PatchSignals, Signals},
    router::{
        content::sse::{Event, Sse},
        route,
    },
    view::{component, view},
};

#[component]
pub async fn count_data() -> Result {
    view! {
        <div data-signals:count="0">
            <h1>
                "Count: "
                <span data-text="$count"></span>
            </h1>

            <p>
                <button class="btn btn-primary" data-on:click="@post('/increment')">
                    "Increment"
                </button>
            </p>
            <div class="flex w-full">
                <div
                    class="card bg-base-300 rounded-box grid h-20 grow place-items-center"
                >
                    <button class="btn btn-primary" data-on:click="@post('/inc')">
                        "Inc"
                    </button>
                </div>
                <div class="divider divider-horizontal">"OR"</div>
                <div
                    class="card bg-base-300 rounded-box grid h-20 grow place-items-center"
                >
                    <button class="btn btn-primary" data-on:click="@post('/dec')">
                        "dec"
                    </button>
                </div>
            </div>

            <ol id="log"></ol>
        </div>
    }
}

// Matches the signals declared by the page.
#[derive(Deserialize, Serialize)]
struct Counter {
    count: u64,
}

#[route(POST "/increment")]
async fn increment(
    cx: &Cx,
    Signals(counter): Signals<Counter>,
) -> Result<Sse<impl Stream<Item = Result<Event>> + use<>>> {
    let count = counter.count + 1;

    let entry = view! {
        <li>
            "Counted to "
            (count)
        </li>
    }?;

    // One event updates the counter signal, the other appends the log entry.
    let events = stream::iter([
        PatchSignals::json(&Counter { count }).map(Into::into),
        Ok(PatchElements::new(entry.render(cx))
            .selector("#log")
            .mode(ElementPatchMode::Append)
            .into()),
    ]);

    Ok(Sse::new(events))
}

#[route(POST "/inc")]
async fn inc(
    Signals(counter): Signals<Counter>,
) -> Result<Sse<impl Stream<Item = Result<Event>> + use<>>> {
    let count = counter.count + 1;

    // Convert PatchSignals to Event using .map(Event::from) or .map(Into::into)
    let events = stream::iter([PatchSignals::json(&Counter { count }).map(Event::from)]);

    Ok(Sse::new(events))
}

#[route(POST "/dec")]
async fn dec(
    Signals(counter): Signals<Counter>,
) -> Result<Sse<impl Stream<Item = Result<Event>> + use<>>> {
    // 1. Exit early if the incoming count is 0
    let mut count = counter.count;
    if count > 0 {
        count -= 1;
    }

    // Convert PatchSignals to Event using .map(Event::from) or .map(Into::into)
    let events = stream::iter([PatchSignals::json(&Counter { count }).map(Event::from)]);

    Ok(Sse::new(events))
}
