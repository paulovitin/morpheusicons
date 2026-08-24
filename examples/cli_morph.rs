use morpheusicons::prelude::*;

fn main() -> Result<(), String> {
    println!("=== MorpheusIcons CLI Demo ===");

    let icon_pairs = [
        (Icon::Play, Icon::Pause, "play_to_pause"),
        (Icon::Sun, Icon::Moon, "sun_to_moon"),
        (Icon::Check, Icon::X, "check_to_x"),
        (Icon::Menu, Icon::X, "menu_to_x"),
        (Icon::ArrowRight, Icon::ArrowDown, "arrow_right_to_down"),
    ];

    for (from, to, name) in icon_pairs {
        println!("\nMorphing {:?} -> {:?}", from, to);
        let morpher = from.morph_to(to)?;

        // Print progress samples
        for step in 0..=5 {
            let t = step as f32 / 5.0;
            let path_d = morpher.to_svg_path(t);
            let full_svg = SvgRenderer::render_svg_document(&path_d, 24.0, 24.0, "#16a34a", 2.0);

            println!("  t = {:.1} -> SVG path length: {} chars", t, path_d.len());
            if step == 3 {
                println!("    Sample SVG XML (t=0.6):\n    {}", full_svg);
            }
        }

        // Save midpoint frame SVG to file
        let mid_path = morpher.to_svg_path(0.5);
        let mid_svg = SvgRenderer::render_svg_document(&mid_path, 24.0, 24.0, "#16a34a", 2.0);
        let filename = format!("assets/examples/{}_midpoint.svg", name);
        // Ensure output directory exists
        let _ = std::fs::create_dir_all("assets/examples");
        if std::fs::write(&filename, mid_svg).is_ok() {
            println!("  Saved midpoint SVG frame to './{}'", filename);
        }
    }

    Ok(())
}
