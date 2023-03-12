#[cfg(feature = "BiSolidSave")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidSave")]
/// *This icon requires the feature* `BiSolidSave` *to be enabled*.
#[component]
pub fn Save(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 21h14a2 2 0 0 0 2-2V8l-5-5H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2zM7 5h4v2h2V5h2v4H7V5zm0 8h10v6H7v-6z" /></svg>
   }
}