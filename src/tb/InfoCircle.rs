#[cfg(feature = "TbInfoCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbInfoCircle")]
/// *This icon requires the feature* `TbInfoCircle` *to be enabled*.
#[component]
pub fn InfoCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-circle" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M12 8l.01 0" /><path d="M11 12l1 0l0 4l1 0" /></svg>
   }
}