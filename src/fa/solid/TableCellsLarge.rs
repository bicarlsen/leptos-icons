#[cfg(feature = "FaSolidTableCellsLarge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidTableCellsLarge")]
/// *This icon requires the feature* `FaSolidTableCellsLarge` *to be enabled*.
#[component]
pub fn TableCellsLarge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M448 96V224H288V96H448zm0 192V416H288V288H448zM224 224H64V96H224V224zM64 288H224V416H64V288zM64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64z" /></svg>
   }
}