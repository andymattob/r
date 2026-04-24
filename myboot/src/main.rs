use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(PartialEq)]
enum SystemState {
    Booting,
    Transition,
    Desktop,
}

#[macroquad::main("System Boot")]
async fn main() {
    let mut frame_count: f32 = 0.0;
    let mut state = SystemState::Booting;
    let mut transition_alpha = 0.0;

    // Ladda bilder
    let boot_tex = load_texture("assets/boot_icon.png").await.ok();
    let wallpaper_tex = load_texture("assets/wallpaper.png").await.ok();

    let status_messages = vec![
        "LADDAR KÄRNMODULER...",
        "INITIERAR NÄTVERK...",
        "SYSTEM STARTAT.",
    ];

    loop {
        frame_count += 0.05;
        let elapsed = get_time() as f32;
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        clear_background(BLACK);

        // --- TILLSTÅNDSLOGIK ---
        if state == SystemState::Booting && elapsed > 5.0 {
            state = SystemState::Transition;
        }

        if state == SystemState::Transition {
            transition_alpha += 0.01;
            if transition_alpha >= 1.0 {
                state = SystemState::Desktop;
            }
        }

        // --- RITA SKRIVBORDSBAKGRUND ---
        if state == SystemState::Transition || state == SystemState::Desktop {
            if let Some(ref bg) = wallpaper_tex {
                draw_texture_ex(
                    bg,
                    0.0,
                    0.0,
                    Color::new(1.0, 1.0, 1.0, transition_alpha),
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );
            }
        }

        // --- RITA MENY / TASKBAR (Endast i Desktop-läge) ---
        if state == SystemState::Desktop {
            let bar_height = 50.0;
            let bar_y = screen_height() - bar_height;
            
            // Rita själva fältet (halvgenomskinlig svart/grå)
            draw_rectangle(0.0, bar_y, screen_width(), bar_height, Color::new(0.1, 0.1, 0.1, 0.8));
            draw_line(0.0, bar_y, screen_width(), bar_y, 1.0, Color::new(1.0, 1.0, 1.0, 0.2));

            // Rita "Start"-knapp och några ikoner
            draw_text("START", 20.0, bar_y + 32.0, 25.0, WHITE);
            
            // Simulerade ikoner (rektanglar)
            let icons = ["WEB", "FILES", "CMD", "SETTINGS"];
            for (i, label) in icons.iter().enumerate() {
                let x_pos = 120.0 + (i as f32 * 90.0);
                draw_rectangle(x_pos, bar_y + 10.0, 70.0, 30.0, Color::new(0.3, 0.3, 0.3, 0.5));
                draw_text(label, x_pos + 10.0, bar_y + 30.0, 18.0, LIGHTGRAY);
            }

            // Klocka längst till höger
            draw_text("23:59", screen_width() - 80.0, bar_y + 32.0, 22.0, WHITE);
        }

        // --- RITA BOOTSCREEN (Tona ut) ---
        if state != SystemState::Desktop {
            let boot_alpha = 1.0 - transition_alpha;
            let boot_color = |r, g, b| Color::new(r, g, b, boot_alpha);

            if let Some(ref tex) = boot_tex {
                let img_w = 120.0;
                let img_h = tex.height() * (img_w / tex.width());
                draw_texture_ex(
                    tex,
                    center_x - (img_w / 2.0),
                    center_y - 120.0,
                    boot_color(1.0, 1.0, 1.0),
                    DrawTextureParams { dest_size: Some(vec2(img_w, img_h)), ..Default::default() },
                );
            }

            let spinner_y = center_y + 20.0;
            draw_custom_arc(center_x, spinner_y, 20, 22.0, (elapsed * 300.0) % 360.0, 90.0, 3.0, Color::new(0.5, 0.8, 0.9, boot_alpha));

            let text_y = center_y + 80.0;
            draw_text("SYSTEM STARTAR", center_x - 115.0, text_y, 30.0, boot_color(1.0, 1.0, 1.0));
        }

        next_frame().await
    }
}

fn draw_custom_arc(x: f32, y: f32, segments: u8, radius: f32, start_angle: f32, sweep_angle: f32, thickness: f32, color: Color) {
    for i in 0..segments {
        let a1 = (start_angle + (i as f32 * sweep_angle / segments as f32)) * PI / 180.0;
        let a2 = (start_angle + ((i + 1) as f32 * sweep_angle / segments as f32)) * PI / 180.0;
        draw_line(x + a1.cos() * radius, y + a1.sin() * radius, x + a2.cos() * radius, y + a2.sin() * radius, thickness, color);
    }
}