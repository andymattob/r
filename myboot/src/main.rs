use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main("System Boot")]
async fn main() {
    // 1. Specificera typen som f32 explicit för att undvika "ambiguous type"
    let mut frame_count: f32 = 0.0;
    
    // --- NYTT: LADDA BILDEN ---
    // Detta måste göras 'async' i början av main.
    // Se till att bilden finns i en mapp som heter 'assets' bredvid din Cargo.toml.
    let boot_texture_result = load_texture("/assets/boot_icon.png").await;

    // Hantera om bilden inte går att ladda (valfritt, men bra praxis)
    let boot_texture = match boot_texture_result {
        Ok(tex) => Some(tex),
        Err(e) => {
            eprintln!("Kunde inte ladda bilden: {}", e);
            None
        }
    };
    // ---------------------------

    let status_messages = vec![
        "LADDAR KÄRNMODULER...",
        "INITIERAR NÄTVERK...",
        "KONTROLLERAR FILSYSTEM...",
        "SYSTEM STARTAT."
    ];

    loop {
        frame_count += 0.05;
        let elapsed = get_time() as f32; // Konvertera till f32
        let pulse = (frame_count.sin() * 0.2) + 0.8;
        
        clear_background(Color::from_rgba(5, 5, 5, 255));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Logofärg med puls
        let logo_color = Color::from_rgba(
            (135.0 * pulse) as u8, 
            (206.0 * pulse) as u8, 
            (235.0 * pulse) as u8, 
            255
        );
        
        // Rita logotyp (pilen) lite högre upp för att ge plats
        let logo_y_offset = -120.0;
        draw_line(center_x, center_y + logo_y_offset - 60.0, center_x, center_y + logo_y_offset, 3.0, logo_color);
        draw_line(center_x, center_y + logo_y_offset - 60.0, center_x - 20.0, center_y + logo_y_offset - 40.0, 3.0, logo_color);
        draw_line(center_x, center_y + logo_y_offset - 60.0, center_x + 20.0, center_y + logo_y_offset - 40.0, 3.0, logo_color);
        
        // --- NYTT: RITA BILDEN ---
        if let Some(tex) = boot_texture {
            // Beräkna storlek och position
            let img_width = 80.0; // Önskad bredd på skärmen
            let img_height = tex.height() * (img_width / tex.width()); // Behåll bildförhållandet
            
            // Positionera bilden ovanför texten
            let img_x = center_x - (img_width / 2.0);
            let img_y = center_y + 10.0; // Justera detta värde för att placera den rätt vertikalt

            // Rita bilden
            draw_texture_ex(
                tex,
                img_x,
                img_y,
                WHITE, // Använd WHITE för att visa bilden med originalfärgerna
                DrawTextureParams {
                    dest_size: Some(vec2(img_width, img_height)),
                    ..Default::default()
                },
            );
        }
        // ------------------------

        // Laddningssnurra (flyttad lite ner)
        let spinner_y = center_y + 110.0;
        let radius = 20.0;
        let angle_speed = elapsed * 5.0;
        draw_circle_lines(center_x, spinner_y, radius, 3.0, Color::from_rgba(30, 30, 30, 255));
        
        draw_custom_arc(
            center_x, 
            spinner_y, 
            15, 
            radius, 
            angle_speed * (180.0 / PI), 
            90.0, 
            3.0, 
            SKYBLUE
        );

        // Text (flyttad lite ner för att ge plats åt bilden)
        let text_y = center_y + 160.0;
        draw_text("SYSTEM STARTAR", center_x - 110.0, text_y, 30.0, WHITE);

        let msg_idx = ((elapsed / 2.0) as usize).min(status_messages.len() - 1);
        draw_text(status_messages[msg_idx], center_x - 60.0, text_y + 30.0, 18.0, GRAY);

        next_frame().await
    }
}

// Hjälpfunktion för att rita bågen (ingår inte som standard i Macroquad)
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