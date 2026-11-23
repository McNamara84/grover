use leptos::prelude::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

mod changelog;
mod footer;

use changelog::ChangelogPage;
use footer::Footer;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| page_notice("Page not found.")>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("changelog") view=ChangelogPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    page_notice(
        "This is where GROVER is being developed - the Generator of Researcher-friendly, Ontology-aligned, Validated Excel templates for Research samples!",
    )
}

fn page_notice(message: &'static str) -> impl IntoView {
    view! {
        <div class="page-notice">
            <p>{message}</p>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
