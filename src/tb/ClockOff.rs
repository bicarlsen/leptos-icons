#[cfg(feature = "TbClockOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbClockOff")]
/// *This icon requires the feature* `TbClockOff` *to be enabled*.
#[component]
pub fn ClockOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-clock-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 7v1" /><path d="M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73" /><path d="M3 3l18 18" /></svg>
   }
}