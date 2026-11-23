use leptos::prelude::*;

struct Change {
    category: &'static str,
    description: &'static str,
}

struct PublicationDate {
    iso: &'static str,
    label: &'static str,
}

struct Release {
    version: &'static str,
    published: Option<PublicationDate>,
    changes: &'static [Change],
}

// Add new releases at the top. The footer reads the newest entry's version.
const RELEASES: &[Release] = &[Release {
    version: "v0.1.0dev",
    published: None,
    changes: &[Change {
        category: "Added",
        description: "Open the release notes from the version link in the footer and return home with “Back to GROVER”.",
    }],
}];

pub(super) const CURRENT_VERSION: &str = RELEASES[0].version;

#[component]
pub(super) fn ChangelogPage() -> impl IntoView {
    view! {
        <article class="changelog">
            <div class="changelog-content">
                <a class="changelog-back" href="/">
                    <span aria-hidden="true">"←"</span>"Back to GROVER"
                </a>
                <h1>"Release notes for GROVER"</h1>
                <ol class="release-list" role="list">
                    {RELEASES.iter().map(release_entry).collect_view()}
                </ol>
            </div>
        </article>
    }
}

fn release_entry(release: &Release) -> impl IntoView {
    let status = match &release.published {
        Some(date) => view! { <time datetime=date.iso>{date.label}</time> }.into_any(),
        None => view! { <span>"Unreleased"</span> }.into_any(),
    };
    let changes = release.changes.iter().map(|change| {
        view! {
            <li>
                <span class="change-category" data-category=change.category>{change.category}</span>
                <span>{change.description}</span>
            </li>
        }
    });
    view! {
        <li class="release-entry">
            <h2><span class="release-version">{release.version}</span>{status}</h2>
            <ul class="release-changes" role="list">{changes.collect_view()}</ul>
        </li>
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests;
