#[cfg(feature = "FaSolidBoxTissue")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidBoxTissue")]
/// *This icon requires the feature* `FaSolidBoxTissue` *to be enabled*.
#[component]
pub fn BoxTissue(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M92.5 0H208c40 0 52 24 64 48s24 48 64 48h85.2C436 96 448 108 448 122.8c0 3.4-.7 6.8-1.9 10L409.6 224 384 288H128l-16-64L64.9 35.4c-.6-2.3-.9-4.6-.9-6.9C64 12.8 76.8 0 92.5 0zM79 224l16 64H80c-8.8 0-16 7.2-16 16s7.2 16 16 16h48H384h48c8.8 0 16-7.2 16-16s-7.2-16-16-16H418.5l25.6-64H464c26.5 0 48 21.5 48 48V384H0V272c0-26.5 21.5-48 48-48H79zM0 416H512v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V416z" /></svg>
   }
}