#[cfg(feature = "BiRegularGlobeAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularGlobeAlt")]
/// *This icon requires the feature* `BiRegularGlobeAlt` *to be enabled*.
#[component]
pub fn GlobeAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 18.791V20H9v2h6v-2h-2v-1.845a9.934 9.934 0 0 0 3.071-2.084c3.898-3.898 3.898-10.243 0-14.143l-1.414 1.414c3.119 3.12 3.119 8.195 0 11.314-3.119 3.118-8.195 3.12-11.314 0L1.929 16.07A9.971 9.971 0 0 0 9 18.994a9.98 9.98 0 0 0 2-.203z" /><path d="M3 9c0 3.309 2.691 6 6 6s6-2.691 6-6-2.691-6-6-6-6 2.691-6 6zm10 0c0 2.206-1.794 4-4 4s-4-1.794-4-4 1.794-4 4-4 4 1.794 4 4z" /></svg>
   }
}