#[cfg(feature = "RiSystemFillArrowLeftDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowLeftDown")]
/// *This icon requires the feature* `RiSystemFillArrowLeftDown` *to be enabled*.
#[component]
pub fn ArrowLeftDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12.36 13.05L17.31 18H5.998V6.688l4.95 4.95 5.656-5.657 1.415 1.414z" /></g></svg>
   }
}