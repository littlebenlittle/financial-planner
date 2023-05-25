use crate::app_state::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DebugWindowProps {
    pub log: Vec<Entry>,
}

#[function_component(DebugWindow)]
pub fn debug_window(props: &DebugWindowProps) -> Html {
    html! {
        <>
        <p><b>{"Debug Info:"}</b></p>
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
