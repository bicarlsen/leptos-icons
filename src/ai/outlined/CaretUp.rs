#[cfg(feature = "AiOutlinedCaretUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedCaretUp")]
/// *This icon requires the feature* `AiOutlinedCaretUp` *to be enabled*.
#[component]
pub fn CaretUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024"><path d="M858.9 689L530.5 308.2c-9.4-10.9-27.5-10.9-37 0L165.1 689c-12.2 14.2-1.2 35 18.5 35h656.8c19.7 0 30.7-20.8 18.5-35z" /></svg>
   }
}