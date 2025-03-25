use comrak::{
    format_html, format_html_with_plugins, markdown_to_html_with_plugins, nodes::NodeValue,
    parse_document, plugins::syntect::SyntectAdapterBuilder, Arena, ExtensionOptions, Options,
    Plugins,
};
use leptos::prelude::*;
use std::cmp::min;


#[component]
pub fn Markdown(text: String) -> impl IntoView {
    let html = Memo::new(move |_| {
        let arena = Arena::new();

        let extension = ExtensionOptions::builder()
            .alerts(true)
            .table(true)
            .underline(true)
            .build();

        let options = Options {
            extension,
            ..Options::default()
        };

        let syntect = SyntectAdapterBuilder::new()
            .theme("base16-ocean.light")
            .build();
        let mut plugins = Plugins::default();

        plugins.render.codefence_syntax_highlighter = Some(&syntect);

        let root = parse_document(&arena, &text, &options);

        for node in root.children() {
            if let NodeValue::Heading(ref mut heading) = node.data.borrow_mut().value {
                heading.level = min(heading.level + 3, 6);
            }
        }

        let mut html = vec![];
        format_html_with_plugins(root, &options, &mut html, &plugins).unwrap();

        String::from_utf8(html).unwrap()
    });
    view! { <div inner_html=html /> }
}
