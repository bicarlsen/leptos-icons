#[cfg(feature = "BiRegularMessageEdit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMessageEdit")]
/// *This icon requires the feature* `BiRegularMessageEdit` *to be enabled*.
#[component]
pub fn MessageEdit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3v3.767L13.277 18H20c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm0 14h-7.277L9 18.233V16H4V4h16v12z" /><path d="m13.803 9.189-1.399-1.398-3.869 3.864v1.399h1.399zm.327-3.123 1.398 1.399-1.066 1.066-1.399-1.398z" /></svg>
   }
}