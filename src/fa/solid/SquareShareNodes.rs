#[cfg(feature = "FaSolidSquareShareNodes")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidSquareShareNodes")]
/// *This icon requires the feature* `FaSolidSquareShareNodes` *to be enabled*.
#[component]
pub fn SquareShareNodes(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64zM384 160c0 35.3-28.7 64-64 64c-15.4 0-29.5-5.4-40.6-14.5L194.1 256l85.3 46.5c11-9.1 25.2-14.5 40.6-14.5c35.3 0 64 28.7 64 64s-28.7 64-64 64s-64-28.7-64-64c0-2.5 .1-4.9 .4-7.3L174.5 300c-11.7 12.3-28.2 20-46.5 20c-35.3 0-64-28.7-64-64s28.7-64 64-64c18.3 0 34.8 7.7 46.5 20l81.9-44.7c-.3-2.4-.4-4.9-.4-7.3c0-35.3 28.7-64 64-64s64 28.7 64 64z" /></svg>
   }
}