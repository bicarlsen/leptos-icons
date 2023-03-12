#[cfg(feature = "SiElementor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiElementor")]
/// *This icon requires the feature* `SiElementor` *to be enabled*.
#[component]
pub fn Elementor(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0C5.372 0 0 5.372 0 12c0 6.626 5.372 12 12 12s12-5.372 12-12c0-6.626-5.372-12-12-12ZM9 17H7V7H9Zm8 0H11V15h6Zm0-4H11V11h6Zm0-4H11V7h6Z" /></svg>
   }
}