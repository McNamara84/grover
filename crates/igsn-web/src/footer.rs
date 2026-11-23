use crate::changelog::CURRENT_VERSION;
use leptos::prelude::*;

#[component]
pub(super) fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <nav class="footer-info" aria-label="Site information">
                <a href="https://www.gfz.de/erklaerung-zur-barrierefreiheit" hreflang="de">
                    "Accessibility"
                </a>
                <a href="https://gfz-data-services.gfz.de/about-us/data-protection">"Data protection"</a>
                <a href="https://gfz-data-services.gfz.de/about-us/legal-notice">"Imprint"</a>
                <a href="/changelog">{CURRENT_VERSION}<span class="visually-hidden">" changelog"</span></a>
            </nav>
            <nav class="footer-social" aria-label="GFZ social media">
                <a href="https://www.instagram.com/gfz_potsdam/" aria-label="GFZ on Instagram"
                    inner_html=include_str!("icons/instagram.svg")/>
                <a href="https://www.youtube.com/user/GFZvideos" aria-label="GFZ on YouTube"
                    inner_html=include_str!("icons/youtube.svg")/>
                <a href="https://bsky.app/profile/gfz.bsky.social" aria-label="GFZ on Bluesky"
                    inner_html=include_str!("icons/bluesky.svg")/>
                <a href="https://de.linkedin.com/company/gfz-helmholtz-zentrum-fuer-geoforschung"
                    aria-label="GFZ on LinkedIn" inner_html=include_str!("icons/linkedin.svg")/>
            </nav>
        </footer>
    }
}
