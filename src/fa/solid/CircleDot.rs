#[cfg(feature = "FaSolidCircleDot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidCircleDot")]
/// *This icon requires the feature* `FaSolidCircleDot` *to be enabled*.
#[component]
pub fn CircleDot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M256 512c141.4 0 256-114.6 256-256S397.4 0 256 0S0 114.6 0 256S114.6 512 256 512zm0-160c-53 0-96-43-96-96s43-96 96-96s96 43 96 96s-43 96-96 96z" /></svg>
   }
}