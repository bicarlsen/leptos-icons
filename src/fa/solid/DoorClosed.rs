#[cfg(feature = "FaSolidDoorClosed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidDoorClosed")]
/// *This icon requires the feature* `FaSolidDoorClosed` *to be enabled*.
#[component]
pub fn DoorClosed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M96 64c0-35.3 28.7-64 64-64H416c35.3 0 64 28.7 64 64V448h64c17.7 0 32 14.3 32 32s-14.3 32-32 32H432 144 32c-17.7 0-32-14.3-32-32s14.3-32 32-32H96V64zM384 288a32 32 0 1 0 0-64 32 32 0 1 0 0 64z" /></svg>
   }
}