#[cfg(feature = "BiSolidFolderPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFolderPlus")]
/// *This icon requires the feature* `BiSolidFolderPlus` *to be enabled*.
#[component]
pub fn FolderPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 5h-9.586L8.707 3.293A.997.997 0 0 0 8 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V7c0-1.103-.897-2-2-2zm-4 9h-3v3h-2v-3H8v-2h3V9h2v3h3v2z" /></svg>
   }
}