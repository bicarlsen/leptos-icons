#[cfg(feature = "BiSolidTerminal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidTerminal")]
/// *This icon requires the feature* `BiSolidTerminal` *to be enabled*.
#[component]
pub fn Terminal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 4H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2zM6.414 15.707 5 14.293 7.293 12 5 9.707l1.414-1.414L10.121 12l-3.707 3.707zM19 16h-7v-2h7v2z" /></svg>
   }
}