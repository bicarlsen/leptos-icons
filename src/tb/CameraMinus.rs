#[cfg(feature = "TbCameraMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCameraMinus")]
/// *This icon requires the feature* `TbCameraMinus` *to be enabled*.
#[component]
pub fn CameraMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-camera-minus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 13m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><path d="M5 7h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h3m9 6v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2" /><path d="M15 6l6 0" /></svg>
   }
}