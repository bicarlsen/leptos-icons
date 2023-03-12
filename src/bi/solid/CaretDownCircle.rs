#[cfg(feature = "BiSolidCaretDownCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCaretDownCircle")]
/// *This icon requires the feature* `BiSolidCaretDownCircle` *to be enabled*.
#[component]
pub fn CaretDownCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10c5.515 0 10-4.486 10-10S17.515 2 12 2zm0 14-5-6h10l-5 6z" /></svg>
   }
}