use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main("System Boot")]
async fn main() {
    // 1. Specificera typen som f32 explicit för att undvika "ambiguous type"
    let mut frame_count: f32 = 0.0;
    
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
        
        // Rita logotyp
        draw_line(center_x, center_y - 80.0, center_x, center_y - 20.0, 3.0, logo_color);
        draw_line(center_x, center_y - 80.0, center_x - 20.0, center_y - 60.0, 3.0, logo_color);
        draw_line(center_x, center_y - 80.0, center_x + 20.0, center_y - 60.0, 3.0, logo_color);
        
        // Laddningssnurra
        let radius = 25.0;
        let angle_speed = elapsed * 5.0;
        draw_circle_lines(center_x, center_y + 40.0, radius, 4.0, Color::from_rgba(30, 30, 30, 255));
        
        draw_custom_arc(
            center_x, 
            center_y + 40.0, 
            20, 
            radius, 
            angle_speed * (180.0 / PI), 
            90.0, 
            4.0, 
            SKYBLUE
        );

        // Text
        draw_text("SYSTEM STARTAR", center_x - 110.0, center_y + 100.0, 30.0, WHITE);

        let msg_idx = ((elapsed / 2.0) as usize).min(status_messages.len() - 1);
        draw_text(status_messages[msg_idx], center_x - 60.0, center_y + 130.0, 18.0, GRAY);

        next_frame().await
    }
}

// Ersatt deg_to_rad() med manuell uträkning för att undvika fel E0599
fn draw_custom_arc(x: f32, y: f32, segments: u8, radius: f32, start_angle: f32, sweep_angle: f32, thickness: f32, color: Color) {
    for i in 0..segments {
        let a1 = start_angle + (i as f32 * sweep_angle / segments as f32);
        let a2 = start_angle + ((i + 1) as f32 * sweep_angle / segments as f32);
        
        // Här gör vi om grader till radianer manuellt: (grad * PI / 180.0)
        let x1 = x + (a1 * PI / 180.0).cos() * radius;
        let y1 = y + (a1 * PI / 180.0).sin() * radius;
        let x2 = x + (a2 * PI / 180.0).cos() * radius;
        let y2 = y + (a2 * PI / 180.0).sin() * radius;
        
        draw_line(x1, y1, x2, y2, thickness, color);
    }
}