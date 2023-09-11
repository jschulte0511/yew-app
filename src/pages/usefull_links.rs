use crate::components::link_list::LinkList;
use crate::models::link::Link;
use gloo_net::http::Request;
use log::info;
use yew::prelude::*;

pub struct UsefullLinks;

impl Component for UsefullLinks {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <LinkPage/>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkPageProps {}

#[function_component(LinkPage)]
pub fn link_page(LinkPageProps {}: &LinkPageProps) -> Html {
    let links = use_state(|| vec![]);

    {
        let links = links.clone();
        use_effect_with_deps(
            move |_| {
                let links = links.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_links: Vec<Link> = Request::get("http://localhost:8000/links")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    info!("Found links in call: {:?}", fetched_links);
                    links.set(fetched_links);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <p class="title">{ "Useful Links" }</p>

                    <div class="content">
                        {r#"
                            Links I have found useful on my journey to become a successful Rust Open Source Engineer
                            "#}
                        <p/>
                        <LinkList links={(*links).clone()} />
                    </div>
                </div>
            </div>
        </>
    }
}
