#[cfg(feature = "BiRegularListPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularListPlus")]
/// *This icon requires the feature* `BiRegularListPlus` *to be enabled*.
#[component]
pub fn ListPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 15v-3h-2v3h-3v2h3v3h2v-3h3v-2h-.937zM4 7h11v2H4zm0 4h11v2H4zm0 4h8v2H4z" /></svg>
   }
}