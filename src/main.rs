use std::thread;
use std::time::{Duration, Instant};  
use ratatui::symbols;  
use ratatui::widgets::Block;  
use ratatui::widgets::canvas::{Canvas, Rectangle, Line};  
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

fn update_viewport_position(game_viewport: &mut GameViewPort){
    if game_viewport.x1.abs() == 480.0{
        game_viewport.x0 = -300.0;
        game_viewport.x1 = 300.0; 
    } else {
        game_viewport.x0 -= 20.0;
        game_viewport.x1 -= 20.0; 
    }
}

fn main() -> Result<()> {  
    color_eyre::install()?;  
  
    let game_view_port = &mut GameViewPort{ x0: -300.0, x1: 300.0, y0: -300.0, y1: 300.0 };  
    let game_object = &mut GameObject{ x: -350.0, y: 0.0};

    let mut last_update = Instant::now();  
    const UPDATE_INTERVAL: Duration = Duration::from_millis(50);  
  
    ratatui::run(|terminal| loop {  
        // Update state between draw calls  
        if last_update.elapsed() >= UPDATE_INTERVAL {  
            update_viewport_position(game_view_port); 
            last_update = Instant::now();  
        }  
  
        // Render with current state  
        terminal.draw(|frame| render(frame, game_object, game_view_port))?;  

        if event::poll(Duration::from_millis(50))? {
            // An event is ready! Now we read it safely.
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    println!("Quitting game!");
                    break Ok(());
                } else if key_event.code == KeyCode::Char('j'){
                    // make it jump
                    game_object.y += 10.0;
                }

            }
        }

    })  
}  
  
fn render(frame: &mut Frame, game_object: &GameObject, game_view_port: &GameViewPort) {  
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);  
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]).spacing(1);  
    let [top, main] = frame.area().layout(&vertical);  
    let [area] = main.layout(&horizontal);  
  
    let title = TextLine::from_iter([  
        Span::from("Canvas Widget").bold(),  
        Span::from(" (Press 'q' to quit)"),  
    ]);  
  
    frame.render_widget(title.centered(), top);  
  
    render_canvas(frame, game_object, game_view_port, area);  
}  
  
fn render_canvas(frame: &mut Frame, game_object: &GameObject, game_view_port: &GameViewPort, area: Rect) {  
    let canvas = Canvas::default()  
        .marker(symbols::Marker::HalfBlock)  
        .block(Block::bordered().title("DinoTerm"))  
        .x_bounds([game_view_port.x0, game_view_port.x1])  
        .y_bounds([game_view_port.y0, game_view_port.y1]) 
        .background_color(Color::Black)
        .paint(|ctx| {  
            ctx.draw(&Line{
                x1: game_view_port.x0,
                x2: game_view_port.x1,
                y1: -2.0, 
                y2: -2.0, 
                color: Color::DarkGray
            });
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
