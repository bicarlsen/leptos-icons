#[cfg(feature = "OcSmFileBadge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmFileBadge")]
/// *This icon requires the feature* `OcSmFileBadge` *to be enabled*.
#[component]
pub fn FileBadge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M2.75 1.5a.25.25 0 0 0-.25.25v11.5c0 .138.112.25.25.25h3.5a.75.75 0 0 1 0 1.5h-3.5A1.75 1.75 0 0 1 1 13.25V1.75C1 .784 1.784 0 2.75 0h8a1.75 1.75 0 0 1 1.508.862.75.75 0 1 1-1.289.768.25.25 0 0 0-.219-.13h-8Z" /><path d="M8 7a3.999 3.999 0 0 1 7.605-1.733 4 4 0 0 1-1.115 4.863l.995 4.973a.75.75 0 0 1-.991.852l-2.409-.876a.248.248 0 0 0-.17 0l-2.409.876a.75.75 0 0 1-.991-.852l.994-4.973A3.994 3.994 0 0 1 8 7Zm4-2.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5Zm0 6.5c-.373 0-.745-.051-1.104-.154l-.649 3.243 1.155-.42c.386-.14.81-.14 1.196 0l1.155.42-.649-3.243A4.004 4.004 0 0 1 12 11Z" /></svg>
   }
}