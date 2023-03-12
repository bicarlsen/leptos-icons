#[cfg(feature = "VsRemoteExplorer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRemoteExplorer")]
/// *This icon requires the feature* `VsRemoteExplorer` *to be enabled*.
#[component]
pub fn RemoteExplorer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.344 2.125h20.312l.782.781v8.599a7.825 7.825 0 0 0-1.563-.912V3.688H2.125V17.75h7.813a7.813 7.813 0 0 0 1.562 4.688H5.25v-1.563h4.688v-1.563H1.344l-.782-.78V2.905l.782-.781zM17.75 11.5a6.25 6.25 0 1 0 0 12.5 6.25 6.25 0 0 0 0-12.5zm0 10.938a4.688 4.688 0 1 1 0-9.377 4.688 4.688 0 0 1 0 9.377zm2.603-3.132L18.2 17.153 20.353 15l.647.646-1.506 1.507L21 18.659l-.647.647zM15 17.246l1.506 1.507L15 20.259l.647.647 2.153-2.153-2.153-2.153-.647.646z" /></svg>
   }
}