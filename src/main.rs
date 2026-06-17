use color_eyre::Result;
use ratatui::widgets::canvas::{Canvas, Circle, Line, Points, Rectangle};  
use std::time::{Duration, Instant};  
use ratatui::symbols;  
use ratatui::widgets::Block;  
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};  
use ratatui::style::{Color, Stylize};  
use ratatui::text::{Line as TextLine, Span};  
use ratatui::Frame;  
  
struct Viewport{
    x0: f64,   
    x1: f64,   
    y0: f64,   
    y1: f64,  
}

struct GameObject{
    x: f64, 
    y: f64, 
    y_velocity: f64,
    x_velocity: f64
}

struct ViewportUpdate{
        dx: f64,
        x0_max: f64,
        x0_default: f64,
        x1_default: f64,
}

fn update_viewport(viewport: &mut Viewport, changes: &ViewportUpdate){
    if viewport.x0.abs() >= changes.x0_max{
        viewport.x0 = changes.x0_default;
        viewport.x1 = changes.x1_default;
    } else {
        viewport.x0 += changes.dx;
        viewport.x1 += changes.dx;
    }
}

const GROUND_Y:f64 = -10.0;
const GROUND_X:f64 = 300.0;
const GRAVITY: f64 = 60.0;
const DT: f64 = 0.08;

fn update_position(game_object: &mut GameObject){

    game_object.y_velocity -= GRAVITY * DT;
    game_object.y += game_object.y_velocity * DT;
    // game_object.x += game_object.x_velocity * DT;
 
    if game_object.y <= GROUND_Y {
        game_object.y = GROUND_Y;
        game_object.y_velocity = 0.0;
    }

    // if game_object.x >= GROUND_X {
    //     game_object.x = 0.0;
    //     game_object.x_velocity = 10.0;
    // }
}

fn main() -> Result<()> {  
    color_eyre::install()?;  
  
    let game_object_viewport = &mut Viewport{ x0: 0.0, x1: 300.0, y0: -300.0, y1: 400.0 };

    let l1_viewport = &mut Viewport{ x0: -300.0, x1: 300.0, y0: -300.0, y1: 400.0};
    let l1_update = &ViewportUpdate{dx: 0.5, x0_max: 600.0, x0_default:-300.0, x1_default:300.0 };

    let l2_viewport = &mut Viewport{ x0: 0.0, x1: 300.0, y0: -300.0, y1: 400.0};
    let l2_update = &ViewportUpdate{dx: 10.0, x0_max: 600.0, x0_default:-300.0, x1_default:300.0 };

    let game_object = &mut GameObject{ x: 10.0, y: GROUND_Y, y_velocity: -10.0, x_velocity: 0.0};

    let mut viewport_updated = Instant::now();  
    const VIEWPORT_UPDATE_INTERVAL: Duration = Duration::from_millis(20);

    ratatui::run(|terminal| loop {  

        // if game_object.x_velocity >= 0.0{
        //     game_object.x_velocity -= DT;
        // }

        if viewport_updated.elapsed() >= VIEWPORT_UPDATE_INTERVAL {  
            update_viewport(l1_viewport, l1_update); 
            update_viewport(l2_viewport, l2_update);
            viewport_updated = Instant::now();  
        }  
        if event::poll(Duration::from_millis(10))? {
            // An event is ready! Now we read it safely.
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    println!("Quitting game!");
                    break Ok(());
                } else if key_event.code == KeyCode::Char(' '){
                    game_object.y_velocity += 90.0;
                    // game_object.x_velocity += 10.0; 
                }
            }
        }

        update_position(game_object); 

        let obstacle_viewport = &Viewport{
            x0: game_object_viewport.x0, 
            x1: game_object_viewport.x1,
            y0: game_object_viewport.y0,
            y1: game_object_viewport.y1
        };

        terminal.draw(|frame| render(frame, game_object, game_object_viewport, l1_viewport, l2_viewport, obstacle_viewport))?;  

    })  
}  
  
fn render(frame: &mut Frame, game_object: &GameObject, game_viewport: &Viewport, l1_viewport: &Viewport, l2_viewport: &mut Viewport, obsacle_viewport: &Viewport) {  
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);  
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]).spacing(1);  
    let [top, main] = frame.area().layout(&vertical);  
    let [area] = main.layout(&horizontal);  
  
    let title = TextLine::from_iter([  
        Span::from("Canvas Widget").bold(),  
        Span::from(" (Press 'q' to quit)"),  
    ]);  
  
    frame.render_widget(title.centered(), top);  

    render_layer_one(frame, l1_viewport, area);  
    render_layer_two(frame, l2_viewport, area);
    render_obstacles(frame, obsacle_viewport, area);
    render_main_canvas(frame, game_object, game_viewport, area);  
} 

fn render_layer_two(frame: &mut Frame, layer_two_viewport: &Viewport, area: Rect) {
    let start_x = -290.0;
    let y = -140.0;
    let width = 150.0;
    let height = 120.0;
    let count = 10;
    let background_canvas = Canvas::default()
        .x_bounds([layer_two_viewport.x0, layer_two_viewport.x1])
        .y_bounds([layer_two_viewport.y0, layer_two_viewport.y1])
        .paint(|ctx| {
            ctx.layer();
            for i in 0..count {
                ctx.draw(&Rectangle {
                    x: start_x + (i as f64 * width),
                    y,
                    width,
                    height,
                    color: Color::White,
                });
            }
            ctx.layer();
        });

    frame.render_widget(background_canvas, area);
}



fn render_obstacles(frame: &mut Frame, obsacle_viewport: &Viewport, area: Rect){

    let obstacles = Canvas::default()
        .x_bounds([obsacle_viewport.x0, obsacle_viewport.x1])
        .y_bounds([obsacle_viewport.y0, obsacle_viewport.y1])
        .paint(|ctx| {
            ctx.draw(&Rectangle{
                x: 10.0, 
                y: 20.0, 
                width: 20.0, 
                height: 40.0,
                color: Color::Cyan
            });
        });


    frame.render_widget(obstacles, area);



}




 

fn render_layer_one(frame: &mut Frame, layer_one_viewport: &Viewport, area: Rect){
    
    let background_canvas = Canvas::default()
        .x_bounds([layer_one_viewport.x0, layer_one_viewport.x1])
        .y_bounds([layer_one_viewport.y0, layer_one_viewport.y1])
        .paint(|ctx|{
            // rng to generate different values of 
            // value of r(radius) in the 50.0 - 80.0 range
            // x, y in the 150 - 310 float range, 
            // count in the 2 - 7 range
        
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
            ctx.layer();
            // rng to generate different values of 
            // x, y in the 150 - 310 float range, 
            // count in the 15 - 50 range

            ctx.draw(&Points{
                color: Color::White, 
                coords: &[(-200.0, 200.0)]
            });
        });
    frame.render_widget(background_canvas, area);
}
  
fn render_main_canvas(frame: &mut Frame, game_object: &GameObject, game_object_viewport: &Viewport, area: Rect) {  
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
            ctx.draw(&Rectangle {  
                x: game_object.x, 
                y: game_object.y,  
                width: 10.0,  
                height: 10.0,  
                color: Color::LightYellow,  
            }); 
        });  

    
    frame.render_widget(canvas, area);  
}
