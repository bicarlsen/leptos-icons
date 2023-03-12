#[cfg(feature = "FaSolidMagnet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidMagnet")]
/// *This icon requires the feature* `FaSolidMagnet` *to be enabled*.
#[component]
pub fn Magnet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M0 160v96C0 379.7 100.3 480 224 480s224-100.3 224-224V160H320v96c0 53-43 96-96 96s-96-43-96-96V160H0zm0-32H128V64c0-17.7-14.3-32-32-32H32C14.3 32 0 46.3 0 64v64zm320 0H448V64c0-17.7-14.3-32-32-32H352c-17.7 0-32 14.3-32 32v64z" /></svg>
   }
}