#[cfg(feature = "FaSolidFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidFill")]
/// *This icon requires the feature* `FaSolidFill` *to be enabled*.
#[component]
pub fn Fill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M84.6 9.4C72.1-3.1 51.9-3.1 39.4 9.4s-12.5 32.8 0 45.3L120.7 136 28.6 228.1c-37.5 37.5-37.5 98.3 0 135.8L146.1 481.4c37.5 37.5 98.3 37.5 135.8 0L472.3 290.9c28.1-28.1 28.1-73.7 0-101.8L320.9 37.7c-28.1-28.1-73.7-28.1-101.8 0L166 90.7 84.6 9.4zM166 181.3l49.4 49.4c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L211.3 136l53.1-53.1c3.1-3.1 8.2-3.1 11.3 0L427.1 234.3c3.1 3.1 3.1 8.2 0 11.3L384.7 288H65.5c1.4-5.4 4.2-10.4 8.4-14.6L166 181.3z" /></svg>
   }
}