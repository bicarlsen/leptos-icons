#[cfg(feature = "TbCornerLeftUpDouble")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCornerLeftUpDouble")]
/// *This icon requires the feature* `TbCornerLeftUpDouble` *to be enabled*.
#[component]
pub fn CornerLeftUpDouble(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-corner-left-up-double" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 19h-6a3 3 0 0 1 -3 -3v-7" /><path d="M13 13l-4 -4l-4 4m8 -5l-4 -4l-4 4" /></svg>
   }
}