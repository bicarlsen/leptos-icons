#[cfg(feature = "AiFilledLayout")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiFilledLayout")]
/// *This icon requires the feature* `AiFilledLayout` *to be enabled*.
#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M384 912h496c17.7 0 32-14.3 32-32V340H384v572zm496-800H384v164h528V144c0-17.7-14.3-32-32-32zm-768 32v736c0 17.7 14.3 32 32 32h176V112H144c-17.7 0-32 14.3-32 32z" /></svg>
   }
}