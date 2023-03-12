#[cfg(feature = "FaSolidFolderClosed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidFolderClosed")]
/// *This icon requires the feature* `FaSolidFolderClosed` *to be enabled*.
#[component]
pub fn FolderClosed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M448 480H64c-35.3 0-64-28.7-64-64V192H512V416c0 35.3-28.7 64-64 64zm64-320H0V96C0 60.7 28.7 32 64 32H192c20.1 0 39.1 9.5 51.2 25.6l19.2 25.6c6 8.1 15.5 12.8 25.6 12.8H448c35.3 0 64 28.7 64 64z" /></svg>
   }
}