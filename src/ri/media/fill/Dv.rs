#[cfg(feature = "RiMediaFillDv")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaFillDv")]
/// *This icon requires the feature* `RiMediaFillDv` *to be enabled*.
#[component]
pub fn Dv(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M4 14.745a7 7 0 1 1 8 0V21a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-6.255zM8 14A5 5 0 1 0 8 4a5 5 0 0 0 0 10zm-1 4v2h2v-2H7zm1-6a3 3 0 1 1 0-6 3 3 0 0 1 0 6zm6 5v-1.292A8.978 8.978 0 0 0 17 9a8.966 8.966 0 0 0-2.292-6H21a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1h-7zm4-10v2h2V7h-2z" /></g></svg>
   }
}