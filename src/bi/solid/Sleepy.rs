#[cfg(feature = "BiSolidSleepy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidSleepy")]
/// *This icon requires the feature* `BiSolidSleepy` *to be enabled*.
#[component]
pub fn Sleepy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm-4 9.01-2-.02C6.017 9.386 7.095 7 10 7v2c-1.924 0-1.998 1.805-2 2.01zM12 18c-1.657 0-3-1.119-3-2.5s1.343-2.5 3-2.5 3 1.119 3 2.5-1.343 2.5-3 2.5zm5-7-1 .008C15.992 10.536 15.826 9 14 9V7c2.935 0 4 2.393 4 4h-1z" /></svg>
   }
}