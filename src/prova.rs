use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::slice::Windows;

use gtk::prelude::*;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use plot_graph::candles::plot;
use plot_graph::helpers::graph::DEFAULT_FONT;

const GLADE_UI_SOURCE: &'static str = include_str!("test.glade");
