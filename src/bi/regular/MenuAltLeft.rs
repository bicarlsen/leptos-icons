#[cfg(feature = "BiRegularMenuAltLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMenuAltLeft")]
/// *This icon requires the feature* `BiRegularMenuAltLeft` *to be enabled*.
#[component]
pub fn MenuAltLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 11h12v2H4zm0-5h16v2H4zm0 12h7.235v-2H4z" /></svg>
   }
}