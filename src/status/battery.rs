use crate::status::StatusIndicator;
use crate::utils::rounded_rect;
use cairo::{Context, LineCap, LineJoin, Operator};
use core::f64::consts::PI;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

const CHECK_INTERVAL: u64 = 5;

#[derive(Debug)]
pub struct BatteryIndicator {
    battery_state: Result<BatteryState, io::Error>,
    prev_bat_check: Instant,
}

impl BatteryIndicator {
    pub fn new() -> BatteryIndicator {
        BatteryIndicator {
            battery_state: battery_status(),
            prev_bat_check: Instant::now(),
        }
    }
}

impl StatusIndicator for BatteryIndicator {
    fn draw(&mut self, ctx: &Context, right_inset: &mut f64, height: f64) {
        if self.prev_bat_check.elapsed().as_secs() > CHECK_INTERVAL {
            self.battery_state = battery_status();
            self.prev_bat_check = Instant::now();
        }

        let font_extents = ctx.font_extents();
        let text_y = height / 2. - font_extents.descent + font_extents.height / 2.;

        *right_inset -= 25.;
        ctx.save();
        ctx.set_line_join(LineJoin::Round);
        ctx.set_line_cap(LineCap::Round);
        ctx.set_line_width(1.);
        ctx.set_source_rgba(1., 1., 1., 0.5);
        ctx.translate(*right_inset, height / 2. - 6.);
        rounded_rect(ctx, 0.5, 0.5, 21., 11., 2.5);
        ctx.stroke();
        ctx.move_to(23., 4.);
        ctx.arc(23., 6., 2., -PI / 2., PI / 2.);
        ctx.fill();

        match self.battery_state {
            Ok(status) if !status.error => {
                let width = (18. * (status.percentage as f64 / 100.)).max(2.);
                if status.charging {
                    ctx.set_source_rgba(0.3, 0.74, 0.42, 1.);
                } else if status.critical {
                    ctx.set_source_rgba(0.95, 0.26, 0.21, 1.);
                } else {
                    ctx.set_source_rgba(1., 1., 1., 1.);
                }
                rounded_rect(ctx, 2., 2., width, 8., 1.);
                ctx.fill();

                macro_rules! lightning_icon {
                    () => {
                        ctx.move_to(11.6, 0.5);
                        ctx.line_to(8., 7.);
                        ctx.line_to(11., 7.);
                        ctx.line_to(10.5, 11.5);
                        ctx.line_to(14., 5.);
                        ctx.line_to(11., 5.);
                        ctx.close_path();
                    };
                }

                if status.charging {
                    ctx.set_source_rgba(1., 1., 1., 1.);
                    ctx.set_operator(Operator::Clear);
                    ctx.set_line_width(2.);
                    lightning_icon!();
                    ctx.stroke();
                    ctx.set_operator(Operator::Over);
                    lightning_icon!();
                    ctx.fill();
                }
            }
            _ => {
                ctx.set_source_rgba(1., 1., 1., 1.);
                ctx.move_to(9.5, 9.);
                ctx.line_to(7., 9.);
                ctx.line_to(11., 2.);
                ctx.line_to(15., 9.);
                ctx.line_to(12.5, 9.);
                ctx.stroke();
                ctx.move_to(11., 5.);
                ctx.line_to(11., 8.);
                ctx.stroke();
                ctx.arc(11., 9.5, 0.5, 0., PI * 2.);
                ctx.fill();
            }
        }
        ctx.restore();

        let battery_text = match self.battery_state {
            Ok(status) => format!("{}%", status.percentage),
            Err(_) => String::from(""),
        };
        let text_extents = ctx.text_extents(&battery_text);
        *right_inset -= 5. + text_extents.width;
        ctx.move_to(*right_inset, text_y);
        ctx.show_text(&battery_text);
    }
}

const FAKEDEV_PATH: &str = "/etc/fakedev/power_supply/BAT1/uevent";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryState {
    pub percentage: u8,
    pub charging: bool,
    pub error: bool,
    pub critical: bool,
}

pub fn battery_status() -> Result<BatteryState, io::Error> {
    let mut file = File::open(FAKEDEV_PATH)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut values = HashMap::new();

    for line in content.lines() {
        let mut parts = line.split("=");
        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
            values.insert(k, v);
        }
    }

    let percentage: u8 = match values.get("POWER_SUPPLY_CAPACITY") {
        Some(v) => v,
        None => return Err(io::ErrorKind::InvalidData.into()),
    }
    .parse()
    .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

    let charging = values.get("POWER_SUPPLY_STATUS") == Some(&"Charging")
        || values.get("POWER_SUPPLY_STATUS") == Some(&"Critical (Charging)");
    let error = values.get("POWER_SUPPLY_STATUS") == Some(&"None");
    let critical = values
        .get("POWER_SUPPLY_STATUS")
        .map(|s| s.starts_with("Critical"))
        .unwrap_or(false);

    Ok(BatteryState {
        percentage,
        charging,
        error,
        critical,
    })
}
