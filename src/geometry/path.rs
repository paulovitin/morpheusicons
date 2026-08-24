use crate::geometry::point::Point;

/// Primitive command of an SVG path segment.
#[derive(Debug, Clone, PartialEq)]
pub enum PathSegment {
    MoveTo(Point),
    LineTo(Point),
    CubicTo {
        ctrl1: Point,
        ctrl2: Point,
        end: Point,
    },
    Close,
}

/// Represents a single continuous stroke subpath in an icon.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SubPath {
    pub segments: Vec<PathSegment>,
    pub is_closed: bool,
}

impl SubPath {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            is_closed: false,
        }
    }

    /// Returns all explicit control points / vertices defined by this subpath.
    pub fn start_point(&self) -> Option<Point> {
        self.segments.first().and_then(|seg| match seg {
            PathSegment::MoveTo(p) => Some(*p),
            PathSegment::LineTo(p) => Some(*p),
            PathSegment::CubicTo { end, .. } => Some(*end),
            PathSegment::Close => None,
        })
    }
}

/// Parsed representation of an SVG stroke path consisting of one or more subpaths.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IconPath {
    pub subpaths: Vec<SubPath>,
}

impl IconPath {
    /// Parses an SVG `d="..."` path string into an `IconPath`.
    pub fn parse(svg_path: &str) -> Result<Self, String> {
        let mut parser = PathLexer::new(svg_path);
        let mut subpaths = Vec::new();
        let mut current_subpath = SubPath::new();

        let mut current_pos = Point::ZERO;
        let mut subpath_start = Point::ZERO;
        let mut last_cubic_ctrl: Option<Point> = None;
        let mut last_quad_ctrl: Option<Point> = None;

        while let Some(cmd) = parser.next_command()? {
            match cmd {
                Command::MoveTo { abs, p } => {
                    if !current_subpath.segments.is_empty() {
                        subpaths.push(current_subpath);
                        current_subpath = SubPath::new();
                    }
                    let pt = if abs { p } else { current_pos + p };
                    current_pos = pt;
                    subpath_start = pt;
                    current_subpath.segments.push(PathSegment::MoveTo(pt));
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
                Command::LineTo { abs, p } => {
                    let pt = if abs { p } else { current_pos + p };
                    current_pos = pt;
                    current_subpath.segments.push(PathSegment::LineTo(pt));
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
                Command::HorizontalLineTo { abs, x } => {
                    let pt = Point::new(if abs { x } else { current_pos.x + x }, current_pos.y);
                    current_pos = pt;
                    current_subpath.segments.push(PathSegment::LineTo(pt));
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
                Command::VerticalLineTo { abs, y } => {
                    let pt = Point::new(current_pos.x, if abs { y } else { current_pos.y + y });
                    current_pos = pt;
                    current_subpath.segments.push(PathSegment::LineTo(pt));
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
                Command::CubicTo { abs, c1, c2, end } => {
                    let ctrl1 = if abs { c1 } else { current_pos + c1 };
                    let ctrl2 = if abs { c2 } else { current_pos + c2 };
                    let end_pt = if abs { end } else { current_pos + end };

                    current_subpath.segments.push(PathSegment::CubicTo {
                        ctrl1,
                        ctrl2,
                        end: end_pt,
                    });

                    current_pos = end_pt;
                    last_cubic_ctrl = Some(ctrl2);
                    last_quad_ctrl = None;
                }
                Command::SmoothCubicTo { abs, c2, end } => {
                    let ctrl1 = match last_cubic_ctrl {
                        Some(prev_c2) => current_pos * 2.0 - prev_c2,
                        None => current_pos,
                    };
                    let ctrl2 = if abs { c2 } else { current_pos + c2 };
                    let end_pt = if abs { end } else { current_pos + end };

                    current_subpath.segments.push(PathSegment::CubicTo {
                        ctrl1,
                        ctrl2,
                        end: end_pt,
                    });

                    current_pos = end_pt;
                    last_cubic_ctrl = Some(ctrl2);
                    last_quad_ctrl = None;
                }
                Command::QuadTo { abs, ctrl, end } => {
                    let c = if abs { ctrl } else { current_pos + ctrl };
                    let end_pt = if abs { end } else { current_pos + end };

                    // Convert quadratic bezier to cubic bezier
                    let c1 = current_pos + (c - current_pos) * (2.0 / 3.0);
                    let c2 = end_pt + (c - end_pt) * (2.0 / 3.0);

                    current_subpath.segments.push(PathSegment::CubicTo {
                        ctrl1: c1,
                        ctrl2: c2,
                        end: end_pt,
                    });

                    current_pos = end_pt;
                    last_quad_ctrl = Some(c);
                    last_cubic_ctrl = None;
                }
                Command::SmoothQuadTo { abs, end } => {
                    let c = match last_quad_ctrl {
                        Some(prev_c) => current_pos * 2.0 - prev_c,
                        None => current_pos,
                    };
                    let end_pt = if abs { end } else { current_pos + end };

                    let c1 = current_pos + (c - current_pos) * (2.0 / 3.0);
                    let c2 = end_pt + (c - end_pt) * (2.0 / 3.0);

                    current_subpath.segments.push(PathSegment::CubicTo {
                        ctrl1: c1,
                        ctrl2: c2,
                        end: end_pt,
                    });

                    current_pos = end_pt;
                    last_quad_ctrl = Some(c);
                    last_cubic_ctrl = None;
                }
                Command::ArcTo {
                    abs,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc_flag,
                    sweep_flag,
                    end,
                } => {
                    let end_pt = if abs { end } else { current_pos + end };
                    // Convert arc to cubic Beziers
                    let beziers = arc_to_cubic_beziers(
                        current_pos,
                        rx.abs(),
                        ry.abs(),
                        x_axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        end_pt,
                    );
                    for (c1, c2, e) in beziers {
                        current_subpath.segments.push(PathSegment::CubicTo {
                            ctrl1: c1,
                            ctrl2: c2,
                            end: e,
                        });
                    }
                    current_pos = end_pt;
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
                Command::Close => {
                    if current_pos != subpath_start {
                        current_subpath
                            .segments
                            .push(PathSegment::LineTo(subpath_start));
                    }
                    current_subpath.segments.push(PathSegment::Close);
                    current_subpath.is_closed = true;
                    current_pos = subpath_start;
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
            }
        }

        if !current_subpath.segments.is_empty() {
            subpaths.push(current_subpath);
        }

        Ok(IconPath { subpaths })
    }

    /// Returns a new `IconPath` with all coordinates scaled by the given factor.
    pub fn scale(&self, factor: f32) -> Self {
        let subpaths = self
            .subpaths
            .iter()
            .map(|sp| {
                let segments = sp
                    .segments
                    .iter()
                    .map(|seg| match seg {
                        PathSegment::MoveTo(p) => PathSegment::MoveTo(*p * factor),
                        PathSegment::LineTo(p) => PathSegment::LineTo(*p * factor),
                        PathSegment::CubicTo { ctrl1, ctrl2, end } => PathSegment::CubicTo {
                            ctrl1: *ctrl1 * factor,
                            ctrl2: *ctrl2 * factor,
                            end: *end * factor,
                        },
                        PathSegment::Close => PathSegment::Close,
                    })
                    .collect();
                SubPath {
                    segments,
                    is_closed: sp.is_closed,
                }
            })
            .collect();
        IconPath { subpaths }
    }
}

#[derive(Debug)]
enum Command {
    MoveTo { abs: bool, p: Point },
    LineTo { abs: bool, p: Point },
    HorizontalLineTo { abs: bool, x: f32 },
    VerticalLineTo { abs: bool, y: f32 },
    CubicTo { abs: bool, c1: Point, c2: Point, end: Point },
    SmoothCubicTo { abs: bool, c2: Point, end: Point },
    QuadTo { abs: bool, ctrl: Point, end: Point },
    SmoothQuadTo { abs: bool, end: Point },
    ArcTo {
        abs: bool,
        rx: f32,
        ry: f32,
        x_axis_rotation: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
        end: Point,
    },
    Close,
}

struct PathLexer<'a> {
    chars: std::str::Chars<'a>,
    current_cmd: Option<char>,
}

impl<'a> PathLexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            current_cmd: None,
        }
    }

    fn skip_whitespace_and_commas(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || c == ',' {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn parse_number(&mut self) -> Result<f32, String> {
        self.skip_whitespace_and_commas();
        let mut buf = String::new();

        if let Some(c) = self.peek_char() {
            if c == '+' || c == '-' {
                buf.push(c);
                self.chars.next();
            }
        }

        let mut has_dot = false;
        let mut has_e = false;

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                buf.push(c);
                self.chars.next();
            } else if c == '.' && !has_dot && !has_e {
                has_dot = true;
                buf.push(c);
                self.chars.next();
            } else if (c == 'e' || c == 'E') && !has_e {
                has_e = true;
                buf.push(c);
                self.chars.next();
                if let Some(next_c) = self.peek_char() {
                    if next_c == '+' || next_c == '-' {
                        buf.push(next_c);
                        self.chars.next();
                    }
                }
            } else {
                break;
            }
        }

        if buf.is_empty() || buf == "+" || buf == "-" {
            return Err("Expected number in SVG path".to_string());
        }

        buf.parse::<f32>()
            .map_err(|e| format!("Failed to parse float '{buf}': {e}"))
    }

    fn parse_point(&mut self) -> Result<Point, String> {
        let x = self.parse_number()?;
        let y = self.parse_number()?;
        Ok(Point::new(x, y))
    }

    fn parse_flag(&mut self) -> Result<bool, String> {
        self.skip_whitespace_and_commas();
        match self.peek_char() {
            Some('0') => {
                self.chars.next();
                Ok(false)
            }
            Some('1') => {
                self.chars.next();
                Ok(true)
            }
            Some(c) => Err(format!("Expected 0 or 1 flag, found '{c}'")),
            None => Err("Unexpected EOF reading flag".to_string()),
        }
    }

    fn next_command(&mut self) -> Result<Option<Command>, String> {
        self.skip_whitespace_and_commas();
        let first_char = match self.peek_char() {
            Some(c) => c,
            None => return Ok(None),
        };

        let is_cmd_char = first_char.is_ascii_alphabetic();
        let cmd_char = if is_cmd_char {
            self.chars.next();
            first_char
        } else if let Some(c) = self.current_cmd {
            // Implicit repeated command (e.g. M 10 10 20 20 -> second set is L 20 20)
            if c == 'M' {
                'L'
            } else if c == 'm' {
                'l'
            } else {
                c
            }
        } else {
            return Err(format!("Unexpected character: '{first_char}'"));
        };

        self.current_cmd = Some(cmd_char);
        let abs = cmd_char.is_uppercase();

        let cmd = match cmd_char.to_ascii_uppercase() {
            'M' => Command::MoveTo {
                abs,
                p: self.parse_point()?,
            },
            'L' => Command::LineTo {
                abs,
                p: self.parse_point()?,
            },
            'H' => Command::HorizontalLineTo {
                abs,
                x: self.parse_number()?,
            },
            'V' => Command::VerticalLineTo {
                abs,
                y: self.parse_number()?,
            },
            'C' => Command::CubicTo {
                abs,
                c1: self.parse_point()?,
                c2: self.parse_point()?,
                end: self.parse_point()?,
            },
            'S' => Command::SmoothCubicTo {
                abs,
                c2: self.parse_point()?,
                end: self.parse_point()?,
            },
            'Q' => Command::QuadTo {
                abs,
                ctrl: self.parse_point()?,
                end: self.parse_point()?,
            },
            'T' => Command::SmoothQuadTo {
                abs,
                end: self.parse_point()?,
            },
            'A' => Command::ArcTo {
                abs,
                rx: self.parse_number()?,
                ry: self.parse_number()?,
                x_axis_rotation: self.parse_number()?,
                large_arc_flag: self.parse_flag()?,
                sweep_flag: self.parse_flag()?,
                end: self.parse_point()?,
            },
            'Z' => Command::Close,
            _ => return Err(format!("Unknown command char: '{cmd_char}'")),
        };

        Ok(Some(cmd))
    }
}

/// Converts SVG Arc parameterization to cubic Beziers
fn arc_to_cubic_beziers(
    p0: Point,
    mut rx: f32,
    mut ry: f32,
    x_axis_rotation: f32,
    large_arc_flag: bool,
    sweep_flag: bool,
    p1: Point,
) -> Vec<(Point, Point, Point)> {
    if p0 == p1 || rx == 0.0 || ry == 0.0 {
        return Vec::new();
    }

    let phi = x_axis_rotation.to_radians();
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();

    let dx2 = (p0.x - p1.x) / 2.0;
    let dy2 = (p0.y - p1.y) / 2.0;

    let x1_prime = cos_phi * dx2 + sin_phi * dy2;
    let y1_prime = -sin_phi * dx2 + cos_phi * dy2;

    let rx_sq = rx * rx;
    let ry_sq = ry * ry;
    let x1_prime_sq = x1_prime * x1_prime;
    let y1_prime_sq = y1_prime * y1_prime;

    let radii_check = x1_prime_sq / rx_sq + y1_prime_sq / ry_sq;
    if radii_check > 1.0 {
        let scale = radii_check.sqrt();
        rx *= scale;
        ry *= scale;
    }

    let rx_sq = rx * rx;
    let ry_sq = ry * ry;

    let sign = if large_arc_flag == sweep_flag {
        -1.0
    } else {
        1.0
    };
    let sq_num = (rx_sq * ry_sq) - (rx_sq * y1_prime_sq) - (ry_sq * x1_prime_sq);
    let sq_den = (rx_sq * y1_prime_sq) + (ry_sq * x1_prime_sq);
    let coef = sign * (sq_num.max(0.0) / sq_den.max(1e-6)).sqrt();

    let cx_prime = coef * ((rx * y1_prime) / ry);
    let cy_prime = coef * (-(ry * x1_prime) / rx);

    let cx = cos_phi * cx_prime - sin_phi * cy_prime + (p0.x + p1.x) / 2.0;
    let cy = sin_phi * cx_prime + cos_phi * cy_prime + (p0.y + p1.y) / 2.0;

    let v1_x = (x1_prime - cx_prime) / rx;
    let v1_y = (y1_prime - cy_prime) / ry;
    let v2_x = (-x1_prime - cx_prime) / rx;
    let v2_y = (-y1_prime - cy_prime) / ry;

    let theta1 = angle_between(1.0, 0.0, v1_x, v1_y);
    let mut d_theta = angle_between(v1_x, v1_y, v2_x, v2_y);

    if !sweep_flag && d_theta > 0.0 {
        d_theta -= std::f32::consts::TAU;
    } else if sweep_flag && d_theta < 0.0 {
        d_theta += std::f32::consts::TAU;
    }

    let segments = (d_theta.abs() / (std::f32::consts::FRAC_PI_2)).ceil() as usize;
    let delta = d_theta / (segments as f32);
    let t = (8.0 / 3.0) * (delta / 4.0).sin() * (delta / 4.0).sin() / (delta / 2.0).sin();

    let mut result = Vec::new();
    let mut current_angle = theta1;

    for _ in 0..segments {
        let next_angle = current_angle + delta;

        let cos_a1 = current_angle.cos();
        let sin_a1 = current_angle.sin();
        let cos_a2 = next_angle.cos();
        let sin_a2 = next_angle.sin();

        let e1 = Point::new(
            cos_phi * rx * cos_a1 - sin_phi * ry * sin_a1 + cx,
            sin_phi * rx * cos_a1 + cos_phi * ry * sin_a1 + cy,
        );
        let e2 = Point::new(
            cos_phi * rx * cos_a2 - sin_phi * ry * sin_a2 + cx,
            sin_phi * rx * cos_a2 + cos_phi * ry * sin_a2 + cy,
        );

        let d1 = Point::new(
            -cos_phi * rx * sin_a1 - sin_phi * ry * cos_a1,
            -sin_phi * rx * sin_a1 + cos_phi * ry * cos_a1,
        );
        let d2 = Point::new(
            -cos_phi * rx * sin_a2 - sin_phi * ry * cos_a2,
            -sin_phi * rx * sin_a2 + cos_phi * ry * cos_a2,
        );

        let c1 = e1 + d1 * t;
        let c2 = e2 - d2 * t;

        result.push((c1, c2, e2));
        current_angle = next_angle;
    }

    result
}

fn angle_between(ux: f32, uy: f32, vx: f32, vy: f32) -> f32 {
    let dot = ux * vx + uy * vy;
    let len = ((ux * ux + uy * uy) * (vx * vx + vy * vy)).sqrt();
    let cos_val = (dot / len.max(1e-6)).clamp(-1.0, 1.0);
    let sign = if ux * vy - uy * vx >= 0.0 { 1.0 } else { -1.0 };
    sign * cos_val.acos()
}


#[cfg(test)]
mod tests {
    use super::*;

    // --- MoveTo & LineTo ---

    #[test]
    fn test_parse_moveto_lineto_absolute() {
        let path = IconPath::parse("M10 20 L30 40").unwrap();
        assert_eq!(path.subpaths.len(), 1);
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments.len(), 2);
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(10.0, 20.0)));
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(30.0, 40.0)));
    }

    #[test]
    fn test_parse_moveto_lineto_relative() {
        let path = IconPath::parse("m10 20 l5 5").unwrap();
        assert_eq!(path.subpaths.len(), 1);
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(10.0, 20.0)));
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(15.0, 25.0)));
    }

    #[test]
    fn test_parse_implicit_lineto_after_moveto() {
        // After M, subsequent coordinate pairs are treated as implicit L commands
        let path = IconPath::parse("M0 0 10 10 20 20").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments.len(), 3);
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(0.0, 0.0)));
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(10.0, 10.0)));
        assert_eq!(sp.segments[2], PathSegment::LineTo(Point::new(20.0, 20.0)));
    }

    #[test]
    fn test_parse_implicit_lineto_after_relative_moveto() {
        let path = IconPath::parse("m5 5 10 10").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments.len(), 2);
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(5.0, 5.0)));
        // implicit l, so relative: 5+10=15, 5+10=15
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(15.0, 15.0)));
    }

    // --- Horizontal & Vertical lines ---

    #[test]
    fn test_parse_horizontal_absolute() {
        let path = IconPath::parse("M0 5 H10").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(10.0, 5.0)));
    }

    #[test]
    fn test_parse_horizontal_relative() {
        let path = IconPath::parse("M3 5 h7").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(10.0, 5.0)));
    }

    #[test]
    fn test_parse_vertical_absolute() {
        let path = IconPath::parse("M5 0 V10").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(5.0, 10.0)));
    }

    #[test]
    fn test_parse_vertical_relative() {
        let path = IconPath::parse("M5 3 v7").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(5.0, 10.0)));
    }

    // --- Cubic Bezier ---

    #[test]
    fn test_parse_cubic_absolute() {
        let path = IconPath::parse("M0 0 C10 10 20 20 30 30").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(
            sp.segments[1],
            PathSegment::CubicTo {
                ctrl1: Point::new(10.0, 10.0),
                ctrl2: Point::new(20.0, 20.0),
                end: Point::new(30.0, 30.0),
            }
        );
    }

    #[test]
    fn test_parse_cubic_relative() {
        let path = IconPath::parse("M5 5 c5 5 10 10 15 15").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(
            sp.segments[1],
            PathSegment::CubicTo {
                ctrl1: Point::new(10.0, 10.0),
                ctrl2: Point::new(15.0, 15.0),
                end: Point::new(20.0, 20.0),
            }
        );
    }

    // --- Smooth Cubic (S) ---

    #[test]
    fn test_parse_smooth_cubic_absolute() {
        let path = IconPath::parse("M0 0 C1 2 3 4 5 6 S9 10 11 12").unwrap();
        let sp = &path.subpaths[0];
        // After C, last ctrl2 = (3,4), current pos = (5,6)
        // Reflected ctrl1 = 2*(5,6) - (3,4) = (7, 8)
        assert_eq!(
            sp.segments[2],
            PathSegment::CubicTo {
                ctrl1: Point::new(7.0, 8.0),
                ctrl2: Point::new(9.0, 10.0),
                end: Point::new(11.0, 12.0),
            }
        );
    }

    #[test]
    fn test_parse_smooth_cubic_no_previous() {
        // S without a preceding C — ctrl1 should equal current_pos
        let path = IconPath::parse("M5 5 S10 10 15 15").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(
            sp.segments[1],
            PathSegment::CubicTo {
                ctrl1: Point::new(5.0, 5.0),
                ctrl2: Point::new(10.0, 10.0),
                end: Point::new(15.0, 15.0),
            }
        );
    }

    // --- Quadratic Bezier (Q) ---

    #[test]
    fn test_parse_quad_absolute() {
        let path = IconPath::parse("M0 0 Q10 10 20 0").unwrap();
        let sp = &path.subpaths[0];
        // Q -> C conversion: ctrl = (10,10), start=(0,0), end=(20,0)
        // c1 = start + (ctrl - start) * 2/3 = (0,0) + (10,10)*2/3 = (6.667, 6.667)
        // c2 = end + (ctrl - end) * 2/3 = (20,0) + (-10,10)*2/3 = (13.333, 6.667)
        if let PathSegment::CubicTo { ctrl1, ctrl2, end } = &sp.segments[1] {
            assert!((ctrl1.x - 6.6667).abs() < 0.01);
            assert!((ctrl1.y - 6.6667).abs() < 0.01);
            assert!((ctrl2.x - 13.3333).abs() < 0.01);
            assert!((ctrl2.y - 6.6667).abs() < 0.01);
            assert!((end.x - 20.0).abs() < 1e-6);
            assert!((end.y - 0.0).abs() < 1e-6);
        } else {
            panic!("Expected CubicTo segment");
        }
    }

    #[test]
    fn test_parse_quad_relative() {
        let path = IconPath::parse("M5 5 q5 5 10 0").unwrap();
        let sp = &path.subpaths[0];
        // ctrl = (5,5) + (5,5) = (10,10), end = (5,5) + (10,0) = (15,5)
        if let PathSegment::CubicTo { end, .. } = &sp.segments[1] {
            assert!((end.x - 15.0).abs() < 1e-6);
            assert!((end.y - 5.0).abs() < 1e-6);
        } else {
            panic!("Expected CubicTo segment");
        }
    }

    // --- Smooth Quadratic (T) ---

    #[test]
    fn test_parse_smooth_quad() {
        let path = IconPath::parse("M0 0 Q10 10 20 0 T40 0").unwrap();
        let sp = &path.subpaths[0];
        // After Q10 10 20 0: last_quad_ctrl = (10,10), current_pos = (20,0)
        // T: reflected ctrl = 2*(20,0) - (10,10) = (30, -10), end = (40,0)
        if let PathSegment::CubicTo { end, .. } = &sp.segments[2] {
            assert!((end.x - 40.0).abs() < 1e-6);
            assert!((end.y - 0.0).abs() < 1e-6);
        } else {
            panic!("Expected CubicTo segment");
        }
    }

    // --- Arc (A) ---

    #[test]
    fn test_parse_arc_absolute() {
        let path = IconPath::parse("M10 80 A25 25 0 0 1 50 80").unwrap();
        let sp = &path.subpaths[0];
        // Arc is converted to cubic bezier(s)
        assert!(sp.segments.len() >= 2); // MoveTo + at least 1 CubicTo
        // Last segment should end at (50, 80)
        let last_seg = sp.segments.last().unwrap();
        if let PathSegment::CubicTo { end, .. } = last_seg {
            assert!((end.x - 50.0).abs() < 0.1);
            assert!((end.y - 80.0).abs() < 0.1);
        } else {
            panic!("Expected CubicTo for arc");
        }
    }

    #[test]
    fn test_parse_arc_relative() {
        let path = IconPath::parse("M10 80 a25 25 0 0 1 40 0").unwrap();
        let sp = &path.subpaths[0];
        // end = (10+40, 80+0) = (50, 80)
        let last_seg = sp.segments.last().unwrap();
        if let PathSegment::CubicTo { end, .. } = last_seg {
            assert!((end.x - 50.0).abs() < 0.1);
            assert!((end.y - 80.0).abs() < 0.1);
        } else {
            panic!("Expected CubicTo for arc");
        }
    }

    #[test]
    fn test_parse_arc_zero_radius_produces_no_segments() {
        let path = IconPath::parse("M10 10 A0 0 0 0 1 20 20").unwrap();
        let sp = &path.subpaths[0];
        // With zero radii, arc_to_cubic_beziers returns empty vec, so no CubicTo added
        assert_eq!(sp.segments.len(), 1); // Only MoveTo
    }

    #[test]
    fn test_parse_arc_same_point_produces_no_segments() {
        let path = IconPath::parse("M10 10 A5 5 0 0 1 10 10").unwrap();
        let sp = &path.subpaths[0];
        // p0 == p1, so arc returns empty
        assert_eq!(sp.segments.len(), 1); // Only MoveTo
    }

    // --- Close (Z/z) ---

    #[test]
    fn test_parse_close() {
        let path = IconPath::parse("M0 0 L10 0 L10 10 Z").unwrap();
        let sp = &path.subpaths[0];
        assert!(sp.is_closed);
        // Should have: MoveTo, LineTo, LineTo, LineTo(back to start), Close
        assert!(sp.segments.iter().any(|s| *s == PathSegment::Close));
    }

    #[test]
    fn test_parse_close_lowercase() {
        let path = IconPath::parse("M0 0 L10 0 L10 10 z").unwrap();
        assert!(path.subpaths[0].is_closed);
    }

    #[test]
    fn test_close_adds_line_back_when_not_at_start() {
        let path = IconPath::parse("M0 0 L10 10 Z").unwrap();
        let sp = &path.subpaths[0];
        // Should add LineTo(0,0) before Close
        assert_eq!(sp.segments.len(), 4); // MoveTo, LineTo(10,10), LineTo(0,0), Close
        assert_eq!(sp.segments[2], PathSegment::LineTo(Point::new(0.0, 0.0)));
    }

    #[test]
    fn test_close_no_extra_line_when_at_start() {
        let path = IconPath::parse("M5 5 L10 10 L5 5 Z").unwrap();
        let sp = &path.subpaths[0];
        // Already at start, so no extra LineTo
        assert_eq!(
            sp.segments.last().unwrap(),
            &PathSegment::Close
        );
    }

    // --- Multiple subpaths ---

    #[test]
    fn test_parse_multiple_subpaths() {
        let path = IconPath::parse("M0 0 L10 10 M20 20 L30 30").unwrap();
        assert_eq!(path.subpaths.len(), 2);
        assert_eq!(
            path.subpaths[0].segments[0],
            PathSegment::MoveTo(Point::new(0.0, 0.0))
        );
        assert_eq!(
            path.subpaths[1].segments[0],
            PathSegment::MoveTo(Point::new(20.0, 20.0))
        );
    }

    #[test]
    fn test_parse_many_subpaths() {
        let path = IconPath::parse("M4 6h16M4 12h16M4 18h16").unwrap();
        assert_eq!(path.subpaths.len(), 3);
    }

    // --- Comma and whitespace handling ---

    #[test]
    fn test_parse_commas_as_separators() {
        let path = IconPath::parse("M10,20 L30,40").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(10.0, 20.0)));
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(30.0, 40.0)));
    }

    #[test]
    fn test_parse_mixed_separators() {
        let path = IconPath::parse("M 10 , 20  L  30  40").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(10.0, 20.0)));
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(30.0, 40.0)));
    }

    #[test]
    fn test_parse_no_spaces_between_commands() {
        let path = IconPath::parse("M0 0L10 10L20 0").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments.len(), 3);
    }

    // --- Negative numbers ---

    #[test]
    fn test_parse_negative_coordinates() {
        let path = IconPath::parse("M-5-10 L-15-20").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(-5.0, -10.0)));
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(-15.0, -20.0)));
    }

    #[test]
    fn test_parse_floating_point() {
        let path = IconPath::parse("M1.5 2.7 L3.14 4.99").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(1.5, 2.7)));
        if let PathSegment::LineTo(p) = &sp.segments[1] {
            assert!((p.x - 3.14).abs() < 1e-5);
            assert!((p.y - 4.99).abs() < 1e-5);
        }
    }

    #[test]
    fn test_parse_scientific_notation() {
        let path = IconPath::parse("M1e2 2E1 L3e-1 4E-2").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(100.0, 20.0)));
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(0.3, 0.04)));
    }

    // --- Error cases ---

    #[test]
    fn test_parse_empty_string() {
        let path = IconPath::parse("");
        assert!(path.is_ok());
        assert_eq!(path.unwrap().subpaths.len(), 0);
    }

    #[test]
    fn test_parse_whitespace_only() {
        let path = IconPath::parse("   ");
        assert!(path.is_ok());
        assert_eq!(path.unwrap().subpaths.len(), 0);
    }

    #[test]
    fn test_parse_invalid_command() {
        let path = IconPath::parse("X10 20");
        assert!(path.is_err());
    }

    #[test]
    fn test_parse_missing_number() {
        let path = IconPath::parse("M10");
        assert!(path.is_err());
    }

    #[test]
    fn test_parse_invalid_flag_in_arc() {
        let path = IconPath::parse("M0 0 A5 5 0 2 1 10 10");
        assert!(path.is_err());
    }

    // --- scale() ---

    #[test]
    fn test_scale() {
        let path = IconPath::parse("M10 10 L20 20").unwrap();
        let scaled = path.scale(2.0);
        let sp = &scaled.subpaths[0];
        assert_eq!(sp.segments[0], PathSegment::MoveTo(Point::new(20.0, 20.0)));
        assert_eq!(sp.segments[1], PathSegment::LineTo(Point::new(40.0, 40.0)));
    }

    #[test]
    fn test_scale_with_cubic() {
        let path = IconPath::parse("M0 0 C10 10 20 20 30 30").unwrap();
        let scaled = path.scale(0.5);
        let sp = &scaled.subpaths[0];
        assert_eq!(
            sp.segments[1],
            PathSegment::CubicTo {
                ctrl1: Point::new(5.0, 5.0),
                ctrl2: Point::new(10.0, 10.0),
                end: Point::new(15.0, 15.0),
            }
        );
    }

    #[test]
    fn test_scale_preserves_close() {
        let path = IconPath::parse("M0 0 L10 0 L10 10 Z").unwrap();
        let scaled = path.scale(3.0);
        assert!(scaled.subpaths[0].is_closed);
        assert!(scaled.subpaths[0].segments.iter().any(|s| *s == PathSegment::Close));
    }

    // --- SubPath ---

    #[test]
    fn test_subpath_start_point() {
        let path = IconPath::parse("M5 10 L20 20").unwrap();
        assert_eq!(path.subpaths[0].start_point(), Some(Point::new(5.0, 10.0)));
    }

    #[test]
    fn test_subpath_start_point_empty() {
        let sp = SubPath::new();
        assert_eq!(sp.start_point(), None);
    }

    // --- Complex real-world paths ---

    #[test]
    fn test_parse_lucide_home_icon() {
        let path = IconPath::parse("M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z");
        assert!(path.is_ok());
        let p = path.unwrap();
        assert!(!p.subpaths.is_empty());
    }

    #[test]
    fn test_parse_lucide_x_icon() {
        let path = IconPath::parse("M18 6L6 18M6 6l12 12");
        assert!(path.is_ok());
        let p = path.unwrap();
        assert_eq!(p.subpaths.len(), 2);
    }

    #[test]
    fn test_parse_complex_path_with_all_commands() {
        // A path using M, L, H, V, C, S, Q, T, A, Z
        let d = "M10 10 L20 20 H30 V40 C35 35 40 40 45 45 S55 55 60 60 Q65 65 70 70 T80 80 A5 5 0 0 1 90 90 Z";
        let path = IconPath::parse(d);
        assert!(path.is_ok());
        let p = path.unwrap();
        assert_eq!(p.subpaths.len(), 1);
        assert!(p.subpaths[0].is_closed);
    }

    // --- Repeated commands ---

    #[test]
    fn test_parse_repeated_lineto() {
        let path = IconPath::parse("M0 0 L10 10 20 20 30 30").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments.len(), 4); // M + 3 L
        assert_eq!(sp.segments[3], PathSegment::LineTo(Point::new(30.0, 30.0)));
    }

    #[test]
    fn test_parse_repeated_cubic() {
        let path = IconPath::parse("M0 0 C1 1 2 2 3 3 4 4 5 5 6 6").unwrap();
        let sp = &path.subpaths[0];
        assert_eq!(sp.segments.len(), 3); // M + 2 C
    }
}
