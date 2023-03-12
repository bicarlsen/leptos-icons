#[cfg(feature = "FaSolidSquareMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidSquareMinus")]
/// *This icon requires the feature* `FaSolidSquareMinus` *to be enabled*.
#[component]
pub fn SquareMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64zm88 200H296c13.3 0 24 10.7 24 24s-10.7 24-24 24H152c-13.3 0-24-10.7-24-24s10.7-24 24-24z" /></svg>
   }
}