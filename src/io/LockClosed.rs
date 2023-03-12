#[cfg(feature = "IoLockClosed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLockClosed")]
/// *This icon requires the feature* `IoLockClosed` *to be enabled*.
#[component]
pub fn LockClosed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M368,192H352V112a96,96,0,1,0-192,0v80H144a64.07,64.07,0,0,0-64,64V432a64.07,64.07,0,0,0,64,64H368a64.07,64.07,0,0,0,64-64V256A64.07,64.07,0,0,0,368,192Zm-48,0H192V112a64,64,0,1,1,128,0Z" /></svg>
   }
}