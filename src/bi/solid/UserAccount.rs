#[cfg(feature = "BiSolidUserAccount")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUserAccount")]
/// *This icon requires the feature* `BiSolidUserAccount` *to be enabled*.
#[component]
pub fn UserAccount(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H8a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2zm-6 2.5a2.5 2.5 0 1 1 0 5 2.5 2.5 0 0 1 0-5zM19 15H9v-.25C9 12.901 11.254 11 14 11s5 1.901 5 3.75V15z" /><path d="M4 8H2v12c0 1.103.897 2 2 2h12v-2H4V8z" /></svg>
   }
}