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


const GROUND_Y:f64 = 0.0;
const GRAVITY: f64 = 50.0;
const DT: f64 = 0.08;
const PLAYER_X0: f64 = 0.0; 
const PLAYER_X1: f64 = 300.0; 

#[derive(PartialEq, Eq)]
enum  ViewportType {
    LayerOne, 
    LayerTwo, 
    Player, 
}

enum ObjectType{
    Rectangle{
        x: f64, 
        y:f64,
        width: f64, 
        height: f64, 
        color: Color
    }, 
    Circle{
        x:f64,
        y: f64, 
        radius: f64, 
        color: Color,
    }, 
    Line{
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: Color,
    }, 
}

struct Viewport{
    viewport_type: ViewportType, 
    x0: f64,   
    x1: f64,   
    y0: f64,   
    y1: f64,  
}

struct Player{
    x: f64, 
    y: f64, 
    y_velocity: f64,
}

struct ViewportUpdate{
    dx: f64,
    x0_max: f64,
    x0_default: f64,
    x1_default: f64,
}

struct Object{
    object_type: ObjectType, 
    viewport: Viewport,
    num_loops: Option<i32>,
}

#[derive(Debug)]
struct GameInfo{
    num_jumps: i64, 
    boundary_collisions: i64
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


fn update_player_pos(player: &mut Player, game_info: &mut GameInfo){
    player.y_velocity -= GRAVITY * DT;
    player.y += player.y_velocity * DT; 
    player.x += player.y_velocity * DT; 

    if player.x <= PLAYER_X0 || player.x >= PLAYER_X1{
        game_info.boundary_collisions += 1;

        if player.x <= PLAYER_X0{
            player.x = PLAYER_X0;
        } else if player.x >= PLAYER_X1 {  
            player.x = PLAYER_X1;
        }
    }

    if player.y <= GROUND_Y {
        player.y = GROUND_Y;
        player.y_velocity = 0.0;
    }
}

fn main() -> Result<()> {  
    color_eyre::install()?;  

    let l1_update = &ViewportUpdate{
        dx: 0.5, 
        x0_max: 600.0, 
        x0_default:-300.0, 
        x1_default:300.0 
    };

    let l2_update = &ViewportUpdate{
        dx: 10.0, 
        x0_max: 600.0, 
        x0_default:-300.0, 
        x1_default:300.0 
    };

    let mut game_info = GameInfo{
        num_jumps: 0, 
        boundary_collisions: 0, 
    };

    let mut viewports = [
        Viewport{
            viewport_type: ViewportType::LayerOne, 
            x0: 0.0, 
            x1: 300.0,
            y0: -300.0, 
            y1: 400.0
        },
        Viewport{
            viewport_type: ViewportType::LayerTwo,  
            x0: -300.0, 
            x1: 300.0, 
            y0: -300.0, 
            y1: 400.0
        },
        Viewport{
            viewport_type: ViewportType::Player, 
            x0: PLAYER_X0, 
            x1: PLAYER_X1, 
            y0: -300.0, 
            y1: 400.0 
        },
    ];

    let mut viewports_list: Vec<&mut Viewport> = viewports.iter_mut().collect();

    let player = &mut Player{ 
        x: 30.0,
        y: GROUND_Y, 
        y_velocity: -20.0
    };

    let mut viewport_updated = Instant::now();  
    const VIEWPORT_UPDATE_INTERVAL: Duration = Duration::from_millis(20);

    ratatui::run(|terminal| loop {  

        if viewport_updated.elapsed() >= VIEWPORT_UPDATE_INTERVAL {   
            let l1_viewport = viewports_list
                                .iter_mut()
                                .find(|v| v.viewport_type.eq(&ViewportType::LayerOne))
                                .unwrap();
            update_viewport(l1_viewport, l1_update);
            let l2_viewport = viewports_list
                                .iter_mut()
                                .find(|v| v.viewport_type.eq(&ViewportType::LayerTwo))
                                .unwrap();
            update_viewport(l2_viewport, l2_update);
            viewport_updated = Instant::now();  
        }  

        if event::poll(Duration::from_millis(10))?  && let Event::Key(key_event) = event::read()?{
            match key_event.code {
                KeyCode::Char('q') => {
                    let _ = std::fs::write("score.txt", format!("{:?}", game_info));
                    break Ok(());
                }
                KeyCode::Char(' ') => {
                    player.y_velocity += 90.0;
                    game_info.num_jumps += 1;
                }
                _ => {}
            }
        }


        update_player_pos(player, &mut game_info); 

        let test_object = &mut Object{
            object_type: ObjectType::Rectangle { x: 30.0, y: 20.0, width: 10.0, height: 10.0, color: Color::Red},
            viewport: Viewport{
                    viewport_type: ViewportType::LayerTwo,  
                    x0: -300.0, 
                    x1: 300.0, 
                    y0: -300.0, 
                    y1: 400.0
            },
            num_loops: None,
        };

        terminal.draw(|frame| render(frame, player, &viewports_list, test_object))?;  

    })  
}  

fn render(frame: &mut Frame, player: &Player, viewports: &Vec<&mut Viewport>, object: &Object) {  
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);  
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]).spacing(1);  
    let [top, main] = frame.area().layout(&vertical);  
    let [area] = main.layout(&horizontal);  
  
    let title = TextLine::from_iter([  
        Span::from("Canvas Widget").bold(),  
        Span::from(" (Press 'q' to quit)"),  
    ]);  
  
    frame.render_widget(title.centered(), top);  

    viewports.iter().for_each(|viewport|

        match viewport.viewport_type {
            ViewportType::LayerOne => render_layer_one(frame, viewport, area),
            ViewportType::LayerTwo => render_layer_two(frame, viewport, area),
            ViewportType::Player => render_main_canvas(frame, player, viewport, area),
        }

    );

    // renderer(frame, object, area);
} 


