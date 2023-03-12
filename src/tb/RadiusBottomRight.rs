#[cfg(feature = "TbRadiusBottomRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRadiusBottomRight")]
/// *This icon requires the feature* `TbRadiusBottomRight` *to be enabled*.
#[component]
pub fn RadiusBottomRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-radius-bottom-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M19 5v6a8 8 0 0 1 -8 8h-6" /></svg>
   }
}