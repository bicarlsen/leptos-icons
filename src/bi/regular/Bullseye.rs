#[cfg(feature = "BiRegularBullseye")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBullseye")]
/// *This icon requires the feature* `BiRegularBullseye` *to be enabled*.
#[component]
pub fn Bullseye(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 6c-3.309 0-6 2.691-6 6s2.691 6 6 6 6-2.691 6-6-2.691-6-6-6zm0 10c-2.206 0-4-1.794-4-4s1.794-4 4-4 4 1.794 4 4-1.794 4-4 4z" /><path d="M12 2C6.579 2 2 6.579 2 12s4.579 10 10 10 10-4.579 10-10S17.421 2 12 2zm0 18c-4.337 0-8-3.663-8-8s3.663-8 8-8 8 3.663 8 8-3.663 8-8 8z" /><path d="M12 10c-1.081 0-2 .919-2 2s.919 2 2 2 2-.919 2-2-.919-2-2-2z" /></svg>
   }
}