#[cfg(feature = "BiSolidPizza")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPizza")]
/// *This icon requires the feature* `BiSolidPizza` *to be enabled*.
#[component]
pub fn Pizza(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M9.76 2.021a.995.995 0 0 0-.989.703L3.579 19.166a1 1 0 0 0 1.255 1.255l16.442-5.192a.991.991 0 0 0 .702-.988C21.6 7.666 16.334 2.4 9.76 2.021zM10 16a2 2 0 1 1 .001-4.001A2 2 0 0 1 10 16zm6-2a2 2 0 1 1 .001-4.001A2 2 0 0 1 16 14z" /></svg>
   }
}