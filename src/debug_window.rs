use crate::app_state::*;
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
        <>
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
        </>
    }
}


#[derive(Properties, PartialEq)]
struct LogEntriesProps {
    pub log: Vec<Entry>,
}

#[function_component(LogEntries)]
fn log_entries(props: &LogEntriesProps) -> Html {
    html!{
    <>
    <p>{"Log Entries:"}</p>
    {for props.log.iter().map(|e: &Entry| match e {
        Entry::Create(t) => html!{
            <>
            <hr />
            <p>{"Create"}</p>
            <p>{"Date: "}{t.date}</p>
            <p>{"Kind: "}{t.kind}</p>
            <p>{"Value: "}{t.value}</p>
            </>
        },
        Entry::Delete(id) => html! {
            <>
            <hr />
            <p>{"Delete"}</p>
            <p>{"Id: "}{id}</p>
            </>
        },
        Entry::SetDate(date_range) => html!{
            <>
            <hr />
            <p>{"Set Date Range"}</p>
            <p>{"Start: "}{date_range.start}</p>
            <p>{"End: "}{date_range.end}</p>
            </>
        }
    })}
    </>
    }
}
