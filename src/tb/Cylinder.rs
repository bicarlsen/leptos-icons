#[cfg(feature = "TbCylinder")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCylinder")]
/// *This icon requires the feature* `TbCylinder` *to be enabled*.
#[component]
pub fn Cylinder(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-cylinder" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 6m-5 0a5 3 0 1 0 10 0a5 3 0 1 0 -10 0" /><path d="M7 6v12c0 1.657 2.239 3 5 3s5 -1.343 5 -3v-12" /></svg>
   }
}