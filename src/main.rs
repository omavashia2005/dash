use std::time::{Duration, Instant};  
use ratatui::symbols;  
use ratatui::widgets::Block;  
use ratatui::widgets::canvas::{Canvas, Circle, Line, Points, Rectangle};  
use color_eyre::Result;  
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};  
use ratatui::style::{Color, Stylize};  
use ratatui::text::{Line as TextLine, Span};  
use ratatui::Frame;  
  
struct GameViewPort {  
    x0: f64,   
    x1: f64,   
    y0: f64,   
    y1: f64,  
}  

struct GameObject{
    x: f64, 
    y: f64
}

struct LayerOneViewport{
    x0: f64, 
    x1: f64, 
    y0: f64, 
    y1: f64
}


struct LayerTwoViewport{
    x0: f64, 
    x1: f64, 
    y0: f64, 
    y1: f64
}


fn update_layer_two_viewport(layer_two_viewport: &mut LayerTwoViewport){
    if layer_two_viewport.x0.abs() == 600.0{
        layer_two_viewport.x0 = -300.0; 
        layer_two_viewport.x1 = 300.0;
    }
    else {
        layer_two_viewport.x0 += 10.0; 
        layer_two_viewport.x1 += 10.0;
    }
}

fn update_layer_one_viewport(layer_one_viewport: &mut LayerOneViewport){
    if layer_one_viewport.x0.abs() == 600.0{
        layer_one_viewport.x0 = -300.0; 
        layer_one_viewport.x1 = 300.0;
    }
    else {
        layer_one_viewport.x0 += 5.0; 
        layer_one_viewport.x1 += 5.0;
    }
}

fn main() -> Result<()> {  
    color_eyre::install()?;  
  
    let game_object_viewport = &mut GameViewPort{ x0: -290.0, x1: 310.0, y0: -300.0, y1: 400.0 };  

    let layer_one_viewport = &mut LayerOneViewport{ x0: -300.0, x1: 300.0, y0: -300.0, y1: 400.0};

    let layer_two_viewport = &mut LayerTwoViewport{ x0: -300.0, x1: 300.0, y0: -300.0, y1: 400.0};

    let game_object = &mut GameObject{ x: -280.0, y: -10.0};

    let mut viewport_last_updated = Instant::now();  
    const VIEWPORT_UPDATE_INTERVAL: Duration = Duration::from_millis(100);


    let mut object_last_updated = Instant::now();  
    const OBJECT_UPDATE_INTERVAL: Duration = Duration::from_millis(70);  

    ratatui::run(|terminal| loop {  

        // Update state between draw calls  
        if viewport_last_updated.elapsed() >= VIEWPORT_UPDATE_INTERVAL {  
            update_layer_one_viewport(layer_one_viewport); 
            update_layer_two_viewport(layer_two_viewport);
            viewport_last_updated = Instant::now();  
        }  
        let lower_bound: f64 = 0.0; 
        let upper_bound: f64 = 80.0; 

        if event::poll(Duration::from_millis(10))? {
            // An event is ready! Now we read it safely.
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    println!("Quitting game!");
                    break Ok(());
                } else if key_event.code == KeyCode::Char('j'){
                    // make it jump
                    if object_last_updated.elapsed() >= OBJECT_UPDATE_INTERVAL && game_object.y <= (upper_bound - 10.0){  
                        game_object.y += 50.0;
                        object_last_updated = Instant::now();  
                    }  

                } else if key_event.code == KeyCode::Char('d'){
                    // make it jump  
                    if object_last_updated.elapsed() >= OBJECT_UPDATE_INTERVAL && game_object.y >= (lower_bound + 0.0){  
                        game_object.y -= 10.0;
                        object_last_updated = Instant::now();  
                    }  
                }

            }
        }

        // Render with current state  
        terminal.draw(|frame| render(frame, game_object, game_object_viewport, layer_one_viewport, layer_two_viewport))?;  

    })  
}  
  
fn render(frame: &mut Frame, game_object: &GameObject, game_view_port: &GameViewPort, layer_one_viewport: &LayerOneViewport, layer_two_viewport: &mut LayerTwoViewport) {  
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);  
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]).spacing(1);  
    let [top, main] = frame.area().layout(&vertical);  
    let [area] = main.layout(&horizontal);  
  
    let title = TextLine::from_iter([  
        Span::from("Canvas Widget").bold(),  
        Span::from(" (Press 'q' to quit)"),  
    ]);  
  
    frame.render_widget(title.centered(), top);  

    render_layer_one(frame, layer_one_viewport, area);  
    render_layer_two(frame, layer_two_viewport, area);
    render_main_canvas(frame, game_object, game_view_port, area);  
} 



fn render_layer_two(frame: &mut Frame, layer_two_viewport: &LayerTwoViewport, area: Rect){
    let background_canvas = Canvas::default()
        .x_bounds([layer_two_viewport.x0, layer_two_viewport.x1])
        .y_bounds([layer_two_viewport.y0, layer_two_viewport.y1])
        .paint(|ctx|{
            ctx.layer();
            ctx.draw(&Points{
                coords: &[(20.0, -40.0)], 
                color: Color::White
            });
            ctx.draw(&Points{
                coords: &[(40.0, -75.0)], 
                color: Color::White
            });
            ctx.draw(&Points{
                coords: &[(60.0, -90.0)], 
                color: Color::White
            });
        });

    frame.render_widget(background_canvas, area);
}
  




fn render_layer_one(frame: &mut Frame, layer_one_viewport: &LayerOneViewport, area: Rect){
    let background_canvas = Canvas::default()
        .x_bounds([layer_one_viewport.x0, layer_one_viewport.x1])
        .y_bounds([layer_one_viewport.y0, layer_one_viewport.y1])
        .paint(|ctx|{
            ctx.layer();
            ctx.draw(&Circle{
                color: Color::DarkGray, 
                radius: 50.0, 
                x: 220.0,
                y: 300.0
            });
            ctx.draw(&Circle{
                color: Color::DarkGray, 
                radius: 25.0, 
                x: 400.0,
                y: 300.0
            });
            ctx.draw(&Circle{
                color: Color::DarkGray, 
                radius: 70.0, 
                x: 100.0,
                y: 200.0
            });
        });

    frame.render_widget(background_canvas, area);
}
  
fn render_main_canvas(frame: &mut Frame, game_object: &GameObject, game_object_viewport: &GameViewPort, area: Rect) {  
    let canvas = Canvas::default()  
        .marker(symbols::Marker::HalfBlock)  
        .block(Block::bordered().title("DinoTerm"))  
        .x_bounds([game_object_viewport.x0, game_object_viewport.x1])  
        .y_bounds([game_object_viewport.y0, game_object_viewport.y1]) 
        .background_color(Color::Black)
        .paint(|ctx| {  
            ctx.draw(&Line{
                x1: game_object_viewport.x0,
                x2: game_object_viewport.x1,
                y1: -20.0, 
                y2: -20.0, 
                color: Color::DarkGray
            });
            // ctx.layer();
            ctx.draw(&Rectangle {  
                x: game_object.x, 
                y: game_object.y,  
                width: 10.0,  
                height: 10.0,  
                color: Color::White,  
            }); 
        });  

    
    frame.render_widget(canvas, area);  
}
