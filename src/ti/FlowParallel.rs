#[cfg(feature = "TiFlowParallel")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiFlowParallel")]
/// *This icon requires the feature* `TiFlowParallel` *to be enabled*.
#[component]
pub fn FlowParallel(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M18 16.184v-8.368c1.161-.415 2-1.514 2-2.816 0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.302.839 2.401 2 2.815v8.369c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816zm-1-12.184c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm0 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1zM10 5c0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.302.839 2.401 2 2.815v8.369c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816v-8.368c1.161-.415 2-1.514 2-2.816zm-3-1c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm0 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1z" /></g></svg>
   }
}