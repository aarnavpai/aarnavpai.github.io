use std::collections::HashMap;

use js_sys::wasm_bindgen::JsCast;
use sycamore::prelude::*;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

mod theme;

#[component]
fn Gallery() -> View {
    let active = create_signal::<Option<u32>>(None);
    let images = (1..=5)
        .map(|x| (String::from("/img/") + &x.to_string() + ".jpeg", x))
        .map(|(file, x)| view! {img(src=file, alt="gallery", on:click=move |_| active.set(Some(x))) {}})
        .collect::<Vec<_>>();

    view! {
        (if let Some(x) = active.get() {
            let src = String::from("/img/") + &x.to_string() + ".jpeg";
            view! {
                div(class="img-modal", on:click=move|_| active.set(None)) {
                    div {
                        img(src=src, alt="modal") {}
                    }
                }
            }
        } else { view!{} })
        div (class="gallery") {
            (images)
        }
    }
}

// assuming only personal pronouns
const PRONOUNS: &'static str = include_str!("../data/pronouns.txt");
const PREPOSITIONS: &'static str = include_str!("../data/prepositions.txt");

#[component]
fn Words() -> View {
    let submit = move |e: SubmitEvent| {
        e.prevent_default();
        let form = e.target().unwrap().unchecked_into::<HtmlFormElement>();
        let fd = FormData::new_with_form(&form).unwrap();
        let mut pronouns: HashMap<&str, i32> = PRONOUNS
            .split("\n")
            .map(|x| (x, 0))
            .collect::<HashMap<_, _>>();
        let mut prepositions: HashMap<&str, i32> = PREPOSITIONS
            .split("\n")
            .map(|x| (x, 0))
            .collect::<HashMap<_, _>>();
        let mut articles: HashMap<&str, i32> = HashMap::new();
        articles.insert("a", 0);
        articles.insert("an", 0);

        let words = fd.get("words").as_string().unwrap();
        let (letters, spaces, newlines, special) = {
            let mut letters = 0;
            let mut spaces = 0;
            // assuming newline means \n
            let mut newlines = 0;
            let mut special = 0;

            for char in words.bytes() {
                match char {
                    b'\n' => newlines += 1,
                    b' ' => spaces += 1,
                    65..=90 | 97..=122 => letters += 1,
                    48..=57 => {}
                    _ => special += 1,
                }
            }

            (letters, spaces, newlines, special)
        };
        let num_words = words
            .split_whitespace()
            .filter(|x| !x.trim().is_empty())
            .count();
        words
            .to_lowercase()
            .split_whitespace()
            .map(|word| {
                let mut start = 0;
                let mut end = word.len() - 1;
                for (i, ch) in word.bytes().enumerate() {
                    if (65..=90).contains(&ch) || (97..=122).contains(&ch) {
                        start = i;
                        break;
                    }
                }
                for (i, ch) in word.bytes().enumerate().rev() {
                    if (65..=90).contains(&ch) || (97..=122).contains(&ch) {
                        end = i;
                        break;
                    }
                }
                let word = &word[start..=end];
                if let Some(num) = pronouns.get_mut(word) {
                    *num += 1;
                } else if let Some(num) = prepositions.get_mut(word) {
                    *num += 1;
                }
                if let Some(num) = articles.get_mut(word) {
                    *num += 1;
                }
            })
            .count();

        let document = document();
        let result = document.get_element_by_id("rust-result").unwrap();
        let ul = document.create_element("ul").unwrap();
        result.set_inner_html("");
        ul.set_class_name("calc");
        {
            let li = document.create_element("li").unwrap();
            li.set_text_content(Some(&format!("Letters: {letters}")));
            ul.append_child(&li.into()).unwrap();
        }
        {
            let li = document.create_element("li").unwrap();
            li.set_text_content(Some(&format!("Words: {num_words}")));
            ul.append_child(&li.into()).unwrap();
        }
        {
            let li = document.create_element("li").unwrap();
            li.set_text_content(Some(&format!("Spaces: {spaces}")));
            ul.append_child(&li.into()).unwrap();
        }
        {
            let li = document.create_element("li").unwrap();
            li.set_text_content(Some(&format!("Newlines: {newlines}")));
            ul.append_child(&li.into()).unwrap();
        }
        {
            let li = document.create_element("li").unwrap();
            li.set_text_content(Some(&format!("Special Characters: {special}")));
            ul.append_child(&li.into()).unwrap();
        }
        result.append_child(&ul.into()).unwrap();

        for (name, nums) in [
            ("Pronouns", pronouns),
            ("Prepositions", prepositions),
            ("Indefinite Articles", articles),
        ] {
            let div = document.create_element("div").unwrap();
            let count = document.create_element("span").unwrap();
            let mut sum = 0;
            let ul = document.create_element("ul").unwrap();
            for (k, v) in nums.into_iter() {
                if v == 0 {
                    continue;
                }
                sum += v;
                let li = document.create_element("li").unwrap();
                li.set_text_content(Some(&format!("{k}: {v}")));
                ul.append_child(&li.into()).unwrap();
            }
            count.set_text_content(Some(&format!("{name}: {}", sum)));
            div.append_child(&count.into()).unwrap();
            div.append_child(&ul.into()).unwrap();
            result.append_child(&div.into()).unwrap();
        }
    };

    view! {
        form(id="words", on:submit=submit) {
            textarea(name="words", placeholder="enter some words here", id="10k", rows="10") {}
            div(style="display: flex; justify-content: flex-end; margin-top: 0.5rem") {
                button(r#type="submit") {"Check!"}
            }
        }
    }
}

#[component]
fn App() -> View {
    const PROG_STYLES: [&'static str; 9] = [
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
            img (class="me", src="/img/me.jpeg", alt="Aarnav Pai") {}

            p { "I'm a web developer studying at " a(href="https://iiit.ac.in") {"IIIT"} "! I am one of the creators and administrators of the " a(href="https://mess.iiit.ac.in") {"IIIT Mess Portal"} ", and a member of the " a(href="https://osdg.iiit.ac.in"){ "OSDG" } "." }

            p { "I was born in Mangalore, Karnataka, but have lived in Bangalore. I'm currently in Hyderabad for my studies." }
            p { "I've studied from kindergarten to 12th grade in Bangalore, and am pursuing undergraduate education in Hyderabad." }
            p { "I studied in VIBGYOR High School."}

            p (style="text-align: center; color: var(--text-dark)") { "check out my " a(href="/cv.pdf") {"CV"} "!" }

            h2 { "Technical Skills" }

            div (style="padding: 0.5rem") {
                h3 { "Programming Languages" }

                ul (class="skills-list") {
                    li (style=PROG_STYLES[0]) { a (href="https://rust-lang.org") { "Rust" } }
                    li (style=PROG_STYLES[1]) { a (href="https://ziglang.org") { "Zig" } }
                    li (style=PROG_STYLES[2]) { a (href="https://go.dev") { "Go" } }
                    li (style=PROG_STYLES[3]) { a (href="https://en.wikipedia.org/wiki/C_(programming_language)") { "C" } }
                    li (style=PROG_STYLES[4]) { a (href="https://gleam.run") { "Gleam" } }
                    li (style=PROG_STYLES[5]) { a (href="https://erlang.org") { "Erlang" } }
                    li (style=PROG_STYLES[6]) { a (href="https://python.org") { "Python" } }
                    li (style=PROG_STYLES[7]) { a (href="https://typescriptlang.org") { "JS/TS" } }
                    li (style=PROG_STYLES[8]) { a (href="https://haskell.org") { "Java, etc." } }
                }

                h3 { "Projects" }

                ul (class="skills-list") {
                    li (style="background-color: #692812") { a (href="https://gitlab.iiit.ac.in/messcom") { "Mess Portal" } }
                }
            }

            h2 { "Gallery" }

            Gallery() {}

            h2 { "A textbox" }

            Words() {}

            div (id="result") {}
            div (id="rust-result", style="display: none; margin-top: 1rem") {}
        }
    }
}

fn main() {
    sycamore::render(App);
}
