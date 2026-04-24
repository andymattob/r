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
    let mut transition_alpha = 0.0; // För att tona in wallpaper

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

        // --- LOGIK FÖR TILLSTÅND ---
        // Efter 6 sekunder (eller när meddelandena är klara), gå till Transition
        if state == SystemState::Booting && elapsed > 6.0 {
            state = SystemState::Transition;
        }

        if state == SystemState::Transition {
            transition_alpha += 0.01; // Sänka/höja hastigheten på toningen här
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
                    Color::new(1.0, 1.0, 1.0, transition_alpha), // Tona in bilden
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );
            }
        }

        // --- RITA BOOTSCREEN (bara om vi inte är helt i Desktop) ---
        if state != SystemState::Desktop {
            let boot_alpha = 1.0 - transition_alpha; // Tona ut boot-elementen
            let pulse = (frame_count.sin() * 0.2) + 0.8;
            let boot_color = |r, g, b| Color::new(r, g, b, boot_alpha);

            // 1. Rita bilden
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

            // 2. Laddningssnurra
            let spinner_y = center_y + 20.0;
            draw_circle_lines(center_x, spinner_y, 22.0, 3.0, Color::new(0.1, 0.1, 0.1, boot_alpha));
            draw_custom_arc(center_x, spinner_y, 20, 22.0, (elapsed * 300.0) % 360.0, 90.0, 3.0, Color::new(0.5, 0.8, 0.9, boot_alpha));

            // 3. Text
            let text_y = center_y + 80.0;
            draw_text("SYSTEM STARTAR", center_x - 115.0, text_y, 30.0, boot_color(1.0, 1.0, 1.0));
            
            let msg_idx = ((elapsed / 2.0) as usize).min(status_messages.len() - 1);
            draw_text(status_messages[msg_idx], center_x - 70.0, text_y + 35.0, 18.0, boot_color(0.5, 0.5, 0.5));
        }

        // Om vi är i Desktop-läge, rita ett enkelt välkomstmeddelande
        if state == SystemState::Desktop {
            draw_text("VÄLKOMMEN", 40.0, 60.0, 40.0, WHITE);
            draw_text("Systemet är redo för användning.", 40.0, 90.0, 20.0, GRAY);
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