#[cfg(feature = "TbCone2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCone2")]
/// *This icon requires the feature* `TbCone2` *to be enabled*.
#[component]
pub fn Cone2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-cone-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 7m-7 0a7 3 0 1 0 14 0a7 3 0 1 0 -14 0" /><path d="M19 7v.5l-7 12.5l-7 -12.5v-.5" /></svg>
   }
}