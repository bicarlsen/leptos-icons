#[cfg(feature = "BiRegularRss")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRss")]
/// *This icon requires the feature* `BiRegularRss` *to be enabled*.
#[component]
pub fn Rss(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 20.001C19 11.729 12.271 5 4 5v2c7.168 0 13 5.832 13 13.001h2z" /><path d="M12 20.001h2C14 14.486 9.514 10 4 10v2c4.411 0 8 3.589 8 8.001z" /><circle cx="6" cy="18" r="2" /></svg>
   }
}