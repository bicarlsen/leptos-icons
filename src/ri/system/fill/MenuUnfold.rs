#[cfg(feature = "RiSystemFillMenuUnfold")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillMenuUnfold")]
/// *This icon requires the feature* `RiSystemFillMenuUnfold` *to be enabled*.
#[component]
pub fn MenuUnfold(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M21 18v2H3v-2h18zM17.05 3.55L22 8.5l-4.95 4.95v-9.9zM12 11v2H3v-2h9zm0-7v2H3V4h9z" /></g></svg>
   }
}