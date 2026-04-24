use macroquad::prelude::*;
use chrono::Local;

#[derive(PartialEq)]
enum SystemState {
    Booting,
    Transition,
    Desktop,
}

struct Window {
    title: String,
    is_open: bool,
    color: Color,
    x: f32,
    y: f32,
    is_dragging: bool,
}

#[macroquad::main("My Rust OS")]
async fn main() {
    let mut state = SystemState::Booting;
    let mut transition_alpha = 0.0;
    
    // Variabler för att själva räkna ut mus-förflyttning (ersätter mouse_delta)
    let mut last_mouse_pos = mouse_position();
    
    let mut file_manager = Window { 
        title: "File Manager".to_string(), is_open: false, color: DARKGRAY, 
        x: 100.0, y: 100.0, is_dragging: false 
    };
    let mut cmd_window = Window { 
        title: "Terminal (CMD)".to_string(), is_open: false, color: BLACK, 
        x: 150.0, y: 150.0, is_dragging: false 
    };

    let wallpaper_tex = load_texture("assets/wallpaper.png").await.ok();
    // Vi lägger till ett _ för att tysta varningen om oanvänd variabel tills vi ritar den igen
    let _boot_tex = load_texture("assets/boot_icon.png").await.ok();

    loop {
        let elapsed = get_time() as f32;
        let (m_x, m_y) = mouse_position();
        
        // Räkna ut hur mycket musen flyttat sig sedan förra framen
        let dx = m_x - last_mouse_pos.0;
        let dy = m_y - last_mouse_pos.1;
        last_mouse_pos = (m_x, m_y);

        clear_background(BLACK);

        if state == SystemState::Booting && elapsed > 2.0 { state = SystemState::Transition; }
        if state == SystemState::Transition {
            transition_alpha += 0.02;
            if transition_alpha >= 1.0 { state = SystemState::Desktop; }
        }

        if state != SystemState::Booting {
            if let Some(ref bg) = wallpaper_tex {
                draw_texture_ex(bg, 0.0, 0.0, Color::new(1., 1., 1., transition_alpha),
                    DrawTextureParams { dest_size: Some(vec2(screen_width(), screen_height())), ..Default::default() });
            }
        }

        if state == SystemState::Desktop {
            // Skicka med dx och dy till fönster-funktionen
            draw_my_window(&mut file_manager, 400.0, 300.0, m_x, m_y, dx, dy);
            draw_my_window(&mut cmd_window, 500.0, 250.0, m_x, m_y, dx, dy);

            let bar_y = screen_height() - 50.0;
            draw_rectangle(0.0, bar_y, screen_width(), 50.0, Color::new(0.05, 0.05, 0.05, 0.95));

            let mut apps = [("FILES", &mut file_manager), ("CMD", &mut cmd_window)];
            for (i, (name, window)) in apps.iter_mut().enumerate() {
                let x = 20.0 + (i as f32 * 110.0);
                let is_hover = m_x > x && m_x < x + 100.0 && m_y > bar_y + 10.0 && m_y < bar_y + 40.0;
                draw_rectangle(x, bar_y + 10.0, 100.0, 30.0, if is_hover { GRAY } else { DARKGRAY });
                draw_text(name, x + 25.0, bar_y + 32.0, 20.0, WHITE);

                if is_hover && is_mouse_button_pressed(MouseButton::Left) {
                    window.is_open = true;
                }
            }

            let time_str = Local::now().format("%H:%M:%S").to_string();
            draw_text(&time_str, screen_width() - 120.0, bar_y + 32.0, 22.0, SKYBLUE);
        }

        if state != SystemState::Desktop {
            let a = 1.0 - transition_alpha;
            draw_text("SYSTEM READY", screen_width()/2.-80., screen_height()/2., 30.0, Color::new(1.,1.,1.,a));
        }

        next_frame().await
    }
}

fn draw_my_window(win: &mut Window, w: f32, h: f32, m_x: f32, m_y: f32, dx: f32, dy: f32) {
    if !win.is_open { return; }

    let header_h = 30.0;
    let is_over_header = m_x > win.x && m_x < win.x + w && m_y > win.y && m_y < win.y + header_h;

    if is_over_header && is_mouse_button_pressed(MouseButton::Left) {
        win.is_dragging = true;
    }
    if is_mouse_button_released(MouseButton::Left) {
        win.is_dragging = false;
    }
    
    if win.is_dragging {
        win.x += dx;
        win.y += dy;
    }

    draw_rectangle(win.x, win.y, w, h, win.color);
    draw_rectangle(win.x, win.y, w, header_h, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_text(&win.title, win.x + 10.0, win.y + 22.0, 20.0, WHITE);

    let close_x = win.x + w - 25.0;
    let is_over_close = m_x > close_x && m_x < close_x + 20.0 && m_y > win.y + 5.0 && m_y < win.y + 25.0;
    draw_text("X", close_x, win.y + 22.0, 25.0, if is_over_close { RED } else { LIGHTGRAY });

    if is_over_close && is_mouse_button_pressed(MouseButton::Left) {
        win.is_open = false;
    }

    if win.title.contains("CMD") {
        draw_text("C:\\Users\\Rust> _", win.x + 10.0, win.y + 60.0, 20.0, GREEN);
    } else {
        draw_text("[Drev C:]  [System]", win.x + 10.0, win.y + 60.0, 18.0, WHITE);
    }
}