#[cfg(feature = "FaSolidF")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidF")]
/// *This icon requires the feature* `FaSolidF` *to be enabled*.
#[component]
pub fn F(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M64 32C28.7 32 0 60.7 0 96V256 448c0 17.7 14.3 32 32 32s32-14.3 32-32V288H224c17.7 0 32-14.3 32-32s-14.3-32-32-32H64V96H288c17.7 0 32-14.3 32-32s-14.3-32-32-32H64z" /></svg>
   }
}