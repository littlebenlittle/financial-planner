use std::error::Error;

use crate::app_state::*;
use chrono::{Duration, NaiveDate};
use itertools::Itertools;
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
    pub set_date_range: Callback<(Date, Date)>
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    let canvas_id = props.canvas_id.clone();
    let start_date_handle = 

    let on_start_date_change = {
        let start_date_handle = start_date_handle.clone();
        move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                start_date_handle.set(input.value());
            }
        }
    };

    let end_date_handle = use_state(String::new);
    let on_end_date_change = {
        let end_date_handle = end_date_handle.clone();
        move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                end_date_handle.set(input.value());
            }
        }
    };

    use_effect({
        let mut start_date = (*start_date_handle).clone();
        let mut end_date = (*end_date_handle).clone();
        let data = props.data.clone().unwrap_or_default();
        move || {
            if start_date.len() == 0 {
                start_date = "2023-05-01".to_owned();
            }
            if end_date.len() == 0 {
                end_date = "2023-06-01".to_owned();
            }
            match start_date.parse::<Date>() {
                Err(e) => gloo_console::log!(format!("{e:?}")),
                Ok(start_date) => match end_date.parse::<Date>() {
                    Err(e) => gloo_console::log!(format!("{e:?}")),
                    Ok(end_date) => {
                        match draw_timeline(&canvas_id, data, start_date) {
                            Err(e) => gloo_console::log!(format!("{e:?}")),
                            _ => {}
                        }
                    }
                },
            }
            || {}
        }
    });
    let start_date = (*start_date_handle).clone();
    let end_date = (*end_date_handle).clone();
    html! {
    <section>
        <h3>{props.title.clone()}</h3>
        <canvas
            id={props.canvas_id.clone()}
            style={"width: 80%; height: auto; max-width: 500px;"}
        />
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

// TODO: this method should be a view; computations performed by State object
fn draw_timeline(
    canvas_id: &str,
    data: TimelineData,
    start_date: Date,
) -> Result<(), Box<dyn Error>> {
    let end_date = start_date + Duration::days(data.len() as i64);
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let max = if let Some(value) = data
        .iter()
        .map(|(income, expenses, balance)| income.max(expenses).max(balance))
        .max()
    {
        math::round::floor(*value as f64 * 1.1, 1).max(100.0) as u32
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
                    .map(|(n, (b, i, e))| (start_date + Duration::days(n as i64), *i)),
            ),
    )?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(
                data.iter()
                    .enumerate()
                    .map(|(n, (b, i, e))| (start_date + Duration::days(n as i64), *e)),
            ),
    )?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLACK.mix(0.5).filled())
            .data(
                data.iter()
                    .enumerate()
                    .map(|(n, (b, i, e))| (start_date + Duration::days(n as i64), *b)),
            ),
    )?;

    root.present()?;
    Ok(())
}
