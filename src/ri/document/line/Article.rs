#[cfg(feature = "RiDocumentLineArticle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentLineArticle")]
/// *This icon requires the feature* `RiDocumentLineArticle` *to be enabled*.
#[component]
pub fn Article(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M20 22H4a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1zm-1-2V4H5v16h14zM7 6h4v4H7V6zm0 6h10v2H7v-2zm0 4h10v2H7v-2zm6-9h4v2h-4V7z" /></g></svg>
   }
}