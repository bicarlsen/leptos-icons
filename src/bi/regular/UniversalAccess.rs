#[cfg(feature = "BiRegularUniversalAccess")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularUniversalAccess")]
/// *This icon requires the feature* `BiRegularUniversalAccess` *to be enabled*.
#[component]
pub fn UniversalAccess(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="7.5" r="1.5" /><path d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8z" /><path d="M16.5 10.5 16 9l-3 1h-2L8 9l-.5 1.5 3 1V13L9 17.25l1.5.75 1.25-3.5h.5L13.5 18l1.5-.75L13.5 13v-1.5l3-1z" /></svg>
   }
}