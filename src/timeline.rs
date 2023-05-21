use std::{error::Error, ops::{Sub, Add}};

use crate::state::*;
use chrono::{NaiveDate, Duration};
use itertools::Itertools;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub dates: DateSummaries,
    pub title: String,
    pub canvas_id: String,
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    let canvas_id = props.canvas_id.clone();
    let dates = props.dates.clone();

    let start_date_handle = use_state(String::new);
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
                    Ok(end_date) => match draw_timeline(&canvas_id, dates, start_date, end_date) {
                        Err(e) => gloo_console::log!(format!("{e:?}")),
                        _ => {}
                    },
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

fn draw_timeline(
    canvas_id: &str,
    dates: DateSummaries,
    start_date: Date,
    end_date: Date,
) -> Result<(), Box<dyn Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let income_data = dates
        .iter()
        .map(|(d, s)| (d.to_owned(), s.income))
        .collect_vec();
    gloo_console::log!(format!("income: {income_data:?}"));

    let expense_data = dates
        .iter()
        .map(|(d, s)| (d.to_owned(), s.expenses))
        .collect_vec();
    gloo_console::log!(format!("expenses: {expense_data:?}"));

    let mut balance_data = Vec::with_capacity((start_date -end_date).num_days() as usize);
    for i in 0..balance_data.len()-1 {
        balance_data[i] = (start_date + Duration::days(i as i64), income_data[i].1 - expense_data[i].1)
    }

    gloo_console::log!(format!("expenses: {expense_data:?}"));

    let max = if let Some(value) = income_data
        .iter()
        .chain(expense_data.iter())
        .map(|pair| pair.1)
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
            .data(income_data),
    )?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(expense_data),
    )?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLACK.mix(0.5).filled())
            .data(balance_data),
    )?;

    root.present()?;
    Ok(())
}
