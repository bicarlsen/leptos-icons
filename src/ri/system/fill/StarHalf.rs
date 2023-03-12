#[cfg(feature = "RiSystemFillStarHalf")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillStarHalf")]
/// *This icon requires the feature* `RiSystemFillStarHalf` *to be enabled*.
#[component]
pub fn StarHalf(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 15.968l4.247 2.377-.949-4.773 3.573-3.305-4.833-.573L12 5.275v10.693zm0 2.292l-7.053 3.948 1.575-7.928L.587 8.792l8.027-.952L12 .5l3.386 7.34 8.027.952-5.935 5.488 1.575 7.928L12 18.26z" /></g></svg>
   }
}