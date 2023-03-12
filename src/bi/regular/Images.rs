#[cfg(feature = "BiRegularImages")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularImages")]
/// *This icon requires the feature* `BiRegularImages` *to be enabled*.
#[component]
pub fn Images(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H8c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zM8 16V4h12l.002 12H8z" /><path d="M4 8H2v12c0 1.103.897 2 2 2h12v-2H4V8z" /><path d="m12 12-1-1-2 3h10l-4-6z" /></svg>
   }
}