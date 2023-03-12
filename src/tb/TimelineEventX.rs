#[cfg(feature = "TbTimelineEventX")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbTimelineEventX")]
/// *This icon requires the feature* `TbTimelineEventX` *to be enabled*.
#[component]
pub fn TimelineEventX(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-timeline-event-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 20m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><path d="M10 20h-6" /><path d="M14 20h6" /><path d="M12 15l-2 -2h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-3l-2 2z" /><path d="M13.5 9.5l-3 -3" /><path d="M10.5 9.5l3 -3" /></svg>
   }
}