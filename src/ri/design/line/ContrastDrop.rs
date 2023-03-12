#[cfg(feature = "RiDesignLineContrastDrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignLineContrastDrop")]
/// *This icon requires the feature* `RiDesignLineContrastDrop` *to be enabled*.
#[component]
pub fn ContrastDrop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 3.1L7.05 8.05a7 7 0 1 0 9.9 0L12 3.1zm0-2.828l6.364 6.364a9 9 0 1 1-12.728 0L12 .272zM12 18V8a5 5 0 0 1 0 10z" /></g></svg>
   }
}