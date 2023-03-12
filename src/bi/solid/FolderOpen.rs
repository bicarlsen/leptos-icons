#[cfg(feature = "BiSolidFolderOpen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFolderOpen")]
/// *This icon requires the feature* `BiSolidFolderOpen` *to be enabled*.
#[component]
pub fn FolderOpen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M2.165 19.551c.186.28.499.449.835.449h15c.4 0 .762-.238.919-.606l3-7A.998.998 0 0 0 21 11h-1V8c0-1.103-.897-2-2-2h-6.655L8.789 4H4c-1.103 0-2 .897-2 2v13h.007a1 1 0 0 0 .158.551zM18 8v3H6c-.4 0-.762.238-.919.606L4 14.129V8h14z" /></svg>
   }
}