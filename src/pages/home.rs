use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Welcome..." }</h1>
                        <h2 class="subtitle">{ "...to my personal Rust journey from Cape Town" }</h2>
                    </div>
                </div>

                <div class="tile is-child">
                    //<figure class="image is-3by1">
                    <img src="img/rust_medium.jpg" width="860" height="640" class="center"/>
                        //<img src="img/tobias-reich-1GgWbP74phY-unsplash.jpg" />
                        //https://source.unsplash.com/random/1200x400/?yew
                    //</figure>
                    <div class="content has-text-centered">{"Photo by "} <a href="https://unsplash.com/@electerious?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">{"Tobias Reich"}</a>{" on "}<a href="https://unsplash.com/photos/1GgWbP74phY?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">{"Unsplash"}</a>
                    </div>

                </div>

                <div class="tile is-parent container">
                    { self.view_info_tiles() }
                </div>
            </div>
        }
    }
}
impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "More about me" }</p>

                        <div class="content">
                            { r#"
                            BPM Engineer by day, Rust enthusiast by night. After spending countless hours finding and fixing JavaScript NullPointerExceptions and watching long-running garbage collection threads taking down production Java servers, I personally believe Rust was sent to us developers, system designers, and solution architects as an answer to our prayers.
                            "#}
                            <br />
                        </div>

                        <p class="title">{ "Purpose of this website" }</p>

                        <div class="content">
                            {r#"
                            After reading through the Rust documentation, doing tutorials, and completing various certificates, I have decided to build this website using only Rust to get real-world experience. Hopefully, this will enable me to fulfill my dream of becoming a Rust Engineer by day in the not-too-distant future. Please contact me if I have piqued your interest at 
                            myrustjourney@gmail.com.
                            "#}
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Architecture for this website" }</p>
                        <div class="content">
                        <p>
                        { r#"After investigating a few technologies and frameworks I decided on using:"#}
                        <br />
                        <ul>
                        <li>{r#"Yew, which is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."#}
                            <br />
                            <a href="https://github.com/yewstack/yew">{"https://github.com/yewstack/yew"}</a>
                        </li>
                        <li>{r#"Actix Web, a powerful, pragmatic, and extremely fast web framework for Rust. "#}
                            <br />
                            <a href="https://github.com/actix/actix-web">{"https://github.com/actix/actix-web"}</a>
                        </li>
                        <li>{r#"Gloo-net to enable communication between Yew and Actix via WebSockets. "#}
                            <br />
                            <a href="https://gloo-rs.web.app/blog">{"https://gloo-rs.web.app/blog"}</a>
                        </li>
                        </ul>
                        </p>
                        <p> {r#"Future releases will incorporate storing data in SurrealDB "#} <a href="https://surrealdb.com/why">{"https://surrealdb.com/why"} </a> {r#", a highly-scalable NewSQL database built in Rust and secure communication between all components using Ockam "#} <a href="https://www.ockam.io/">{"https://www.ockam.io/"}</a> {r#" which enables developers to build apps that can Trust data-in-motion."#}
                        </p>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
