#[cfg(feature = "TbMoodTongueWink")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodTongueWink")]
/// *This icon requires the feature* `TbMoodTongueWink` *to be enabled*.
#[component]
pub fn MoodTongueWink(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-tongue-wink" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18z" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M9 10h.01" /><path d="M10 14v2a2 2 0 0 0 4 0v-2" /><path d="M15.5 14h-7" /><path d="M17 10c-.5 -1 -2.5 -1 -3 0" /></svg>
   }
}