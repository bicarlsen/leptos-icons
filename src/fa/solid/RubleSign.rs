#[cfg(feature = "FaSolidRubleSign")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidRubleSign")]
/// *This icon requires the feature* `FaSolidRubleSign` *to be enabled*.
#[component]
pub fn RubleSign(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M96 32C78.3 32 64 46.3 64 64V256H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H64v32H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H64v32c0 17.7 14.3 32 32 32s32-14.3 32-32V416H288c17.7 0 32-14.3 32-32s-14.3-32-32-32H128V320H240c79.5 0 144-64.5 144-144s-64.5-144-144-144H96zM240 256H128V96H240c44.2 0 80 35.8 80 80s-35.8 80-80 80z" /></svg>
   }
}