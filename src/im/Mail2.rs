#[cfg(feature = "ImMail2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImMail2")]
/// *This icon requires the feature* `ImMail2` *to be enabled*.
#[component]
pub fn Mail2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13.333 0h-10.666c-1.467 0-2.667 1.2-2.667 2.667v10.666c0 1.468 1.2 2.667 2.667 2.667h10.666c1.467 0 2.667-1.199 2.667-2.667v-10.666c0-1.467-1.2-2.667-2.667-2.667zM13.333 2c0.125 0 0.243 0.036 0.344 0.099l-5.678 4.694-5.677-4.694c0.101-0.063 0.219-0.099 0.344-0.099h10.666zM2.667 14c-0.030 0-0.060-0.002-0.089-0.006l3.525-4.89-0.457-0.457-3.646 3.646v-9.549l6 7.256 6-7.256v9.549l-3.646-3.646-0.457 0.457 3.525 4.89c-0.029 0.004-0.059 0.006-0.088 0.006h-10.666z" /></svg>
   }
}