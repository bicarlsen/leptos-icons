#[cfg(feature = "TbNumber3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbNumber3")]
/// *This icon requires the feature* `TbNumber3` *to be enabled*.
#[component]
pub fn Number3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-number-3" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12a4 4 0 1 0 -4 -4" /><path d="M8 16a4 4 0 1 0 4 -4" /></svg>
   }
}