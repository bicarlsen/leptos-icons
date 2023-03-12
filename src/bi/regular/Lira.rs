#[cfg(feature = "BiRegularLira")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularLira")]
/// *This icon requires the feature* `BiRegularLira` *to be enabled*.
#[component]
pub fn Lira(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M9 21h2c4.411 0 8-4.038 8-9h-2c0 3.86-2.691 7-6 7v-7.358l6-1.385V8.204l-6 1.385V7.642l6-1.385V4.204l-6 1.385V3H9v3.05l-3 .693v2.053l3-.692v1.947l-3 .692v2.053l3-.692V21z" /></svg>
   }
}