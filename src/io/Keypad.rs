#[cfg(feature = "IoKeypad")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoKeypad")]
/// *This icon requires the feature* `IoKeypad` *to be enabled*.
#[component]
pub fn Keypad(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,400a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M256,272a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M256,144a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M256,16a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M384,272a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M384,144a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M384,16a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M128,272a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M128,144a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /><path d="M128,16a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /></svg>
   }
}