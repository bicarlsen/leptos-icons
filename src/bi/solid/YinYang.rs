#[cfg(feature = "BiSolidYinYang")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidYinYang")]
/// *This icon requires the feature* `BiSolidYinYang` *to be enabled*.
#[component]
pub fn YinYang(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19.062 4.938A9.942 9.942 0 0 0 12.016 2h-.026a9.94 9.94 0 0 0-7.071 2.938c-3.898 3.898-3.898 10.243 0 14.143 1.895 1.895 4.405 2.938 7.071 2.938s5.177-1.043 7.071-2.938c3.9-3.899 3.9-10.243.001-14.143zM13.5 15a1.5 1.5 0 1 1-.001 3.001A1.5 1.5 0 0 1 13.5 15zM6.333 6.353A7.953 7.953 0 0 1 11.99 4l.026.001c1.652.008 3.242 1.066 3.55 2.371.366 1.552-1.098 3.278-4.018 4.737-5.113 2.555-5.312 5.333-4.975 6.762l.008.021c-.082-.075-.169-.146-.249-.226-3.118-3.119-3.118-8.194.001-11.313z" /><circle cx="10.5" cy="7.5" r="1.5" /></svg>
   }
}