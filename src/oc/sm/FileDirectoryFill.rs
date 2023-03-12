#[cfg(feature = "OcSmFileDirectoryFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmFileDirectoryFill")]
/// *This icon requires the feature* `OcSmFileDirectoryFill` *to be enabled*.
#[component]
pub fn FileDirectoryFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M1.75 1A1.75 1.75 0 0 0 0 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0 0 16 13.25v-8.5A1.75 1.75 0 0 0 14.25 3H7.5a.25.25 0 0 1-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75Z" /></svg>
   }
}