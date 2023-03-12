#[cfg(feature = "CgPlayListRemove")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayListRemove")]
/// *This icon requires the feature* `CgPlayListRemove` *to be enabled*.
#[component]
pub fn PlayListRemove(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M15.9644 4.63379H3.96442V6.63379H15.9644V4.63379Z" fill="currentColor" /><path d="M15.9644 8.63379H3.96442V10.6338H15.9644V8.63379Z" fill="currentColor" /><path d="M3.96442 12.6338H11.9644V14.6338H3.96442V12.6338Z" fill="currentColor" /><path d="M12.9645 13.7093L14.3787 12.295L16.5 14.4163L18.6213 12.2951L20.0355 13.7093L17.9142 15.8305L20.0356 17.9519L18.6214 19.3661L16.5 17.2447L14.3786 19.3661L12.9644 17.9519L15.0858 15.8305L12.9645 13.7093Z" fill="currentColor" /></svg>
   }
}