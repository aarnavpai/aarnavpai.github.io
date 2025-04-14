use sycamore::prelude::*;

mod theme;

#[component]
fn App() -> View {
    const STYLES: [&'static str; 9] = [
        "background-color: #000000",
        "background-color: #F7A41D; color: black",
        "background-color: #00ADD8",
        "background-color: #A8B9CC; color: black",
        "background-color: #FFAFF3",
        "background-color: #A90533",
        "background-color: #3776AB",
        "background-color: #3178C6",
        "background-color: #C00000"
    ];
    view! {
        theme::ThemeSelector() {}
        main {
            h1 { "Aarnav Pai's homepage" }

            p { "I'm a web developer studying at " a(href="https://iiit.ac.in") {"IIIT"} "! I am one of the creators and administrators of the " a(href="https://mess.iiit.ac.in") {"IIIT Mess Portal"} ", and a member of the " a(href="https://osdg.iiit.ac.in"){ "OSDG" } "." }

            p { "I was born in Mangalore, Karnataka, but have lived in Bangalore. I'm currently in Hyderabad for my studies." }
            p { "I've studied from kindergarten to 12th grade in Bangalore, and am pursuing undergraduate education in Hyderabad." }

            p (style="text-align: center; color: var(--text-dark)") { "check out my " a(href="/cv.pdf") {"CV"} "!" }

            h2 { "Technical Skills"}

            h3 { "Programming Languages"}

            ul (class="skills-list") {
                li (style=STYLES[0]) { "Rust" }
                li (style=STYLES[1]) { "Zig" }
                li (style=STYLES[2]) { "Go" }
                li (style=STYLES[3]){ "C" }
                li (style=STYLES[4]){ "Gleam" }
                li (style=STYLES[5]){ "Erlang" }
                li (style=STYLES[6]){ "Python" }
                li (style=STYLES[7]){ "JS/TS" }
                li (style=STYLES[8]){ "Java, etc." }
            }
        }
    }
}

fn main() {
    sycamore::render(App);
}
