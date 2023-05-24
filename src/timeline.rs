use std::{error::Error, str::FromStr};

use crate::app_state::*;
use chrono::{Duration, NaiveDate};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub data: Option<TimelineData>,
    pub title: String,
    pub canvas_id: String,
    pub set_date_range: Callback<(Date, Date)>,
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    let start_date_handle = use_state(String::new);
    let end_date_handle = use_state(String::new);
    let view_type_handle = use_state_eq(|| ViewType::Text);

    let on_start_date_change = {
        let start_date = start_date_handle.clone();
        let end_date = end_date_handle.clone();
        let set_date_range = props.set_date_range.clone();
        move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                start_date.set(input.value());
                if let Some(range) = validate_dates(&*start_date, &*end_date) {
                    set_date_range.emit(range)
                }
            }
        }
    };

    let on_end_date_change = {
        let start_date = start_date_handle.clone();
        let end_date = end_date_handle.clone();
        let set_date_range = props.set_date_range.clone();
        move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                end_date.set(input.value());
                if let Some(range) = validate_dates(&*start_date, &*end_date) {
                    set_date_range.emit(range)
                }
            }
        }
    };
    
    let on_view_type_change = {
        let view_type_handle = view_type_handle.clone();
        move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                if let Ok(view_type) = input.value().parse::<ViewType>() {
                    view_type_handle.set(view_type);
                } else {
                    gloo_console::log!(format!("could not parse view type"))
                }
            }
        }
    };
    
    let start_date = (*start_date_handle).clone();
    let end_date = (*end_date_handle).clone();
    let view_type = (*view_type_handle).clone();
    html! {
    <section>
        <h3>{props.title.clone()}</h3>
        <p>{"View Type: "}</p>
        <input
            type="radio"
            id="text"
            name="view_type"
            value="text"
            onchange={on_view_type_change.clone()}
        />
        <label for="text">{"Text"}</label>
        <input
            type="radio"
            id="histogram"
            name="view_type"
            value="histogram"
            onchange={on_view_type_change.clone()}
        />
        <label for="histogram">{"Histogram"}</label>
        {if let Some(data) = props.data.clone() {
            match view_type {
                ViewType::Histogram => html!{
                    <HistogramView
                        canvas_id={"my_canvas"}
                        data={props.data.clone().unwrap_or_default()}
                    />
                },
                ViewType::Text => html!{
                    <DateSummaryView
                        data={data}
                    />
                },
            }
        } else {
            html!{}
        }}
        <p>{"Start Date: "}</p>
        <input onchange={on_start_date_change}
            type="date"
            value={start_date}
        />
        <p>{"End Date: "}</p>
        <input onchange={on_end_date_change}
            type="date"
            value={end_date}
        />
    </section>
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ViewType {
    Text,
    Histogram,
}

impl FromStr for ViewType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "histogram" => Ok(Self::Histogram),
            _ => Err(())
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct HistogramViewProps {
    pub canvas_id: String,
    pub data: TimelineData,
}

#[function_component(HistogramView)]
pub fn histogram(props: &HistogramViewProps) -> Html {
    use_effect({
        let data = props.data.clone();
        let canvas_id = props.canvas_id.clone();
        move || {
            match draw_timeline(&canvas_id, data) {
                Err(e) => gloo_console::log!(format!("{e:?}")),
                _ => {}
            }
            || {}
        }
    });
    html! {
        <canvas
            id={props.canvas_id.clone()}
            style={"width: 80%; height: auto; max-width: 500px;"}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct DateSummaryViewProps {
    data: TimelineData,
}

#[function_component(DateSummaryView)]
pub fn histogram(props: &DateSummaryViewProps) -> Html {
    html! {
    <>
    <p><b>{"Date Summaries"}</b></p>
    <hr />
    {for props.data.iter().map(|summary: &DateSummary| html!{
        <>
        <p>{"Date: "}{summary.date}</p>
        <p>{"Income: "}{summary.income}</p>
        <p>{"Expenses: "}{summary.expenses}</p>
        <hr />
        </>
    })}
    </>
    }
}

fn validate_dates(start_date: &str, end_date: &str) -> Option<(Date, Date)> {
    match (start_date.parse::<Date>(), end_date.parse::<Date>()) {
        (Err(e1), Err(e2)) => {
            gloo_console::log!(format!("start date parse error: {e1:?}"));
            gloo_console::log!(format!("end date parse error: {e2:?}"));
            None
        }
        (Err(e), Ok(_)) => {
            gloo_console::log!(format!("start date parse error: {e:?}"));
            None
        }
        (Ok(_), Err(e)) => {
            gloo_console::log!(format!("end date parse error: {e:?}"));
            None
        }
        (Ok(start_date), Ok(end_date)) => Some((start_date, end_date)),
    }
}

// TODO: this method should be a view; computations performed by State object
fn draw_timeline(canvas_id: &str, data: TimelineData) -> Result<(), Box<dyn Error>> {
    let start_date = match data.start_date() {
        Some(d) => d,
        _ => return Ok(()),
    };
    let end_date = start_date + Duration::days(data.len() as i64);
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let max = if let Some(value) = data
        .iter()
        .map(|v| v.income.max(v.expenses).max(v.balance))
        .max()
    {
        math::round::floor(value as f64 * 1.1, 1).max(100.0) as u32
    } else {
        return Ok(());
    };

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .margin(5)
        //.caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((start_date..end_date).into_segmented(), 0u32..max)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_label_formatter(&|v: &SegmentValue<NaiveDate>| match v {
            SegmentValue::Exact(d) | SegmentValue::CenterOf(d) => d.format("%d").to_string(),
            _ => "<em>SeinfeldHEUHH.mp3</em>".to_owned(),
        })
        .y_labels(10)
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Dollars")
        .x_desc("Date")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(
                data.iter()
                    .enumerate()
                    .map(|(n, s)| (start_date + Duration::days(n as i64), s.income)),
            ),
    )?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(
                data.iter()
                    .enumerate()
                    .map(|(n, s)| (start_date + Duration::days(n as i64), s.expenses)),
            ),
    )?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLACK.mix(0.5).filled())
            .data(
                data.iter()
                    .enumerate()
                    .map(|(n, s)| (start_date + Duration::days(n as i64), s.balance)),
            ),
    )?;

    root.present()?;
    Ok(())
}