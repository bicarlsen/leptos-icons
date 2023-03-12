#[cfg(feature = "BiLogosAngular")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosAngular")]
/// *This icon requires the feature* `BiLogosAngular` *to be enabled*.
#[component]
pub fn Angular(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10.483 12.482h3.034L12 8.831z" /><path d="M12 3.074 3.689 6.038l1.268 10.987 7.043 3.9 7.043-3.9 1.268-10.987L12 3.074zm5.187 13.621H15.25l-1.045-2.606h-4.41L8.75 16.695H6.813L12 5.047l5.187 11.648z" /></svg>
   }
}