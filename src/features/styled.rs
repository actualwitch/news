use leptos::prelude::*;

#[macro_export]
macro_rules! styled {
    ($component:ident, $tag:expr, $styles:expr) => {
        #[component]
        pub fn $component(
            #[prop(optional)] class: Option<String>,
            #[prop(optional)] style: Option<String>,
            #[prop(optional)] children: Option<Children>,
            #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
        ) -> impl IntoView {
            let class_value = class.unwrap_or_default();
            let style_value = format!("{}; {}", $styles, style.unwrap_or_default());
            
            view! {
                <$tag
                    class=class_value
                    style=style_value
                    ..attrs
                >
                    {children.map(|c| c())}
                </$tag>
            }
        }
    };
}