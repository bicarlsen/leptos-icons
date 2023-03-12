#[cfg(feature = "BiRegularBorderAll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBorderAll")]
/// *This icon requires the feature* `BiRegularBorderAll` *to be enabled*.
#[component]
pub fn BorderAll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M9 21h12V3H3v18h6zm10-4v2h-6v-6h6v4zM15 5h4v6h-6V5h2zM5 7V5h6v6H5V7zm0 12v-6h6v6H5z" /></svg>
   }
}