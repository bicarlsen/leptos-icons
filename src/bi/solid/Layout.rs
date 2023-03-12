#[cfg(feature = "BiSolidLayout")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidLayout")]
/// *This icon requires the feature* `BiSolidLayout` *to be enabled*.
#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 3H5c-1.103 0-2 .897-2 2v4h18V5c0-1.103-.897-2-2-2zM3 19c0 1.103.897 2 2 2h8V11H3v8zm12 2h4c1.103 0 2-.897 2-2v-8h-6v10z" /></svg>
   }
}