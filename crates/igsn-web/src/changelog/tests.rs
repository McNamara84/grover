use super::*;

#[test]
fn the_page_shows_release_headings_in_order_starting_with_the_footer_version() {
    let html = view! { <ChangelogPage/> }.to_html();
    let headings: Vec<_> = html
        .split("<h2>")
        .skip(1)
        .map(|entry| entry.split_once("</h2>").unwrap().0)
        .collect();
    assert_eq!(headings.len(), RELEASES.len());
    assert!(headings[0].contains(CURRENT_VERSION));
    for (heading, release) in headings.iter().zip(RELEASES) {
        assert!(heading.contains(release.version));
        match &release.published {
            Some(date) => assert!(heading.contains(date.label)),
            None => {
                assert!(heading.contains("Unreleased"));
                assert!(!heading.contains("<time"));
            }
        }
    }
}

#[test]
fn a_published_entry_renders_its_date_and_keeps_changes_in_editorial_order() {
    let release = Release {
        version: "v1.0.0",
        published: Some(PublicationDate {
            iso: "2026-03-14",
            label: "March 14, 2026",
        }),
        changes: &[
            Change {
                category: "Fixed",
                description: "First change",
            },
            Change {
                category: "Improved",
                description: "Second change",
            },
        ],
    };
    let html = release_entry(&release).to_html();
    assert!(html.contains("v1.0.0"));
    assert!(html.contains(r#"<time datetime="2026-03-14">March 14, 2026</time>"#));
    assert!(!html.contains("Unreleased"));
    let (first, second) = html.split_once("First change").unwrap();
    assert!(first.contains("Fixed"));
    assert!(second.contains("Improved"));
    assert!(second.contains("Second change"));
}
