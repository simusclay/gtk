// use std::cell::RefCell;
use std::error::Error;
// use std::rc::Rc;
use std::slice::Windows;

use plot_graph::{helpers::{self, graph::DEFAULT_FONT},candles};

use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;

const GLADE_UI_SOURCE: &'static str = include_str!("prova.glade");
const TITLE: &str = "title"; //TODO

// #[derive(Clone, Copy)]
// struct PlottingState {
//     mean_x: f64,
//     mean_y: f64,
//     std_x: f64,
//     std_y: f64,
//     pitch: f64,
//     roll: f64,
// }

// impl PlottingState {
//     fn guassian_pdf(&self, x: f64, y: f64) -> f64 {
//         let x_diff = (x - self.mean_x) / self.std_x;
//         let y_diff = (y - self.mean_y) / self.std_y;
//         let exponent = -(x_diff * x_diff + y_diff * y_diff) / 2.0;
//         let denom = (2.0 * std::f64::consts::PI / self.std_x / self.std_y).sqrt();
//         let gaussian_pdf = 1.0 / denom;
//         gaussian_pdf * exponent.exp()
//     }
    // fn plot_pdf<'a, DB: DrawingBackend + 'a>(&self,backend: DB) -> Result<(), Box<dyn Error + 'a>> {
    //     let root = backend.into_drawing_area();

    //     root.fill(&WHITE)?;

    //     let mut chart = ChartBuilder::on(&root).build_cartesian_3d(
    //         -10.0f64..10.0,
    //         0.0f64..1.2,
    //         -10.0f64..10.0,
    //     )?;

    //     chart.with_projection(|mut p| {
    //         p.pitch = self.pitch;
    //         p.yaw = self.roll;
    //         p.scale = 0.7;
    //         p.into_matrix() // build the projection matrix
    //     });

    //     chart
    //         .configure_axes()
    //         .light_grid_style(BLACK.mix(0.15))
    //         .max_light_lines(3)
    //         .draw()?;
    //     let self_cloned = self.clone();
    //     chart.draw_series(
    //         SurfaceSeries::xoz(
    //             (-50..=50).map(|x| x as f64 / 5.0),
    //             (-50..=50).map(|x| x as f64 / 5.0),
    //             move |x, y| self_cloned.guassian_pdf(x, y),
    //         )
    //         .style_func(&|&v| (&HSLColor(240.0 / 360.0 - 240.0 / 360.0 * v, 1.0, 0.7)).into()),
    //     )?;

    //     root.present()?;
    //     Ok(())
    // }
// }

