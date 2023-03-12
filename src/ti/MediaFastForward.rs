#[cfg(feature = "TiMediaFastForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMediaFastForward")]
/// *This icon requires the feature* `TiMediaFastForward` *to be enabled*.
#[component]
pub fn MediaFastForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M15.053 6.912c-.324-.314-.765-.512-1.253-.512-.994 0-1.8.807-1.8 1.801v9c0 .994.806 1.799 1.8 1.799.488 0 .93-.195 1.253-.512 2.381-2.314 5.947-5.787 5.947-5.787s-3.566-3.474-5.947-5.789zM6.053 6.912c-.324-.314-.765-.512-1.253-.512-.994 0-1.8.807-1.8 1.801v9c0 .994.806 1.799 1.8 1.799.488 0 .93-.195 1.253-.512 2.381-2.314 5.947-5.787 5.947-5.787s-3.566-3.474-5.947-5.789z" /></svg>
   }
}