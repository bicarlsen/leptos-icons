#[cfg(feature = "BiRegularSushi")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSushi")]
/// *This icon requires the feature* `BiRegularSushi` *to be enabled*.
#[component]
pub fn Sushi(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C7.51 2 4 4.2 4 7v10c0 2.8 3.51 5 8 5s8-2.2 8-5V7c0-2.8-3.51-5-8-5zm0 18c-3.54 0-6-1.58-6-3v-6.67A10.52 10.52 0 0 0 12 12a10.52 10.52 0 0 0 6-1.67V17c0 1.42-2.46 3-6 3zm0-10c-3.54 0-6-1.58-6-3s2.46-3 6-3 6 1.58 6 3-2.46 3-6 3z" /><ellipse cx="12" cy="7" rx="3" ry="1.71" /></svg>
   }
}