use std::error::Error;

use crate::state::*;
use itertools::Itertools;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use std::ops::Sub;
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
    use_effect({
        let mut start_date = (*start_date_handle).clone();
        move || {
            if start_date.len() == 0 {
                start_date = "2023-05-01".to_owned();
            }
            match start_date.parse() {
                Err(e) => gloo_console::log!(format!("{e:?}")),
                Ok(start_date) => match draw_timeline(&canvas_id, dates, start_date) {
                    Err(e) => gloo_console::log!(format!("{e:?}")),
                    _ => {}
                },
            }
            || {}
        }
    });
    let start_date = (*start_date_handle).clone();
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
    </section>
    }
}

fn draw_timeline(
    canvas_id: &str,
    dates: DateSummaries,
    start_date: Date,
) -> Result<(), Box<dyn Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let income_data = dates
        .iter()
        .map(|(d, s)| (d.sub(start_date).num_days() as u32, s.income))
        .collect_vec();
    gloo_console::log!(format!("{income_data:?}"));
    let max = if let Some(value) = income_data.iter().map(|pair| pair.1).max() {
        math::round::floor(value as f64 * 1.1, 1).max(100.0) as u32
    } else {
        return Ok(());
    };

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .margin(5)
        //.caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((1u32..10u32).into_segmented(), 0u32..max)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_labels(10)
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Dollars")
        .x_desc("Date")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(income_data),
    )?;

    root.present()?;
    Ok(())
}
