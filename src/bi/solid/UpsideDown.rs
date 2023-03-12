#[cfg(feature = "BiSolidUpsideDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUpsideDown")]
/// *This icon requires the feature* `BiSolidUpsideDown` *to be enabled*.
#[component]
pub fn UpsideDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zM8.507 15a1.494 1.494 0 1 1 .001-2.987A1.494 1.494 0 0 1 8.507 15zm4.301-6.919a4.108 4.108 0 0 0-1.616 0 4.12 4.12 0 0 0-.751.233c-.234.1-.463.224-.678.368a4.077 4.077 0 0 0-1.08 1.082L7.024 8.646a6.026 6.026 0 0 1 2.639-2.175 6.097 6.097 0 0 1 1.128-.35 6.061 6.061 0 0 1 2.415 0 5.919 5.919 0 0 1 2.148.903 6.078 6.078 0 0 1 1.621 1.622l-1.658 1.117a3.994 3.994 0 0 0-.488-.59 3.988 3.988 0 0 0-2.021-1.092zM15.5 15a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 15.5 15z" /></svg>
   }
}