#[cfg(feature = "FaSolidChalkboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidChalkboard")]
/// *This icon requires the feature* `FaSolidChalkboard` *to be enabled*.
#[component]
pub fn Chalkboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M96 32C60.7 32 32 60.7 32 96V384H96V96l384 0V384h64V96c0-35.3-28.7-64-64-64H96zM224 384v32H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H544c17.7 0 32-14.3 32-32s-14.3-32-32-32H416V384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32z" /></svg>
   }
}