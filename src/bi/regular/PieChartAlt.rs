#[cfg(feature = "BiRegularPieChartAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPieChartAlt")]
/// *This icon requires the feature* `BiRegularPieChartAlt` *to be enabled*.
#[component]
pub fn PieChartAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2a9.936 9.936 0 0 0-7.071 2.929C3.04 6.818 2 9.33 2 12s1.04 5.182 2.929 7.071C6.818 20.96 9.33 22 12 22s5.182-1.04 7.071-2.929C20.96 17.182 22 14.67 22 12s-1.04-5.182-2.929-7.071A9.936 9.936 0 0 0 12 2zm5.657 15.657C16.146 19.168 14.137 20 12 20s-4.146-.832-5.657-2.343C4.832 16.146 4 14.137 4 12s.832-4.146 2.343-5.657A7.927 7.927 0 0 1 11 4.069V12a1 1 0 0 0 1 1h7.931a7.927 7.927 0 0 1-2.274 4.657zM13 11V4.062a7.945 7.945 0 0 1 4.657 2.281A7.934 7.934 0 0 1 19.938 11H13z" /></svg>
   }
}