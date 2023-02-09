// use std::slice::Windows;
// use std::error::Error;
// use rand::Rng;

// use std::borrow::{BorrowMut};
use plot_graph::liquidity_plot;
use plot_graph::{
    candles::plot,
    helpers::{self, generate_data_series, graph::DEFAULT_FONT},
};

use std::cell::RefCell;
use std::cmp::Eq;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::{prelude::*, DrawingArea};
// use plotters::prelude::*;
use plotters_cairo::CairoBackend;

const GLADE_UI_SOURCE: &'static str = include_str!("prova.glade");
const TITLE: &str = "title"; //TODO
#[derive(PartialEq, PartialOrd, Eq, Hash, Debug)]
enum GoodKind {
    USD,
    YEN,
    YUAN,
}

fn build_ui(app: &gtk::Application) {
    // let v_sell_usd = generate_data_series(100., 200, -0.0985, 0.1);

    let builder = gtk::Builder::from_string(GLADE_UI_SOURCE);
    let window = builder.object::<gtk::Window>("MainWindow").unwrap();

    window.set_title(TITLE);

    let sell_usd: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.object("SellUSD").unwrap()));
    let sell_yen: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.object("SellYEN").unwrap()));
    let sell_yuan: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.object("SellYUAN").unwrap()));
    let buy_usd: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.object("BuyUSD").unwrap()));
    let buy_yen: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.object("BuyYEN").unwrap()));
    let buy_yuan: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.object("BuyYUAN").unwrap()));

    let liquidity: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.object("LiquidityPlot").unwrap()));

    // let sell_usd = Rc::new(RefCell::new(sell_usd));

    //new
    // let vettor = vec![sell_usd];

    let starting_day_scale = builder.object::<gtk::Scale>("StartingDayScale").unwrap();
    let ending_day_scale = builder.object::<gtk::Scale>("EndingDayScale").unwrap();
    let market = builder
        .object::<gtk::ComboBoxText>("MarketComboBox")
        .unwrap();

    


    let start_state = Rc::new(RefCell::new(starting_day_scale));
    let end_state = Rc::new(RefCell::new(ending_day_scale));
    let market_state = Rc::new(RefCell::new(market));

    // Bose -> sell -> vec
    //     buy -> vec
    //     liq -> vec

    // BFB ...  bose[0]12] bfb[1][12] doge[2][12]

    //BOSE
    // sell (4 valute)
    // EUR : vec
    // USD
    // YEN
    // YUAN
    // buy (4 valute)
    // liq (4 valute)

    window.set_application(Some(app));

    let max: f64 = 200.;
    start_state.borrow_mut().set_range(0., max-1.);
    end_state.borrow_mut().set_range(1., max);

    // let vettore2 = vec![v_sell_usd];

    let is_start_changing = true;
    let is_start_changing = Rc::new(RefCell::new(is_start_changing));

    let mut data: Vec<Vec<HashMap<GoodKind, Vec<f32>>>> = Vec::new();
    let liq: Vec<Vec<Vec<f32>>> = vec![
        // bose
        vec![generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1)],
        // bfb
        vec![generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1)],
        vec![generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1),generate_data_series(100., 200, -0.0985, 0.1)],
    ];

    data.push(vec![HashMap::new(), HashMap::new(), HashMap::new()]);
    // sell
    // data.get_mut(0)
    //     .unwrap()
    //     .get_mut(0)
    //     .unwrap()
    //     .insert(GoodKind::EUR, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(0)
        .unwrap()
        .get_mut(0)
        .unwrap()
        .insert(GoodKind::USD, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(0)
        .unwrap()
        .get_mut(0)
        .unwrap()
        .insert(GoodKind::YEN, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(0).unwrap().get_mut(0).unwrap().insert(
        GoodKind::YUAN,
        generate_data_series(100., 200, -0.0985, 0.1),
    ); //buy
    // data.get_mut(0)
    //     .unwrap()
    //     .get_mut(1)
    //     .unwrap()
    //     .insert(GoodKind::EUR, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(0)
        .unwrap()
        .get_mut(1)
        .unwrap()
        .insert(GoodKind::USD, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(0)
        .unwrap()
        .get_mut(1)
        .unwrap()
        .insert(GoodKind::YEN, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(0).unwrap().get_mut(1).unwrap().insert(
        GoodKind::YUAN,
        generate_data_series(100., 200, -0.0985, 0.1),
    );

    //other market
    //sell
    data.push(vec![HashMap::new(), HashMap::new(), HashMap::new()]);
    // data.get_mut(1)
    //     .unwrap()
    //     .get_mut(0)
    //     .unwrap()
    //     .insert(GoodKind::EUR, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(1)
        .unwrap()
        .get_mut(0)
        .unwrap()
        .insert(GoodKind::USD, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(1)
        .unwrap()
        .get_mut(0)
        .unwrap()
        .insert(GoodKind::YEN, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(1).unwrap().get_mut(0).unwrap().insert(
        GoodKind::YUAN,
        generate_data_series(100., 200, -0.0985, 0.1),
    ); // buy
    data.push(vec![HashMap::new(), HashMap::new(), HashMap::new()]);
    // data.get_mut(1)
    //     .unwrap()
    //     .get_mut(1)
    //     .unwrap()
    //     .insert(GoodKind::EUR, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(1)
        .unwrap()
        .get_mut(1)
        .unwrap()
        .insert(GoodKind::USD, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(1)
        .unwrap()
        .get_mut(1)
        .unwrap()
        .insert(GoodKind::YEN, generate_data_series(100., 200, -0.0985, 0.1));
    data.get_mut(1).unwrap().get_mut(1).unwrap().insert(
        GoodKind::YUAN,
        generate_data_series(100., 200, -0.0985, 0.1),
    );

    let data = Rc::new(data);
    let liq = Rc::new(liq);

    // plot sell usd,yen,yuan
    plot_drawing_area(
        &start_state,
        &end_state,
        &market_state,
        &is_start_changing,
        data.clone(),
        0,
        GoodKind::USD,
        &sell_usd,
    );
    plot_drawing_area(
        &start_state,
        &end_state,
        &market_state,
        &is_start_changing,
        data.clone(),
        0,
        GoodKind::YEN,
        &sell_yen,
    );
    plot_drawing_area(
        &start_state,
        &end_state,
        &market_state,
        &is_start_changing,
        data.clone(),
        0,
        GoodKind::YUAN,
        &sell_yuan,
    );

    // plot buy usd,yen,yuan
    plot_drawing_area(
        &start_state,
        &end_state,
        &market_state,
        &is_start_changing,
        data.clone(),
        1,
        GoodKind::USD,
        &buy_usd,
    );
    plot_drawing_area(
        &start_state,
        &end_state,
        &market_state,
        &is_start_changing,
        data.clone(),
        1,
        GoodKind::YEN,
        &buy_yen,
    );
    plot_drawing_area(
        &start_state,
        &end_state,
        &market_state,
        &is_start_changing,
        data.clone(),
        1,
        GoodKind::YUAN,
        &buy_yuan,
    );

    plot_liquidity_drawing_area(&start_state, &end_state, &market_state, &is_start_changing, liq.clone(), &liquidity);

    window.show_all();
}

fn plot_drawing_area(
    start_state: &Rc<RefCell<gtk::Scale>>,
    end_state: &Rc<RefCell<gtk::Scale>>,
    market_state: &Rc<RefCell<gtk::ComboBoxText>>,
    is_start_changing: &Rc<RefCell<bool>>,
    data: Rc<Vec<Vec<HashMap<GoodKind, Vec<f32>>>>>,
    // market_index: usize,
    op: usize,
    gk: GoodKind,
    draw_area: &Rc<RefCell<DrawingArea>>,
) {
    let start_cloned = start_state.clone();
    let end_cloned = end_state.clone();
    let is_start_changing_clone = is_start_changing.clone();
    let market_clone = market_state.clone();

    // set start ed end secondo parametri vettore (0, 245);

    draw_area.borrow().connect_draw(move |widget, cr| {
        // let state = start_cloned.borrow().clone();
        let mut start = start_cloned.borrow().clone().value() as usize;
        let mut end = end_cloned.borrow().clone().value() as usize;
        let market_index = market_clone.borrow().clone().active().unwrap() as usize;

        if start >= end {
            if *is_start_changing_clone.borrow() {
                end_cloned.borrow_mut().set_value(start as f64 + 1.);
                end = start + 1;
            } else {
                start_cloned.borrow_mut().set_value(end as f64 - 1.);
                start = end - 1;
            }
        }

        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        plot(
            &data[market_index][op][&gk].clone()[start..end],
            2,
            format!("{:?}",&gk).as_str(),
            backend,
            60,
            500,
            ("sans-serif", 50),
        )
        .unwrap();
        Inhibit(false)
    });

    let draw_area_clone = draw_area.clone();
    let is_start_changing_clone = is_start_changing.clone();

    start_state.borrow_mut().connect_value_changed(move |_| {
        *is_start_changing_clone.borrow_mut() = true;

        draw_area_clone.borrow().queue_draw();
    });

    let draw_area_clone = draw_area.clone();
    let is_start_changing_clone = is_start_changing.clone();

    end_state.borrow_mut().connect_value_changed(move |_| {
        *is_start_changing_clone.borrow_mut() = false;

        draw_area_clone.borrow().queue_draw();
    });

    //test
    let draw_area_clone = draw_area.clone();

    market_state.borrow_mut().connect_changed(move |_| {
        draw_area_clone.borrow().queue_draw();
    });
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


fn plot_liquidity_drawing_area(
    start_state: &Rc<RefCell<gtk::Scale>>,
    end_state: &Rc<RefCell<gtk::Scale>>,
    market_state: &Rc<RefCell<gtk::ComboBoxText>>,
    is_start_changing: &Rc<RefCell<bool>>,
    liq: Rc<Vec<Vec<Vec<f32>>>>,
    // market_index: usize,
    draw_area: &Rc<RefCell<DrawingArea>>,
) {
    let start_cloned = start_state.clone();
    let end_cloned = end_state.clone();
    let is_start_changing_clone = is_start_changing.clone();
    let market_clone = market_state.clone();

    // set start ed end secondo parametri vettore (0, 245);

    draw_area.borrow().connect_draw(move |widget, cr| {
        // let state = start_cloned.borrow().clone();
        let mut start = start_cloned.borrow().clone().value() as usize;
        let mut end = end_cloned.borrow().clone().value() as usize;
        let market_index = market_clone.borrow().clone().active().unwrap() as usize;

        if start >= end {
            if *is_start_changing_clone.borrow() {
                end_cloned.borrow_mut().set_value(start as f64 + 1.);
                end = start + 1;
            } else {
                start_cloned.borrow_mut().set_value(end as f64 - 1.);
                start = end - 1;
            }
        }

        // let state = state_cloned.borrow().clone();
        let w = widget.allocated_width();
        let h = widget.allocated_height();
        let backend = CairoBackend::new(cr, (w as u32, h as u32)).unwrap();
        liquidity_plot::plot(vec![liq[market_index][0].clone()[start..end].to_vec(),liq[market_index][1].clone()[start..end].to_vec(),liq[market_index][2].clone()[start..end].to_vec(),liq[market_index][3].clone()[start..end].to_vec()], vec![format!("USD"), format!("YEN"), format!("YUAN"),format!("EUR")], backend)
        .unwrap();
        Inhibit(false)
    });

    let draw_area_clone = draw_area.clone();
    let is_start_changing_clone = is_start_changing.clone();

    start_state.borrow_mut().connect_value_changed(move |_| {
        *is_start_changing_clone.borrow_mut() = true;
        draw_area_clone.borrow().queue_draw();
    });

    let draw_area_clone = draw_area.clone();
    let is_start_changing_clone = is_start_changing.clone();

    end_state.borrow_mut().connect_value_changed(move |_| {
        *is_start_changing_clone.borrow_mut() = false;
        draw_area_clone.borrow().queue_draw();
    });

    //test
    let draw_area_clone = draw_area.clone();

    market_state.borrow_mut().connect_changed(move |_| {
        draw_area_clone.borrow().queue_draw();
    });
}




/*

    let mut data: Vec<Vec<HashMap<GoodKind, Vec<f32>>>>

        data ->

            0 (bose) -> 

                0 (buy) ->
                    vec<f32>

                1 (sell) ->
                    vec<f32>

            1 (bfb) ->  ...
            2 (doge) -> ...


    let liq: Vec<Vec<Vec<f32>>>

        liq ->

            0 (bose) ->
                0(EUR) -> 
                    vec<f32>
                1(USD)

*/