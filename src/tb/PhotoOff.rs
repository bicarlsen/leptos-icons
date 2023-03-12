#[cfg(feature = "TbPhotoOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbPhotoOff")]
/// *This icon requires the feature* `TbPhotoOff` *to be enabled*.
#[component]
pub fn PhotoOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-photo-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 3l18 18" /><path d="M15 8l.01 0" /><path d="M19.121 19.122a3 3 0 0 1 -2.121 .878h-10a3 3 0 0 1 -3 -3v-10c0 -.833 .34 -1.587 .888 -2.131m3.112 -.869h9a3 3 0 0 1 3 3v9" /><path d="M4 15l4 -4c.928 -.893 2.072 -.893 3 0l5 5" /><path d="M16.32 12.34c.577 -.059 1.162 .162 1.68 .66l2 2" /></svg>
   }
}