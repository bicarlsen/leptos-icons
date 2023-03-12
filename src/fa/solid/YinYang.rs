#[cfg(feature = "FaSolidYinYang")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidYinYang")]
/// *This icon requires the feature* `FaSolidYinYang` *to be enabled*.
#[component]
pub fn YinYang(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M256 64c53 0 96 43 96 96s-43 96-96 96s-96 43-96 96s43 96 96 96C150 448 64 362 64 256S150 64 256 64zm0 448A256 256 0 1 0 256 0a256 256 0 1 0 0 512zm32-352a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM224 352a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z" /></svg>
   }
}