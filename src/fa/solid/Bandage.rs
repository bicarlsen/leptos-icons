#[cfg(feature = "FaSolidBandage")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidBandage")]
/// *This icon requires the feature* `FaSolidBandage` *to be enabled*.
#[component]
pub fn Bandage(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M480 416h96c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H480V416zM448 96H192V416H448V96zM64 96C28.7 96 0 124.7 0 160V352c0 35.3 28.7 64 64 64h96V96H64zM248 208a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm120-24a24 24 0 1 1 0 48 24 24 0 1 1 0-48zM248 304a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm120-24a24 24 0 1 1 0 48 24 24 0 1 1 0-48z" /></svg>
   }
}