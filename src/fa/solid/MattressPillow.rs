#[cfg(feature = "FaSolidMattressPillow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidMattressPillow")]
/// *This icon requires the feature* `FaSolidMattressPillow` *to be enabled*.
#[component]
pub fn MattressPillow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M256 64H64C28.7 64 0 92.7 0 128V384c0 35.3 28.7 64 64 64H256V64zm32 384H576c35.3 0 64-28.7 64-64V128c0-35.3-28.7-64-64-64H288V448zM64 160c0-17.7 14.3-32 32-32h64c17.7 0 32 14.3 32 32V352c0 17.7-14.3 32-32 32H96c-17.7 0-32-14.3-32-32V160z" /></svg>
   }
}