#[cfg(feature = "RiDesignLineTable")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignLineTable")]
/// *This icon requires the feature* `RiDesignLineTable` *to be enabled*.
#[component]
pub fn Table(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M4 8h16V5H4v3zm10 11v-9h-4v9h4zm2 0h4v-9h-4v9zm-8 0v-9H4v9h4zM3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1z" /></g></svg>
   }
}