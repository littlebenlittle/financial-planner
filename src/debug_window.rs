use crate::app_state::*;
use itertools::Itertools;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DebugWindowProps {
    pub log: Vec<Entry>,
}

#[function_component(DebugWindow)]
pub fn debug_window(props: &DebugWindowProps) -> Html {
    let show = use_state(|| false);
    let onclick = {
        let show = show.clone();
        move |_| {
            show.set(!(*show));
        }
    };
    html! {
    <section
        class={classes!("w3-container", "w3-card", "w3-padding-16")}
    >
        <p><b>{"Debug Info:"}</b></p>
        <button {onclick}>{
            if *show {
                "Hide debug info"
            } else {
                "Show debug info"
            }
        }</button>
        {if *show {
            html!{<LogEntries log={props.log.clone()} />}
        } else {
            html!{}
        }}
    </section>
    }
}

#[derive(Properties, PartialEq)]
struct LogEntriesProps {
    pub log: Vec<Entry>,
}

#[function_component(LogEntries)]
fn log_entries(props: &LogEntriesProps) -> Html {
    // let text = props
    //     .log
    //     .iter()
    //     .map(|e| serde_yaml::to_string(e).unwrap())
    //     .join("\n");
    let text = serde_yaml::to_string(&props.log).unwrap();
    html! {
        <div id={"log-entries"}>
        <p>{"Log Entries:"}</p>
        <div class={classes!("w3-dark-gray")}>
        <pre class={classes!("w3-dark-gray", "w3-text-white")}>
            {text}
        </pre>
        </div>
        </div>
    }
}
