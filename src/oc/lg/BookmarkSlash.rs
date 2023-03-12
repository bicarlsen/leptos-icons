#[cfg(feature = "OcLgBookmarkSlash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgBookmarkSlash")]
/// *This icon requires the feature* `OcLgBookmarkSlash` *to be enabled*.
#[component]
pub fn BookmarkSlash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M1.565 2.018v-.001l21.75 15.75a.75.75 0 1 1-.88 1.215L19 16.495v4.764a.748.748 0 0 1-1.219.584L12 17.21l-5.781 4.634A.75.75 0 0 1 5 21.259V6.357L.685 3.232a.75.75 0 0 1 .88-1.214ZM17.5 15.408l-11-7.965v12.254l5.031-4.032a.749.749 0 0 1 .938 0l5.031 4.032ZM7.25 2a.75.75 0 0 0 0 1.5h10a.25.25 0 0 1 .25.25v6.5a.75.75 0 0 0 1.5 0v-6.5A1.75 1.75 0 0 0 17.25 2h-10Z" /></svg>
   }
}