#[cfg(feature = "TbChartDonut2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbChartDonut2")]
/// *This icon requires the feature* `TbChartDonut2` *to be enabled*.
#[component]
pub fn ChartDonut2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-chart-donut-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 3v5m4 4h5" /><path d="M12 12m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /></svg>
   }
}