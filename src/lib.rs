use std::io::{self, Write};


pub struct CollisionPair {
    pub v_big_after_box_collision: f64,
    pub v_small_after_box_collision: f64,

    /// Is `None` if no wall collision happened after the box collision. This
    /// can only happen once at the very end.
    pub v_small_after_wall_collision: Option<f64>,
}

impl CollisionPair {
    fn num_collisions(&self) -> u64 {
        if self.v_small_after_wall_collision.is_some() {
            2
        } else {
            1
        }
    }
}

pub struct Collisions {
    collisions: Vec<CollisionPair>,
    mass_big: f64,
}

impl Collisions {
    /// Calculates all collisions in the staged scenario where the bigger box
    /// has the mass `mass_big`.
    ///
    /// The parameter must be greater than 0.
    pub fn calculate(mass_big: f64) -> Self {
        let mut collisions = Vec::new();

        let mut v_big = 1.0;
        let mut v_small = 0.0;

        let mass_small = 1.0;
        let mass_sum = mass_big + mass_small;

        loop {
            // Simulate collision between boxes
            let offset = 2.0 * ((mass_small * v_small + mass_big * v_big) / mass_sum);
            v_big = offset - v_big;
            v_small = offset - v_small;

            // Store results of this step
            let v_big_after_box_collision = v_big;
            let v_small_after_box_collision = v_small;

            // Check if we have reached the end of simulation.
            //
            // We only end here if the small box doesn't hit the wall again.
            // The velocity of the small box being positive means that it has
            // leftwards motion. So if there is no leftwards motion, we can
            // stop.
            let end_after_box_collision = v_small <= 0.0;

            // If necessary, simulate collision with wall
            let v_small_after_wall_collision = if end_after_box_collision {
                None
            } else {
                // The box is simply reflected
                v_small = -v_small;
                Some(v_small)
            };

            // Store this simulation step
            collisions.push(CollisionPair {
                v_big_after_box_collision,
                v_small_after_box_collision,
                v_small_after_wall_collision,
            });

            // Check if we can stop the simulation here.
            //
            // Note that the small box has a negative velocity after hitting
            // the wall (to the right). As velocity is specified as leftwards
            // motion, negative velocity means rightwards motion. `v_small <
            // v_big` means: the small box has more rightward motion as the big
            // one. That means the small box will catch up with the big one.
            // Negate that to get our stop condition.
            let end_after_wall_collision = !(v_small < v_big);

            if end_after_box_collision || end_after_wall_collision {
                break;
            }
        }

        Self {
            collisions,
            mass_big,
        }
    }

    pub fn count(&self) -> u64 {
        (self.collisions.len() as u64 - 1) * 2
            + self.collisions.last().unwrap().num_collisions()
    }

    pub fn write_svg(&self, mut w: impl Write) -> io::Result<()> {
        let radius = self.mass_big.sqrt();
        let svg_radius = 500.0;
        let size = 1400.0;

        writeln!(
            w,
            r#"<svg width="{}" height="{}" version="1.1" xmlns="http://www.w3.org/2000/svg">"#,
            size,
            size,
        )?;

        // Add style definitions
        writeln!(w, "<style>")?;
        writeln!(w, ".text {{ font-weight: bold; font-size: 20px; font-family: sans-serif; }}")?;
        writeln!(w, "</style>")?;

        // We don't want to have a transparent background
        writeln!(w, r#"<rect width="100%" height="100%" fill="white"/>"#)?;

        // Draw circle
        writeln!(
            w,
            r#"<circle cx="{}" cy="{}" r="{}" stroke="red" fill="transparent" stroke-width="3"/>"#,
            size / 2.0,
            size / 2.0,
            svg_radius,
        )?;

        // Draw coordinate system
        {
            let margin = 50.0;

            // Y axis
            writeln!(
                w,
                r#"<line x1="{}" x2="{}" y1="{}" y2="{}" stroke="orange" stroke-width="5"/>"#,
                size / 2.0,
                size / 2.0,
                margin,
                size - margin,
            )?;
            writeln!(
                w,
                concat!(
                    r#"<text x="{}" y="{}" text-anchor="middle" dominant-baseline="middle" "#,
                    r#"class="text">sqrt(m_b) * v_b</text>"#,
                ),
                size / 2.0,
                margin / 2.0,
            )?;

            // X axis
            writeln!(
                w,
                r#"<line x1="{}" x2="{}" y1="{}" y2="{}" stroke="orange" stroke-width="5"/>"#,
                margin,
                size - margin,
                size / 2.0,
                size / 2.0,
            )?;
            writeln!(
                w,
                concat!(
                    r#"<text x="{}" y="{}" text-anchor="middle" "#,
                    r#"class="text">sqrt(m_s) * v_s</text>"#,
                ),
                size - 2.0 * margin,
                size / 2.0 - 10.0,
            )?;
        }



        let point_size = 4.0;
        let line_width = 1.0;
        let sqrt_m_s = 1.0;
        let sqrt_m_b = self.mass_big.sqrt();

        let mut last_x = 0.0;
        let mut last_y = 1.0;

        let x_to_svg = |x: f64| (size / 2.0) + x * svg_radius;
        let y_to_svg = |y: f64| (size / 2.0) + (-y) * svg_radius;

        for pair in &self.collisions {
            let x = pair.v_small_after_box_collision / radius;
            let y = (pair.v_big_after_box_collision * sqrt_m_b) / radius;

            writeln!(
                w,
                r#"<circle cx="{}" cy="{}" r="{}" stroke="black" fill="blue" stroke-width="1"/>"#,
                x_to_svg(x),
                y_to_svg(y),
                point_size,
            )?;

            writeln!(
                w,
                r#"<line x1="{}" x2="{}" y1="{}" y2="{}" stroke="black" stroke-width="{}"/>"#,
                x_to_svg(last_x),
                x_to_svg(x),
                y_to_svg(last_y),
                y_to_svg(y),
                line_width,
            )?;

            if let Some(v_small) = pair.v_small_after_wall_collision {
                let x_reflected = v_small / radius;

                writeln!(
                    w,
                    r#"<circle cx="{}" cy="{}" r="{}" stroke="black" fill="blue" stroke-width="1"/>"#,
                    x_to_svg(x_reflected),
                    y_to_svg(y),
                    point_size,
                )?;

                writeln!(
                    w,
                    r#"<line x1="{}" x2="{}" y1="{}" y2="{}" stroke="black" stroke-width="{}"/>"#,
                    x_to_svg(x),
                    x_to_svg(x_reflected),
                    y_to_svg(y),
                    y_to_svg(y),
                    line_width,
                )?;

                last_x = x_reflected;
                last_y = y;
            }
        }




        writeln!(w, "</svg>")?;

        Ok(())
    }
}
