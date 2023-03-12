#[cfg(feature = "SiDapr")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiDapr")]
/// *This icon requires the feature* `SiDapr` *to be enabled*.
#[component]
pub fn Dapr(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M7.6175 7.658h10.5093a.2456.2456 0 0 1 .2461.2461v.656a.2456.2456 0 0 1-.2461.2462H7.6175a.2456.2456 0 0 1-.2461-.2462v-.656a.2456.2456 0 0 1 .2461-.2462zm2.0329-4.476h6.533a.132.132 0 0 1 .1323.1324v4.5199a.132.132 0 0 1-.1323.1323h-6.533a.132.132 0 0 1-.1324-.1323v-4.52a.132.132 0 0 1 .1324-.1323zm5.8312 16.9-1.0503.736-1.0502-.736.1544-4.2073h1.7917zm-9.6058-4.244H4.0963v-.5588c-.17.2134-.3418.369-.5155.4666-.3038.17-.6492.255-1.0362.255-.6258 0-1.1828-.2152-1.671-.6457C.2911 14.8415 0 14.1614 0 13.315c0-.861.2984-1.5482.8952-2.0619.4738-.4088 1.0182-.6131 1.6331-.6131.358 0 .6945.076 1.0091.2279.1809.0868.3672.2297.5589.4286V7.6881h1.7795Zm-1.7253-2.5177c0-.3183-.1121-.5887-.3364-.8112-.2242-.2224-.4955-.3337-.8138-.3337-.3545 0-.6456.1339-.8735.4016-.1845.217-.2767.4648-.2767.7433 0 .2786.0922.5264.2767.7434.2243.2677.5154.4016.8735.4016.322 0 .5941-.1104.8166-.331.2224-.2207.3336-.492.3336-.814Zm8.3997 2.5178h-1.7796v-.559c-.17.2135-.3418.369-.5154.4667-.3038.17-.6492.255-1.0363.255-.6257 0-1.1827-.2152-1.671-.6457-.5824-.5136-.8735-1.1937-.8735-2.0402 0-.861.2984-1.5482.8952-2.0619.4738-.4088 1.0182-.6131 1.633-.6131.3581 0 .6945.076 1.0092.2279.1809.0868.3671.2297.5588.4286v-.4938h1.7796zm-1.7253-2.5178c0-.3183-.1121-.5887-.3364-.8112-.2242-.2224-.4955-.3337-.8138-.3337-.3545 0-.6457.1339-.8735.4016-.1845.217-.2767.4648-.2767.7433 0 .2786.0922.5264.2767.7434.2242.2677.5154.4016.8735.4016.3219 0 .594-.1104.8165-.331.2225-.2207.3337-.492.3337-.814zm8.6004.0054c0 .861-.2984 1.5483-.8952 2.062-.4738.4087-1.0182.6131-1.633.6131-.3582 0-.6945-.076-1.0092-.2279-.1809-.0868-.3672-.2297-.5589-.4286v3.0114h-1.7795v-7.553h1.7795v.5588c.1592-.2098.331-.3654.5155-.4666.3038-.17.6492-.255 1.0362-.255.6258 0 1.1828.2152 1.6711.6456.5823.5137.8735 1.1938.8735 2.0402zm-1.85-.0054c0-.2857-.0905-.5335-.2714-.7433-.2278-.2677-.5208-.4016-.8789-.4016-.322 0-.594.1104-.8165.331-.2225.2207-.3337.492-.3337.814 0 .3183.1121.5887.3364.8111.2242.2225.4955.3338.8138.3338.358 0 .6493-.1339.8735-.4016.1845-.217.2767-.4648.2767-.7434zM24 12.4467c-.2496-.1193-.5028-.179-.7596-.179-.586 0-.9657.2387-1.1393.7162-.0651.1737-.0977.407-.0977.7v2.1542h-1.7796v-5.0355h1.7796v.8248c.188-.293.3906-.5046.6077-.6348.293-.1737.6402-.2605 1.0417-.2605a4.51 4.51 0 0 1 .3472.0163z" /></svg>
   }
}