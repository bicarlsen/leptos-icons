#[cfg(feature = "BiRegularMessageRoundedEdit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMessageRoundedEdit")]
/// *This icon requires the feature* `BiRegularMessageRoundedEdit` *to be enabled*.
#[component]
pub fn MessageRoundedEdit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 5.589 2 10c0 2.908 1.898 5.515 5 6.934V22l5.34-4.005C17.697 17.852 22 14.32 22 10c0-4.411-4.486-8-10-8zm0 14h-.333L9 18v-2.417l-.641-.247C5.67 14.301 4 12.256 4 10c0-3.309 3.589-6 8-6s8 2.691 8 6-3.589 6-8 6z" /><path d="M8.503 11.589v1.398h1.398l3.87-3.864-1.399-1.398zm5.927-3.125-1.398-1.398 1.067-1.067 1.398 1.398z" /></svg>
   }
}