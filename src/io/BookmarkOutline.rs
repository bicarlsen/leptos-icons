#[cfg(feature = "IoBookmarkOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBookmarkOutline")]
/// *This icon requires the feature* `IoBookmarkOutline` *to be enabled*.
#[component]
pub fn BookmarkOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M352,48H160a48,48,0,0,0-48,48V464L256,336,400,464V96A48,48,0,0,0,352,48Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}