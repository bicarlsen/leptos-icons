#[cfg(feature = "AiOutlinedExclamationCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedExclamationCircle")]
/// *This icon requires the feature* `AiOutlinedExclamationCircle` *to be enabled*.
#[component]
pub fn ExclamationCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z" /><path d="M464 688a48 48 0 1 0 96 0 48 48 0 1 0-96 0zm24-112h48c4.4 0 8-3.6 8-8V296c0-4.4-3.6-8-8-8h-48c-4.4 0-8 3.6-8 8v272c0 4.4 3.6 8 8 8z" /></svg>
   }
}