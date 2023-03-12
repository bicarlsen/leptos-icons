#[cfg(feature = "FaSolidParagraph")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidParagraph")]
/// *This icon requires the feature* `FaSolidParagraph` *to be enabled*.
#[component]
pub fn Paragraph(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M192 32h64H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H384l0 352c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-352H288V448c0 17.7-14.3 32-32 32s-32-14.3-32-32V352H192c-88.4 0-160-71.6-160-160s71.6-160 160-160z" /></svg>
   }
}