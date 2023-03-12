#[cfg(feature = "TbMoodDollar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodDollar")]
/// *This icon requires the feature* `TbMoodDollar` *to be enabled*.
#[component]
pub fn MoodDollar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-dollar" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M21 12a9.003 9.003 0 0 0 -4.5 -7.794a9 9 0 1 0 -3.506 16.739" /><path d="M9 10h.01" /><path d="M15 10h.01" /><path d="M9.5 15c.658 .64 1.56 1 2.5 1c.357 0 .709 -.052 1.043 -.151" /><path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /><path d="M19 21v1m0 -8v1" /></svg>
   }
}