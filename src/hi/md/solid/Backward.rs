#[cfg(feature = "HiMdSolidBackward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidBackward")]
/// *This icon requires the feature* `HiMdSolidBackward` *to be enabled*.
#[component]
pub fn Backward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path d="M7.71176 4.81895C8.71109 4.20172 10 4.92057 10 6.09515V9.06794C10.1044 8.93679 10.234 8.81991 10.389 8.72419L16.7118 4.81895C17.7111 4.20172 19 4.92057 19 6.09515V13.9056C19 15.0802 17.7111 15.7991 16.7118 15.1818L10.389 11.2766C10.234 11.1809 10.1044 11.064 10 10.9328V13.9056C10 15.0802 8.7111 15.7991 7.71176 15.1818L1.38899 11.2766C0.439979 10.6904 0.439975 9.31035 1.38899 8.72419L7.71176 4.81895Z" fill="#0F172A" /></svg>
   }
}