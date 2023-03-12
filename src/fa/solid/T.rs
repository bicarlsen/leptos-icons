#[cfg(feature = "FaSolidT")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidT")]
/// *This icon requires the feature* `FaSolidT` *to be enabled*.
#[component]
pub fn T(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M32 32C14.3 32 0 46.3 0 64S14.3 96 32 96H160V448c0 17.7 14.3 32 32 32s32-14.3 32-32V96H352c17.7 0 32-14.3 32-32s-14.3-32-32-32H192 32z" /></svg>
   }
}