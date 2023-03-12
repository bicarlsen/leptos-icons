#[cfg(feature = "RiSystemLineMenuUnfold")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineMenuUnfold")]
/// *This icon requires the feature* `RiSystemLineMenuUnfold` *to be enabled*.
#[component]
pub fn MenuUnfold(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M21 18v2H3v-2h18zM17.404 3.904L22 8.5l-4.596 4.596-1.414-1.414L19.172 8.5 15.99 5.318l1.414-1.414zM12 11v2H3v-2h9zm0-7v2H3V4h9z" /></g></svg>
   }
}