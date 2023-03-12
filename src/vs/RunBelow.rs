#[cfg(feature = "VsRunBelow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRunBelow")]
/// *This icon requires the feature* `VsRunBelow` *to be enabled*.
#[component]
pub fn RunBelow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M1.8 1.01l-.78.41v12l.78.42 9-6v-.83l-9-6zm.22 11.48V2.36l7.6 5.07-7.6 5.06zM12.85 15h-.71l-2.5-2.5.71-.71L12 13.44V8h1v5.45l1.65-1.65.71.71L12.85 15z" /></svg>
   }
}