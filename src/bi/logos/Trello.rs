#[cfg(feature = "BiLogosTrello")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosTrello")]
/// *This icon requires the feature* `BiLogosTrello` *to be enabled*.
#[component]
pub fn Trello(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18.75 3H5.25A2.25 2.25 0 0 0 3 5.25v13.5A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V5.25A2.25 2.25 0 0 0 18.75 3zm-7.92 13.635a1.08 1.08 0 0 1-1.08 1.08H6.42a1.08 1.08 0 0 1-1.08-1.08V6.42c0-.597.483-1.08 1.08-1.08h3.33c.596 0 1.08.483 1.08 1.08v10.215zm7.83-4.5a1.08 1.08 0 0 1-1.08 1.08h-3.33a1.08 1.08 0 0 1-1.08-1.08V6.42c0-.597.484-1.08 1.08-1.08h3.33c.597 0 1.08.483 1.08 1.08v5.715z" /></svg>
   }
}