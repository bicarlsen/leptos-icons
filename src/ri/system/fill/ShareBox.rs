#[cfg(feature = "RiSystemFillShareBox")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillShareBox")]
/// *This icon requires the feature* `RiSystemFillShareBox` *to be enabled*.
#[component]
pub fn ShareBox(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M10 3v2H5v14h14v-5h2v6a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h6zm7.707 4.707L12 13.414 10.586 12l5.707-5.707L13 3h8v8l-3.293-3.293z" /></g></svg>
   }
}