use leptos::*;
use leptos::html::Select;
use leptos_markdown::*;

use std::collections::HashMap;

use serde::Deserialize;


static INITIAL_CV: &'static str = r#"
---
contact:
  email: "antonin.peronnet@foobar.tech"
  linkedin: "todo"
---

# Antonin Peronnet

## Projects
<Experience date="2022 - 2023">

Contributed to open-source projects (my github: <https://github.com/rambip>)
- `leptos`, a frontend & backend web framework
- `pulldown_cmark`, a markdown parser
- worked on a website generator for knowledge bases

</Experience>

<Experience date="2023">

ARTEFACT project, Telecom Paris
- prototyping of mini-autonomous vehicles
- data transfer, raspberrypi, I2C control

</Experience>

<Experience date="2021">

Design and prototyping of an ergonomic keyboard 
- with a classmate, from scratch
- fully usable

</Experience>

## Education and training

<Experience date="2023 - present">

Telecom Paris, France
- Maths, Advanced algorithms and compilation
- Economy
- Telecomunications

</Experience>

<Experience date="2021 - 2023">

Dense quality training, selective exams, to enter top engineering school.
- Maths, Physics and Computer Science fundamentals. 

</Experience>

## Professional experience

<Experience date="Summer 2022">

Math lessons to a 13 years old public as an independent
- demotivated students, had to find ways to make the course interactive

</Experience>

<Experience date="Summer 2021, Summer 2022, Summer 2023">

Farm job in a corn field
- conscientious manual work
- managing a team of 10 young peoples

</Experience>

## Skills


Digital skills
- in-depth knowledge of Rust, Mastery of OCaml, basics of C
- basics of web technology: CSS, JS, Webassembly
- Linux, code infrastructure with NixOS

Communication skills
- love writing good documentation and improving with feedback
- can write quality *Markdown* and $\LaTeX$

Languages
- French: native language
- English: fluent
- German: limited knowledge
"#;

static FONTS: [&str; 7] = [
    "verdana",
    "times",
    "sans-serif",
    "monaco",
    "serif",
    "monospace",
    "arial",
];


#[derive(Deserialize)]
struct MetaData {
    contact: HashMap<String, String>
}


fn experience(props: MdComponentProps) -> impl IntoView {
    let date = props.attributes
        .into_iter()
        .find(|x| x.0=="date")
        .map(|x| x.1);

    view!{
    <div class="date-container">
        <span class="date">{date}</span>
    </div>
    <div style="padding-left: 10px">{props.children}</div>
    }
}


fn contact(m: Result<MetaData, serde_yaml::Error>) -> impl IntoView {
    let m = m.unwrap();
    view!{
        <h3>Contact me</h3>
        {m.contact["email"].clone()}
    }
}

#[component]
fn Cv(content: ReadSignal<String>, font_index: ReadSignal<usize>) -> impl IntoView {
    let (frontmatter, set_frontmatter) = create_signal(String::new());

    let custom_components = ComponentMap::new()
        .add("Experience", experience);

    view!{
        <div class="grid" style:font-family=move || FONTS[font_index()]>
            <Markdown src=content components=custom_components frontmatter=set_frontmatter/>
            <div class="contact">
            {move || contact(serde_yaml::from_str(&frontmatter()))}
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (content, set_content) = create_signal(INITIAL_CV.to_string());
    let (font_index, set_font_index) = create_signal(0usize);


    let font_options = FONTS
        .into_iter()
        .map(|x| view!{ <option value=x.clone()>{x}</option> })
        .collect_view();

    let select_ref = create_node_ref::<Select>();

    view!{
        <div style={"display: flex; align-items: top;"}>
            <div style="width: 30%">
                <label for="cars">Choose a font:</label>
                <select name="cars" id="cars"
                    ref=select_ref
                    on:change=move |_| set_font_index(
                        select_ref.get().unwrap().selected_index() as usize
                        )
                >
                    {font_options}
                </select>
                <textarea type="text"
                    on:input = move |ev| set_content(event_target_value(&ev))
                    prop:value = content
                    placeholder = "enter cv here"
                    rows={30}
                    style="margin: 10px; width: 80%"
                />
            </div>
            <Cv content=content font_index=font_index/>
        </div>
    }
}


fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
