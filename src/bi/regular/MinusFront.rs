#[cfg(feature = "BiRegularMinusFront")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMinusFront")]
/// *This icon requires the feature* `BiRegularMinusFront` *to be enabled*.
#[component]
pub fn MinusFront(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 16h3v3c0 1.103.897 2 2 2h9c1.103 0 2-.897 2-2v-9c0-1.103-.897-2-2-2h-3V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v9c0 1.103.897 2 2 2zm13.997 3H10v-9h9l-.003 9z" /></svg>
   }
}