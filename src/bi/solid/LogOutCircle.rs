#[cfg(feature = "BiSolidLogOutCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidLogOutCircle")]
/// *This icon requires the feature* `BiSolidLogOutCircle` *to be enabled*.
#[component]
pub fn LogOutCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 3c-4.963 0-9 4.037-9 9v.001l5-4v3h7v2H8v3l-5-4C3.001 16.964 7.037 21 12 21s9-4.037 9-9-4.037-9-9-9z" /></svg>
   }
}