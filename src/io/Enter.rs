#[cfg(feature = "IoEnter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEnter")]
/// *This icon requires the feature* `IoEnter` *to be enabled*.
#[component]
pub fn Enter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M160,136V240H313.37l-52.68-52.69a16,16,0,0,1,22.62-22.62l80,80a16,16,0,0,1,0,22.62l-80,80a16,16,0,0,1-22.62-22.62L313.37,272H160V376a56.06,56.06,0,0,0,56,56H424a56.06,56.06,0,0,0,56-56V136a56.06,56.06,0,0,0-56-56H216A56.06,56.06,0,0,0,160,136Z" /><path d="M48,240a16,16,0,0,0,0,32H160V240Z" /></svg>
   }
}