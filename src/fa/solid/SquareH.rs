#[cfg(feature = "FaSolidSquareH")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidSquareH")]
/// *This icon requires the feature* `FaSolidSquareH` *to be enabled*.
#[component]
pub fn SquareH(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64zM336 152V256 360c0 13.3-10.7 24-24 24s-24-10.7-24-24V280H160l0 80c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-208c0-13.3 10.7-24 24-24s24 10.7 24 24v80H288V152c0-13.3 10.7-24 24-24s24 10.7 24 24z" /></svg>
   }
}