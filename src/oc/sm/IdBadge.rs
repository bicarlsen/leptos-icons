#[cfg(feature = "OcSmIdBadge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmIdBadge")]
/// *This icon requires the feature* `OcSmIdBadge` *to be enabled*.
#[component]
pub fn IdBadge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M3 7.5a.5.5 0 0 1 .5-.5h2a.5.5 0 0 1 .5.5v3a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-3Zm10 .25a.75.75 0 0 1-.75.75h-4.5a.75.75 0 0 1 0-1.5h4.5a.75.75 0 0 1 .75.75ZM10.25 11a.75.75 0 0 0 0-1.5h-2.5a.75.75 0 0 0 0 1.5h2.5Z" /><path d="M7.25 0h1.5c.966 0 1.75.784 1.75 1.75V3h3.75c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0 1 14.25 15H1.75A1.75 1.75 0 0 1 0 13.25v-8.5C0 3.784.784 3 1.75 3H5.5V1.75C5.5.784 6.284 0 7.25 0Zm3.232 4.5A1.75 1.75 0 0 1 8.75 6h-1.5a1.75 1.75 0 0 1-1.732-1.5H1.75a.25.25 0 0 0-.25.25v8.5c0 .138.112.25.25.25h12.5a.25.25 0 0 0 .25-.25v-8.5a.25.25 0 0 0-.25-.25ZM7 1.75v2.5c0 .138.112.25.25.25h1.5A.25.25 0 0 0 9 4.25v-2.5a.25.25 0 0 0-.25-.25h-1.5a.25.25 0 0 0-.25.25Z" /></svg>
   }
}