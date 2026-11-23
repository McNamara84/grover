use leptos::prelude::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <p>"This is where GROVER is being developed - the Generator of Researcher-friendly, Ontology-aligned, Validated Excel templates for Research samples!"</p>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
