#[cfg(feature = "OcLgMultiSelect")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgMultiSelect")]
/// *This icon requires the feature* `OcLgMultiSelect` *to be enabled*.
#[component]
pub fn MultiSelect(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8.75 11.5h11.5a.75.75 0 0 1 0 1.5H8.75a.75.75 0 0 1 0-1.5Zm0 6h11.5a.75.75 0 0 1 0 1.5H8.75a.75.75 0 0 1 0-1.5Zm-5-12h10a.75.75 0 0 1 0 1.5h-10a.75.75 0 0 1 0-1.5ZM5 12a1 1 0 1 1-2 0 1 1 0 0 1 2 0Zm-1 7a1 1 0 1 1 0-2 1 1 0 0 1 0 2ZM19.309 7.918l-2.245-2.501A.25.25 0 0 1 17.25 5h4.49a.25.25 0 0 1 .185.417l-2.244 2.5a.25.25 0 0 1-.372 0Z" /></svg>
   }
}