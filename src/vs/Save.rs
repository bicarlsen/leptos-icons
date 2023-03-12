#[cfg(feature = "VsSave")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSave")]
/// *This icon requires the feature* `VsSave` *to be enabled*.
#[component]
pub fn Save(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M13.353 1.146l1.5 1.5L15 3v11.5l-.5.5h-13l-.5-.5v-13l.5-.5H13l.353.146zM2 2v12h12V3.208L12.793 2H11v4H4V2H2zm6 0v3h2V2H8z" /></svg>
   }
}