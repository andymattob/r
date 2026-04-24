use macroquad::prelude::*;
use std::f32::consts::PI;
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
}

#[macroquad::main("My Rust OS")]
async fn main() {
    let mut state = SystemState::Booting;
    let mut transition_alpha = 0.0;
    
    // Fönster-instanser
    let mut file_manager = Window { title: "File Manager".to_string(), is_open: false, color: DARKGRAY };
    let mut cmd_window = Window { title: "Terminal (CMD)".to_string(), is_open: false, color: BLACK };

    let boot_tex = load_texture("assets/boot_icon.png").await.ok();
    let wallpaper_tex = load_texture("assets/wallpaper.png").await.ok();

    loop {
        let elapsed = get_time() as f32;
        let (m_x, m_y) = mouse_position();
        clear_background(BLACK);

        // --- BOOT LOGIK ---
        if state == SystemState::Booting && elapsed > 3.0 { state = SystemState::Transition; }
        if state == SystemState::Transition {
            transition_alpha += 0.02;
            if transition_alpha >= 1.0 { state = SystemState::Desktop; }
        }

        // --- RITA SKRIVBORD ---
        if state != SystemState::Booting {
            if let Some(ref bg) = wallpaper_tex {
                draw_texture_ex(bg, 0.0, 0.0, Color::new(1., 1., 1., transition_alpha),
                    DrawTextureParams { dest_size: Some(vec2(screen_width(), screen_height())), ..Default::default() });
            }
        }

        if state == SystemState::Desktop {
            // --- RITA ÖPPNA FÖNSTER ---
            draw_my_window(&mut file_manager, 100.0, 100.0, 400.0, 300.0, m_x, m_y);
            draw_my_window(&mut cmd_window, 150.0, 150.0, 500.0, 250.0, m_x, m_y);

            // --- TASKBAR ---
            let bar_y = screen_height() - 50.0;
            draw_rectangle(0.0, bar_y, screen_width(), 50.0, Color::new(0.05, 0.05, 0.05, 0.95));

            // Knappar för appar
            let apps = [("FILES", &mut file_manager), ("CMD", &mut cmd_window)];
            for (i, (name, window)) in apps.into_iter().enumerate() {
                let x = 20.0 + (i as f32 * 110.0);
                let btn_rect = (x, bar_y + 10.0, 100.0, 30.0);
                
                let is_hover = m_x > btn_rect.0 && m_x < btn_rect.0 + btn_rect.2 && m_y > btn_rect.1 && m_y < btn_rect.1 + btn_rect.3;
                draw_rectangle(btn_rect.0, btn_rect.1, btn_rect.2, btn_rect.3, if is_hover { GRAY } else { DARKGRAY });
                draw_text(name, x + 25.0, bar_y + 32.0, 20.0, WHITE);

                if is_hover && is_mouse_button_pressed(MouseButton::Left) {
                    window.is_open = true;
                }
            }

            // Klocka
            let time_str = Local::now().format("%H:%M:%S").to_string();
            draw_text(&time_str, screen_width() - 120.0, bar_y + 32.0, 22.0, CYAN);
        }

        // --- BOOTSCREEN (Tona ut) ---
        if state != SystemState::Desktop {
            let a = 1.0 - transition_alpha;
            draw_text("SYSTEM INITIALIZING...", screen_width()/2.-120., screen_height()/2., 30.0, Color::new(1.,1.,1.,a));
        }

        next_frame().await
    }
}

// Funktion för att rita ett generiskt fönster
fn draw_my_window(win: &mut Window, x: f32, y: f32, w: f32, h: f32, m_x: f32, m_y: f32) {
    if !win.is_open { return; }

    // Huvudfönster
    draw_rectangle(x, y, w, h, win.color);
    // Titelrad
    draw_rectangle(x, y, w, 30.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_text(&win.title, x + 10.0, y + 22.0, 20.0, WHITE);

    // Stäng-knapp (X)
    let close_x = x + w - 25.0;
    let is_over_close = m_x > close_x && m_x < close_x + 20.0 && m_y > y + 5.0 && m_y < y + 25.0;
    draw_text("X", close_x, y + 22.0, 25.0, if is_over_close { RED } else { LIGHTGRAY });

    if is_over_close && is_mouse_button_pressed(MouseButton::Left) {
        win.is_open = false;
    }

    // Innehåll baserat på titel
    if win.title.contains("CMD") {
        draw_text("C:\\Users\\Rust> _", x + 10.0, y + 60.0, 20.0, GREEN);
    } else {
        draw_text("[Drev C:]  [Dokument]  [Bilder]", x + 10.0, y + 60.0, 18.0, WHITE);
    }
}