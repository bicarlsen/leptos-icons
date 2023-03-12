#[cfg(feature = "BiSolidCompass")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCompass")]
/// *This icon requires the feature* `BiSolidCompass` *to be enabled*.
#[component]
pub fn Compass(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm3 13-8 2 2-8 8-2-2 8z" /><circle cx="12" cy="12" r="2" /></svg>
   }
}