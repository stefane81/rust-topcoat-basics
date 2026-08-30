use crate::app::components;
use topcoat::{Result, router::page, view::view};

// A page in app::docs renders at /docs.
#[page("/dashboard")]
async fn dashboard_index() -> Result {
    view! { <div>components::carousel::carousel()</div> }
}