fn renderer(frame: &mut Frame, object: &Object, area: Rect){

    let entity = Canvas::default()
        .x_bounds([object.viewport.x0, object.viewport.x1])
        .y_bounds([object.viewport.y0, object.viewport.y1])
        .paint(|ctx|{
            match &object.object_type {
                ObjectType::Rectangle { x, y, width, height, color } => {
                    ctx.draw(&Rectangle{
                        x: *x, 
                        y: *y, 
                        width: *width, 
                        color: *color,
                        height: *height
                    })
                },
                ObjectType::Circle { x, y, radius, color } => {
                    ctx.draw(&Circle{
                        x: *x, 
                        y: *y, 
                        radius: *radius,
                        color: *color,
                    })
                },
                ObjectType::Line { x1, y1, x2, y2, color } => {
                    ctx.draw(&Line{
                        x1: *x1,
                        x2: *x2, 
                        y1: *y1, 
                        y2: *y2,
                        color: *color,
                    })
                },
                
            }
        });

    frame.render_widget(entity, area);
}


fn render_layer_one(frame: &mut Frame, layer_one_viewport: &Viewport, area: Rect){
    let layer_one = Canvas::default()
        .x_bounds([layer_one_viewport.x0, layer_one_viewport.x1])
        .y_bounds([layer_one_viewport.y0, layer_one_viewport.y1])
        .paint(|ctx|{
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
            ctx.draw(&Points{
                color: Color::White, 
                coords: &[(-200.0, 200.0)]
            });
        });
    frame.render_widget(layer_one, area);
}

fn render_layer_two(frame: &mut Frame, layer_two_viewport: &Viewport, area: Rect) {
    let start_x = -290.0;
    let y = -140.0;
    let width = 150.0;
    let height = 130.0;
    let count = 10;

    let layer_two = Canvas::default()
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
        });

    frame.render_widget(layer_two, area);
}

fn render_main_canvas(frame: &mut Frame, player: &Player, player_viewport: &Viewport, area: Rect) {  
    let main_canvas = Canvas::default()  
        .marker(symbols::Marker::Block)  
        .block(Block::bordered().title("DinoTerm"))  
        .x_bounds([player_viewport.x0, player_viewport.x1])  
        .y_bounds([player_viewport.y0, player_viewport.y1]) 
        .background_color(Color::Black)
        .paint(|ctx| {  
            ctx.draw(&Rectangle {  
                x: player.x, 
                y: player.y,  
                width: 10.0,  
                height: 10.0,  
                color: Color::LightYellow,  
            }); 
            ctx.layer();
            ctx.marker(symbols::Marker::HalfBlock);
            ctx.draw(&Line{
                x1: player_viewport.x0,
                x2: player_viewport.x1,
                y1: -10.0, 
                y2: -10.0, 
                color: Color::DarkGray
            });
        });   
    frame.render_widget(main_canvas, area);   

}
