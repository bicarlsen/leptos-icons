#[cfg(feature = "TbGenderDemiboy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbGenderDemiboy")]
/// *This icon requires the feature* `TbGenderDemiboy` *to be enabled*.
#[component]
pub fn GenderDemiboy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-gender-demiboy" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 14m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /><path d="M19 5l-5.4 5.4" /><path d="M19 5h-5" /></svg>
   }
}