#[cfg(feature = "ImKey")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImKey")]
/// *This icon requires the feature* `ImKey` *to be enabled*.
#[component]
pub fn Key(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M11 0c-2.761 0-5 2.239-5 5 0 0.313 0.029 0.619 0.084 0.916l-6.084 6.084v3c0 0.552 0.448 1 1 1h1v-1h2v-2h2v-2h2l1.298-1.298c0.531 0.192 1.105 0.298 1.702 0.298 2.761 0 5-2.239 5-5s-2.239-5-5-5zM12.498 5.002c-0.828 0-1.5-0.672-1.5-1.5s0.672-1.5 1.5-1.5 1.5 0.672 1.5 1.5-0.672 1.5-1.5 1.5z" /></svg>
   }
}