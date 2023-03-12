#[cfg(feature = "IoTime")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTime")]
/// *This icon requires the feature* `IoTime` *to be enabled*.
#[component]
pub fn Time(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48Zm96,240H256a16,16,0,0,1-16-16V128a16,16,0,0,1,32,0V256h80a16,16,0,0,1,0,32Z" /></svg>
   }
}