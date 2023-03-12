#[cfg(feature = "RiSystemFillSpam")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillSpam")]
/// *This icon requires the feature* `RiSystemFillSpam` *to be enabled*.
#[component]
pub fn Spam(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M17.5 2.5L23 12l-5.5 9.5h-11L1 12l5.5-9.5h11zM11 15v2h2v-2h-2zm0-8v6h2V7h-2z" /></g></svg>
   }
}