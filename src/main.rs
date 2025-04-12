use sycamore::prelude::*;

#[component]
fn App() -> View {
    view! {
        h1 { "Aarnav Pai's homepage" }

        p { "I'm a web developer studying at " a(href="https://iiit.ac.in") {"IIIT"} "! I am one of the creators and administrators of the " a(href="https://mess.iiit.ac.in") {"IIIT Mess Portal"} ", and a member of the " a(href="https://osdg.iiit.ac.in"){ "OSDG" } "." }

        p { "I'm born in Mangalore, Karnataka, but have lived in Bangalore. I'm currently in Hyderabad for my studies." }

        p { "Click on any of the following boxes to be taken to their respective pages:"}
    }
}

fn main() {
    sycamore::render(App);
}