fn build_ui(app: &gtk::Application) {

    let v_sell_usd = helpers::generate_data_series(100., 200, -0.0985, 0.1);
    // let v_sell_usd = vec![1.5, 4., 2., 5., 10., 12., 3.,1.5, 4., 2., 5., 10., 12., 3., 5., 10., 12., 3.];
    let v_sell_yen = vec![1.5, 4., 2., 5., 10., 12., 3.,1.5, 4., 2., 5., 10., 12., 3., 5., 10., 12., 3.];
    let v_sell_yuan = vec![1.5, 4., 2., 5., 10., 12., 3.,1.5, 4., 2., 5., 10., 12., 3., 5., 10., 12., 3.];
    let v_buy_usd = vec![1.5, 4., 2., 5., 10., 12., 3.,1.5, 4., 2., 5., 10., 12., 3., 5., 10., 12., 3.];
    let v_buy_yen = vec![1.5, 4., 2., 5., 10., 12., 3.,1.5, 4., 2., 5., 10., 12., 3., 5., 10., 12., 3.];
    let v_buy_yuan = vec![1.5, 4., 2., 5., 10., 12., 3.,1.5, 4., 2., 5., 10., 12., 3., 5., 10., 12., 3.];
    let v_liquidity = vec![1.5, 4., 2., 5., 10., 12., 3.,1.5, 4., 2., 5., 10., 12., 3., 5., 10., 12., 3.];

    

    let builder = gtk::Builder::from_string(GLADE_UI_SOURCE);
    let window = builder.object::<gtk::Window>("MainWindow").unwrap();

    window.set_title(TITLE);

    let sell_usd: gtk::DrawingArea = builder.object("SellUSD").unwrap();
    let sell_yen: gtk::DrawingArea = builder.object("SellYEN").unwrap();
    let sell_yuan: gtk::DrawingArea = builder.object("SellYUAN").unwrap();
    let buy_usd: gtk::DrawingArea = builder.object("BuyUSD").unwrap();
    let buy_yen: gtk::DrawingArea = builder.object("BuyYEN").unwrap();
    let buy_yuan: gtk::DrawingArea = builder.object("BuyYUAN").unwrap();
    let liquidity: gtk::DrawingArea = builder.object("LiquidityPlot").unwrap();



    // let day_scale = builder.object::<gtk::Scale>("DayScale").unwrap();
    // let yaw_scale = builder.object::<gtk::Scale>("YawScale").unwrap();
    // let mean_x_scale = builder.object::<gtk::Scale>("MeanXScale").unwrap();
    // let mean_y_scale = builder.object::<gtk::Scale>("MeanYScale").unwrap();
    // let std_x_scale = builder.object::<gtk::Scale>("SDXScale").unwrap();
    // let std_y_scale = builder.object::<gtk::Scale>("SDYScale").unwrap();

    // let app_state = Rc::new(RefCell::new(PlottingState {
    // //     mean_x: mean_x_scale.value(),
    // //     mean_y: mean_y_scale.value(),
    // //     std_x: std_x_scale.value(),
    // //     std_y: std_y_scale.value(),
    // //     pitch: pitch_scale.value(),
    // //     roll: yaw_scale.value(),
    //         // day: day_scale.value()
    // }));

    window.set_application(Some(app));

    // let state_cloned = app_state.clone();
    sell_usd.connect_draw(move |widget, cr| {
        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        // candles::plot(&v_sell_usd.clone(), 2, "str", backend, 60, 500, DEFAULT_FONT).unwrap();
        try_plot(v_sell_usd.clone(), 2,"str",backend).unwrap();
        Inhibit(false)
    });
    sell_yen.connect_draw(move |widget, cr| {
        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        try_plot(v_sell_yen.clone(), 2,"str",backend).unwrap(); // copy backend?
        Inhibit(false)
    });
    sell_yuan.connect_draw(move |widget, cr| {
        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        try_plot(v_sell_yuan.clone(), 2,"str",backend).unwrap(); // copy backend?
        Inhibit(false)
    });
    buy_usd.connect_draw(move |widget, cr| {
        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        try_plot(v_buy_usd.clone(), 2,"str",backend).unwrap(); // copy backend?
        Inhibit(false)
    });
    buy_yen.connect_draw(move |widget, cr| {
        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        try_plot(v_buy_yen.clone(), 2,"str",backend).unwrap(); // copy backend?
        Inhibit(false)
    });
    buy_yuan.connect_draw(move |widget, cr| {
        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        try_plot(v_buy_yuan.clone(), 2,"str",backend).unwrap(); // copy backend?
        Inhibit(false)
    });
    liquidity.connect_draw(move |widget, cr| {
        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        try_plot(v_liquidity.clone(), 2,"str",backend).unwrap(); // copy backend?
        Inhibit(false)
    });

    // let handle_change =
    //     |what: &gtk::Scale, how: Box<dyn Fn(&mut PlottingState) -> &mut f64 + 'static>| {
    //         let app_state = app_state.clone();
    //         let drawing_area2 = drawing_area2.clone();
    //         what.connect_value_changed(move |target| {
    //             let mut state = app_state.borrow_mut();
    //             *how(&mut *state) = target.value();
    //             drawing_area2.queue_draw();
    //         });
    //     };

    // handle_change(&pitch_scale, Box::new(|s| &mut s.pitch));
    // handle_change(&yaw_scale, Box::new(|s| &mut s.roll));
    // handle_change(&mean_x_scale, Box::new(|s| &mut s.mean_x));
    // handle_change(&mean_y_scale, Box::new(|s| &mut s.mean_y));
    // handle_change(&std_x_scale, Box::new(|s| &mut s.std_x));
    // handle_change(&std_y_scale, Box::new(|s| &mut s.std_y));

    // handle_change(&day_scale, Box::new(|s| &mut s.day));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.example"), // TODO
        Default::default(),
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}


#[allow(dead_code)]
fn try_parse_data(v: Vec<f32>, candle_size: usize) -> Vec<(usize, f32, f32, f32, f32)> {
    fn try_parse_data_inner(
        data: Windows<f32>,
        candle_size: usize,
    ) -> Vec<(usize, f32, f32, f32, f32)> {
        data.enumerate()
            .filter(|(i, _)| i % candle_size == 0)
            .enumerate()
            .map(|(i, (_, v))| {
                (
                    (i + 1),
                    *v.first().unwrap(),
                    v.iter().copied().fold(f32::NAN, f32::max),
                    v.iter().copied().fold(f32::NAN, f32::min),
                    *v.last().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    }

    return if v.len() < candle_size {
        try_parse_data_inner(v.windows(v.len()), candle_size)
    } else {
        let last = *v.last().unwrap();
        let to_add = v.len() % candle_size;
        let new_v = [v, vec![last; to_add]].concat();
        // println!("Vector: {:?}", new_v);
        // println!("Window: {:?}", new_v.windows(candle_size + 1));
        try_parse_data_inner(new_v.windows(candle_size + 1), candle_size)
    };
}


#[allow(dead_code)]
fn try_plot<'a, DB: DrawingBackend + 'a>(v: Vec<f32>, candle_size: usize, _caption: &str, backend: DB) -> Result<(), Box<dyn Error + 'a>>{

    let root = backend.into_drawing_area();
    root.fill(&WHITE).expect("Error filling background.");

    let el_max = v.iter().copied().fold(f32::NAN, f32::max);
    let el_min = v.iter().copied().fold(f32::NAN, f32::min);
    let data = try_parse_data(v, candle_size);

    // Get date range
    let (start_date, end_date) = (data[0].0 - 1, data[data.len() - 1].0 + 1);

    // Basic chart configuration
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(60)
        // .caption(caption, ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(start_date..end_date, (el_min * 0.1)..(el_max * 1.2))
        .unwrap();

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()
        .unwrap();

    chart
        .draw_series(data.iter().map(|x| {
            CandleStick::new(
                x.0,
                x.1,
                x.2,
                x.3,
                x.4,
                RGBColor(98, 209, 61).filled(),
                RGBColor(209, 61, 61).filled(),
                (500. / (data.len() as f32 + 2.)).floor() as u32,
            )
        }))
        .unwrap();

    root.present()?;
    Ok(())

}