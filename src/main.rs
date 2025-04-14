use sycamore::prelude::*;

mod theme;

#[component]
fn App() -> View {
    const STYLES: [&'static str; 9] = [
        "background-color: #000000",
        "background-color: #F7A41D; color: black",
        "background-color: #00ADD8",
        "background-color: #A8B9CC; color: black",
        "background-color: #FFAFF3; color: black",
        "background-color: #A90533",
        "background-color: #3776AB",
        "background-color: #3178C6",
        "background-color: #C00000",
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
                li (style=STYLES[0]) { a (href="https://rust-lang.org") { "Rust" } }
                li (style=STYLES[1]) { a (href="https://ziglang.org") { "Zig" } }
                li (style=STYLES[2]) { a (href="https://go.dev") { "Go" } }
                li (style=STYLES[3]) { a (href="https://en.wikipedia.org/wiki/C_(programming_language)") { "C" } }
                li (style=STYLES[4]) { a (href="https://gleam.run") { "Gleam" } }
                li (style=STYLES[5]) { a (href="https://erlang.org") { "Erlang" } }
                li (style=STYLES[6]) { a (href="https://python.org") { "Python" } }
                li (style=STYLES[7]) { a (href="https://typescriptlang.org") { "JS/TS" } }
                li (style=STYLES[8]) { a (href="https://haskell.org") { "Java, etc." } }
            }
        }
    }
}

fn main() {
    sycamore::render(App);
}
