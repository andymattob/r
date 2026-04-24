use macroquad::prelude::*;
use std::f32::consts::PI;
use chrono::Local; // För klockan

#[derive(PartialEq)]
enum SystemState {
    Booting,
    Transition,
    Desktop,
}

#[macroquad::main("System Boot")]
async fn main() {
    let mut state = SystemState::Booting;
    let mut transition_alpha = 0.0;

    // Ladda bilder
    let boot_tex = load_texture("assets/boot_icon.png").await.ok();
    let wallpaper_tex = load_texture("assets/wallpaper.png").await.ok();

    let mut opened_app = String::from("Ingen app öppen");

    loop {
        let elapsed = get_time() as f32;
        let mouse_pos = mouse_position();
        
        clear_background(BLACK);

        // --- TILLSTÅNDSLOGIK ---
        if state == SystemState::Booting && elapsed > 4.0 {
            state = SystemState::Transition;
        }
        if state == SystemState::Transition {
            transition_alpha += 0.02;
            if transition_alpha >= 1.0 { state = SystemState::Desktop; }
        }

        // --- RITA SKRIVBORD ---
        if state != SystemState::Booting {
            if let Some(ref bg) = wallpaper_tex {
                draw_texture_ex(bg, 0.0, 0.0, Color::new(1.0, 1.0, 1.0, transition_alpha),
                    DrawTextureParams { dest_size: Some(vec2(screen_width(), screen_height())), ..Default::default() });
            }
        }

        // --- MENY OCH INTERAKTION (Endast Desktop) ---
        if state == SystemState::Desktop {
            let bar_h = 50.0;
            let bar_y = screen_height() - bar_h;
            
            // Rita Taskbar
            draw_rectangle(0.0, bar_y, screen_width(), bar_h, Color::new(0.05, 0.05, 0.05, 0.9));

            // Knappar
            let apps = ["WEB", "FILES", "CMD"];
            for (i, name) in apps.iter().enumerate() {
                let x = 20.0 + (i as f32 * 100.0);
                let y = bar_y + 10.0;
                let w = 80.0;
                let h = 30.0;

                // Kolla om musen är över knappen (Hover)
                let is_hover = mouse_pos.0 > x && mouse_pos.0 < x + w && mouse_pos.1 > y && mouse_pos.1 < y + h;
                
                let btn_color = if is_hover { Color::new(0.4, 0.4, 0.4, 1.0) } else { Color::new(0.2, 0.2, 0.2, 1.0) };
                
                draw_rectangle(x, y, w, h, btn_color);
                draw_text(name, x + 15.0, y + 22.0, 20.0, WHITE);

                // Klick-logik
                if is_hover && is_mouse_button_pressed(MouseButton::Left) {
                    opened_app = format!("Öppnar: {}", name);
                }
            }

            // Visa vilken app som är vald
            draw_text(&opened_app, 20.0, 50.0, 30.0, WHITE);

            // Realtidsklocka
            let now = Local::now();
            let time_str = now.format("%H:%M:%S").to_string();
            draw_text(&time_str, screen_width() - 120.0, bar_y + 32.0, 25.0, SKYBLUE);
        }

        // --- BOOTSCREEN (Tona ut) ---
        if state != SystemState::Desktop {
            let alpha = 1.0 - transition_alpha;
            if let Some(ref tex) = boot_tex {
                draw_texture_ex(tex, screen_width()/2.0 - 60.0, screen_height()/2.0 - 100.0, Color::new(1.,1.,1., alpha),
                    DrawTextureParams { dest_size: Some(vec2(120.0, 120.0)), ..Default::default() });
            }
            draw_text("STARTAR...", screen_width()/2.0 - 60.0, screen_height()/2.0 + 60.0, 30.0, Color::new(1.,1.,1., alpha));
        }

        next_frame().await
    }
}