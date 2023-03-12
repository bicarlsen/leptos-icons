#[cfg(feature = "BiRegularFolderPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFolderPlus")]
/// *This icon requires the feature* `BiRegularFolderPlus` *to be enabled*.
#[component]
pub fn FolderPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13 9h-2v3H8v2h3v3h2v-3h3v-2h-3z" /><path d="M20 5h-8.586L9.707 3.293A.996.996 0 0 0 9 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V7c0-1.103-.897-2-2-2zM4 19V7h16l.002 12H4z" /></svg>
   }
}