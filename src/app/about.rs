use crate::app::components;
use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

// A page in app::docs renders at /docs.
#[page("/about")]
async fn about_index() -> Result<impl View> {
    Ok(view! { <div>components::page::carousel::carousel()</div> })
}
