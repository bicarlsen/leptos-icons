#[cfg(feature = "TbFenceOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbFenceOff")]
/// *This icon requires the feature* `TbFenceOff` *to be enabled*.
#[component]
pub fn FenceOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-fence-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12h-8v4h12m4 0v-4h-4" /><path d="M6 16v4h4v-4" /><path d="M10 12v-2m0 -4l-2 -2m-2 2v6" /><path d="M14 16v4h4v-2" /><path d="M18 12v-6l-2 -2l-2 2v4" /><path d="M3 3l18 18" /></svg>
   }
}