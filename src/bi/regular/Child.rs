#[cfg(feature = "BiRegularChild")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularChild")]
/// *This icon requires the feature* `BiRegularChild` *to be enabled*.
#[component]
pub fn Child(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="6" r="2" /><path d="M14 9h-4a1 1 0 0 0-.8.4l-3 4 1.6 1.2L9 13v7h2v-4h2v4h2v-7l1.2 1.6 1.6-1.2-3-4A1 1 0 0 0 14 9z" /></svg>
   }
}