use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main("System Boot")]
async fn main() {
    let mut frame_count = 0.0;
    
    // Statusmeddelanden och tidtagning
    let status_messages = vec![
        "LADDAR KÄRNMODULER...",
        "INITIERAR NÄTVERK...",
        "KONTROLLERAR FILSYSTEM...",
        "SYSTEM STARTAT."
    ];

    loop {
        // Beräkna tid och animationer
        frame_count += 0.05;
        let elapsed = get_time();
        let pulse = (frame_count.sin() * 0.2) + 0.8;
        
        clear_background(Color::from_rgba(5, 5, 5, 255));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // --- 1. RITA LOGOTYP (PILEN) ---
        let logo_color = Color::from_rgba(
            (135.0 * pulse) as u8, 
            (206.0 * pulse) as u8, 
            (235.0 * pulse) as u8, 
            255
        );
        
        // Mittlinje
        draw_line(center_x, center_y - 80.0, center_x, center_y - 20.0, 3.0, logo_color);
        // Pilspets
        draw_line(center_x, center_y - 80.0, center_x - 20.0, center_y - 60.0, 3.0, logo_color);
        draw_line(center_x, center_y - 80.0, center_x + 20.0, center_y - 60.0, 3.0, logo_color);
        // Bågar (Vingar)
        draw_poly_lines(center_x, center_y - 30.0, 20, 30.0, 180.0, 3.0, logo_color);

        // --- 2. LADDNINGSSNURRA ---
        let radius = 25.0;
        let angle_speed = elapsed * 5.0;
        draw_circle_lines(center_x, center_y + 40.0, radius, 4.0, Color::from_rgba(30, 30, 30, 255));
        
        // Den roterande biten
        draw_arc(
            center_x, 
            center_y + 40.0, 
            20, 
            radius, 
            angle_speed as f32 * (180.0 / PI), 
            90.0, 
            4.0, 
            SKYBLUE
        );

        // --- 3. TEXT ---
        draw_text_ex(
            "SYSTEM STARTAR",
            center_x - 110.0,
            center_y + 100.0,
            TextParams {
                font_size: 30,
                color: Color::from_rgba(220, 220, 220, 255),
                ..Default::default()
            },
        );

        // Dynamisk status-text
        let msg_idx = ((elapsed / 2.0) as usize).min(status_messages.len() - 1);
        draw_text(
            status_messages[msg_idx],
            center_x - 60.0,
            center_y + 130.0,
            18.0,
            GRAY,
        );

        // Version
        draw_text("Version 2.3.1", screen_width() - 100.0, screen_height() - 20.0, 15.0, DARKGRAY);

        next_frame().await
    }
}

// Hjälpfunktion för att rita bågen (ingår inte som standard i Macroquad)
fn draw_arc(x: f32, y: f32, segments: u8, radius: f32, start_angle: f32, sweep_angle: f32, thickness: f32, color: Color) {
    for i in 0..segments {
        let angle1 = start_angle + (i as f32 * sweep_angle / segments as f32);
        let angle2 = start_angle + ((i + 1) as f32 * sweep_angle / segments as f32);
        
        let x1 = x + angle1.deg_to_rad().cos() * radius;
        let y1 = y + angle1.deg_to_rad().sin() * radius;
        let x2 = x + angle2.deg_to_rad().cos() * radius;
        let y2 = y + angle2.deg_to_rad().sin() * radius;
        
        draw_line(x1, y1, x2, y2, thickness, color);
    }
}