#[cfg(feature = "FaSolidR")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidR")]
/// *This icon requires the feature* `FaSolidR` *to be enabled*.
#[component]
pub fn R(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M64 32C28.7 32 0 60.7 0 96V288 448c0 17.7 14.3 32 32 32s32-14.3 32-32V320h95.3L261.8 466.4c10.1 14.5 30.1 18 44.6 7.9s18-30.1 7.9-44.6L230.1 309.5C282.8 288.1 320 236.4 320 176c0-79.5-64.5-144-144-144H64zM176 256H64V96H176c44.2 0 80 35.8 80 80s-35.8 80-80 80z" /></svg>
   }
}