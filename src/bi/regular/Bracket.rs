#[cfg(feature = "BiRegularBracket")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBracket")]
/// *This icon requires the feature* `BiRegularBracket` *to be enabled*.
#[component]
pub fn Bracket(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 4V2H4v20h6v-2H6V4zm4 16v2h6V2h-6v2h4v16z" /></svg>
   }
}