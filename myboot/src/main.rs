use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main("System Boot")]
async fn main() {
    // 1. Explicit f32 för att undvika "ambiguous numeric type"
    let mut frame_count: f32 = 0.0;
    
    // 2. Ladda bilden (Se till att bilden finns i: assets/boot_icon.png)
    let boot_texture_result = load_texture("assets/boot_icon.png").await;

    let boot_texture = match boot_texture_result {
        Ok(tex) => Some(tex),
        Err(e) => {
            eprintln!("Kunde inte ladda bilden: {}", e);
            None
        }
    };

    let status_messages = vec![
        "LADDAR KÄRNMODULER...",
        "INITIERAR NÄTVERK...",
        "KONTROLLERAR FILSYSTEM...",
        "SYSTEM STARTAT."
    ];

    loop {
        frame_count += 0.05;
        let elapsed = get_time() as f32;
        let pulse = (frame_count.sin() * 0.2) + 0.8;
        
        clear_background(Color::from_rgba(5, 5, 5, 255));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // --- 1. RITA BILDEN (Uppflyttad eftersom pilen är borta) ---
        if let Some(ref tex) = boot_texture {
            let img_width = 120.0; 
            let img_height = tex.height() * (img_width / tex.width());
            
            let img_x = center_x - (img_width / 2.0);
            
            // Justerad Y-position för att kompensera för den borttagna pilen
            let img_y = center_y - 120.0; 

            draw_texture_ex(
                tex, // Referensen skickas korrekt
                img_x,
                img_y,
                WHITE, 
                DrawTextureParams {
                    dest_size: Some(vec2(img_width, img_height)),
                    ..Default::default()
                },
            );
        }

        // --- 2. LADDNINGSSNURRA (Justerad Y-position) ---
        let spinner_y = center_y + 20.0; 
        let radius = 22.0;
        let angle_speed = elapsed * 5.0;
        draw_circle_lines(center_x, spinner_y, radius, 3.0, Color::from_rgba(40, 40, 40, 255));
        
        draw_custom_arc(
            center_x, 
            spinner_y, 
            20, 
            radius, 
            angle_speed * (180.0 / PI), 
            90.0, 
            3.0, 
            SKYBLUE
        );

        // --- 3. TEXT (Justerad Y-position) ---
        let text_y = center_y + 80.0; 
        draw_text("SYSTEM STARTAR", center_x - 115.0, text_y, 30.0, WHITE);

        let msg_idx = ((elapsed / 2.0) as usize).min(status_messages.len() - 1);
        draw_text(status_messages[msg_idx], center_x - 70.0, text_y + 35.0, 18.0, GRAY);

        // Version i hörnet
        draw_text("Version 2.3.1", screen_width() - 110.0, screen_height() - 20.0, 16.0, DARKGRAY);

        next_frame().await
    }
}

// Hjälpfunktion för att rita bågen (fixad för att undvika deg_to_rad-felet)
fn draw_custom_arc(x: f32, y: f32, segments: u8, radius: f32, start_angle: f32, sweep_angle: f32, thickness: f32, color: Color) {
    for i in 0..segments {
        let a1 = start_angle + (i as f32 * sweep_angle / segments as f32);
        let a2 = start_angle + ((i + 1) as f32 * sweep_angle / segments as f32);
        
        let x1 = x + (a1 * PI / 180.0).cos() * radius;
        let y1 = y + (a1 * PI / 180.0).sin() * radius;
        let x2 = x + (a2 * PI / 180.0).cos() * radius;
        let y2 = y + (a2 * PI / 180.0).sin() * radius;
        
        draw_line(x1, y1, x2, y2, thickness, color);
    }
}