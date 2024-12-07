use leptos::{
    component, create_rw_signal, spawn_local, view, For, IntoView, Show, SignalGet, SignalSet,
};
use leptos_meta::provide_meta_context;
use reqwest::get;

use proto::Rustacean;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let rustaceans = create_rw_signal::<Vec<Rustacean>>(Vec::default());
    let error = create_rw_signal::<Option<String>>(None);

    spawn_local(async move {
        match get("https://api.rustacean.sh/api/v1/rustaceans").await {
            Ok(res) => match res.json::<Vec<Rustacean>>().await {
                Ok(items) => {
                    rustaceans.set(items);
                }
                Err(err) => {
                    leptos::logging::error!("Failed to deserialize rustaceans: {:?}", err);
                    error.set(Some("Failed to deserialize response.".into()));
                }
            },
            Err(err) => {
                leptos::logging::error!("Failed to fetch resource: {:?}", err);
                error.set(Some("Failed to fetch resource.".into()));
            }
        }
    });

    view! {
        <h1 class="flex flex-col justify-center items-center bg-gray-800 text-white h-screen w-screen font-bold">
            <code class="text-2xl py-4">"rustacean.sh"</code>
            <p>"The Rustacean Hub, for contributors, projects and news."</p>
            <Show when=move || error.get().is_none() fallback=move || view! {
                <span class="bg-rose-600 text-white p-4 rounded-md">
                    {error.get().unwrap_or_default()}
                </span>
            }>
                <ul class="py-4 space-y-4 w-10/12 md:w-[300px] mx-auto">
                    <For
                        key=|rus: &Rustacean| rus.name.clone()
                        each=move || rustaceans.get()
                        children=move |rus: Rustacean| {
                            view! {
                                <li class="grid grid-cols-[70px,auto] gap-4">
                                    <figure class="h-[70px] w-[70px] rounded-full overflow-hidden">
                                        <img height="70" width="70" src={rus.image} alt={rus.name.clone()} />
                                    </figure>
                                    <article class="flex flex-col items-start justify-center">
                                        <strong>{rus.name}</strong>
                                        <a class="font-medium text-sm underline" target="blank" href={rus.github_url}>
                                            "GitHub"
                                        </a>
                                    </article>
                                </li>
                            }
                        }
                    />
                </ul>
            </Show>
        </h1>
    }
}
