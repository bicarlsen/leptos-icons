#[cfg(feature = "TbZodiacAquarius")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbZodiacAquarius")]
/// *This icon requires the feature* `TbZodiacAquarius` *to be enabled*.
#[component]
pub fn ZodiacAquarius(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-zodiac-aquarius" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 10l3 -3l3 3l3 -3l3 3l3 -3l3 3" /><path d="M3 17l3 -3l3 3l3 -3l3 3l3 -3l3 3" /></svg>
   }
}