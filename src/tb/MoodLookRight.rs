#[cfg(feature = "TbMoodLookRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodLookRight")]
/// *This icon requires the feature* `TbMoodLookRight` *to be enabled*.
#[component]
pub fn MoodLookRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-look-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18z" /><path d="M15 9h-.01" /><path d="M20 15h-4" /></svg>
   }
}