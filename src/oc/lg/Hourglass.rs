#[cfg(feature = "OcLgHourglass")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgHourglass")]
/// *This icon requires the feature* `OcLgHourglass` *to be enabled*.
#[component]
pub fn Hourglass(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4.75 2h14.5a.75.75 0 0 1 0 1.5h-.75v2.982a4.75 4.75 0 0 1-2.215 4.017l-2.044 1.29a.25.25 0 0 0 0 .422l2.044 1.29a4.75 4.75 0 0 1 2.215 4.017V20.5h.75a.75.75 0 0 1 0 1.5H4.75a.75.75 0 0 1 0-1.5h.75v-2.982a4.75 4.75 0 0 1 2.215-4.017l2.044-1.29a.25.25 0 0 0 0-.422l-2.044-1.29A4.75 4.75 0 0 1 5.5 6.482V3.5h-.75a.75.75 0 0 1 0-1.5ZM17 3.5H7v2.982A3.25 3.25 0 0 0 8.516 9.23l2.044 1.29a1.75 1.75 0 0 1 0 2.96l-2.044 1.29A3.25 3.25 0 0 0 7 17.518V20.5h10v-2.982a3.25 3.25 0 0 0-1.516-2.748l-2.044-1.29a1.75 1.75 0 0 1 0-2.96l2.044-1.29A3.25 3.25 0 0 0 17 6.482Z" /></svg>
   }
}