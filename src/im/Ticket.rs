#[cfg(feature = "ImTicket")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImTicket")]
/// *This icon requires the feature* `ImTicket` *to be enabled*.
#[component]
pub fn Ticket(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 5l2 2-4 4-2-2zM15.649 4.649l-1.149-1.149-0.5 0.5c-0.256 0.256-0.61 0.414-1 0.414-0.781 0-1.414-0.633-1.414-1.414 0-0.391 0.158-0.744 0.415-1l0.5-0.5-1.149-1.149c-0.468-0.468-1.235-0.468-1.703 0l-9.297 9.297c-0.468 0.468-0.468 1.235 0 1.703l1.149 1.149 0.499-0.499c0.256-0.256 0.61-0.415 1.001-0.415 0.781 0 1.414 0.633 1.414 1.414 0 0.391-0.158 0.744-0.415 1l-0.5 0.5 1.149 1.149c0.468 0.468 1.234 0.468 1.703 0l9.297-9.297c0.468-0.468 0.468-1.235 0-1.703zM7 13l-4-4 6-6 4 4-6 6z" /></svg>
   }
}