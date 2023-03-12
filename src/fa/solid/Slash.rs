#[cfg(feature = "FaSolidSlash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidSlash")]
/// *This icon requires the feature* `FaSolidSlash` *to be enabled*.
#[component]
pub fn Slash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M5.1 9.2C13.3-1.2 28.4-3.1 38.8 5.1l592 464c10.4 8.2 12.3 23.3 4.1 33.7s-23.3 12.3-33.7 4.1L9.2 42.9C-1.2 34.7-3.1 19.6 5.1 9.2z" /></svg>
   }
}