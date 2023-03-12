#[cfg(feature = "BiRegularDumbbell")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDumbbell")]
/// *This icon requires the feature* `BiRegularDumbbell` *to be enabled*.
#[component]
pub fn Dumbbell(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 5v14h3v-6h6v6h3V5h-3v6H9V5zM3 15a1 1 0 0 0 1 1h1V8H4a1 1 0 0 0-1 1v2H2v2h1v2zm18-6a1 1 0 0 0-1-1h-1v8h1a1 1 0 0 0 1-1v-2h1v-2h-1V9z" /></svg>
   }
}