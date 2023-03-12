#[cfg(feature = "BiRegularDish")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDish")]
/// *This icon requires the feature* `BiRegularDish` *to be enabled*.
#[component]
pub fn Dish(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21 15c0-4.625-3.507-8.441-8-8.941V4h-2v2.059c-4.493.5-8 4.316-8 8.941v2h18v-2zM5 15c0-3.859 3.141-7 7-7s7 3.141 7 7H5zm-3 3h20v2H2z" /></svg>
   }
}