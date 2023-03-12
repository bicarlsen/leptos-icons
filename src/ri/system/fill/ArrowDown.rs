#[cfg(feature = "RiSystemFillArrowDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowDown")]
/// *This icon requires the feature* `RiSystemFillArrowDown` *to be enabled*.
#[component]
pub fn ArrowDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M13 12h7l-8 8-8-8h7V4h2z" /></g></svg>
   }
}