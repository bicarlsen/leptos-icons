#[cfg(feature = "BiRegularCalendarWeek")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCalendarWeek")]
/// *This icon requires the feature* `BiRegularCalendarWeek` *to be enabled*.
#[component]
pub fn CalendarWeek(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 4h-3V2h-2v2h-4V2H8v2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zM5 20V7h14V6l.002 14H5z" /><path d="M7 10v2h10V9H7z" /></svg>
   }
}