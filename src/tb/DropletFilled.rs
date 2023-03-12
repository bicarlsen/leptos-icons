#[cfg(feature = "TbDropletFilled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDropletFilled")]
/// *This icon requires the feature* `TbDropletFilled` *to be enabled*.
#[component]
pub fn DropletFilled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-droplet-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6.801 11.003a6 6 0 1 0 10.396 -.003l-5.197 -8l-5.199 8.003z" stroke="#010202" stroke-width="0" fill="currentColor" /><path d="M12 3v17" stroke-width="0" fill="currentColor" /><path d="M12 12l3.544 -3.544" stroke-width="0" fill="currentColor" /><path d="M12 17.3l5.558 -5.558" stroke-width="0" fill="currentColor" /></svg>
   }
}