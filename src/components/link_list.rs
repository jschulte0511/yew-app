use crate::models::link::Link;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkListProps {
    pub links: Vec<Link>,
}

#[function_component(LinkList)]
pub fn link_list(LinkListProps { links }: &LinkListProps) -> Html {
    // To display them, we need to convert the Vec into Html. We can do that by creating an iterator, mapping
    // it to html! and collecting it as Html

    //info!("Found links: {:?}", links);
    //let links = generate_links();

    links
        .iter()
        .map(|link| {
            html! {
                <>
                    <p key={link.id}>
                        <b>{"Title: "} </b> {format!("{}", link.title)} <br/>
                        {"Url: "}  <a href={format!("{}",link.url)}>{format!("{}",link.url)}</a>
                    </p>
                </>
            }
        })
        .collect()
}
