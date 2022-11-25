use crate::steps_generator::StepError;

#[derive(Debug, PartialEq, Clone)]
pub enum Mark {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    ForeColor(Color),
    BackColor(Color),
}

impl Mark {
    pub fn from_str(mark: &str) -> Result<Self, StepError> {
        match mark {
            "bold" => Ok(Mark::Bold),
            "italic" => Ok(Mark::Italic),
            "underline" => Ok(Mark::Underline),
            "strikethrough" => Ok(Mark::Strikethrough),
            mark if mark.contains("fore_color") | mark.contains("back_color") => Mark::color_mark_from_str(mark),
            _ => Err(StepError(format!("Invalid Mark: {}", mark)))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Mark::Bold => "bold".to_string(),
            Mark::Italic => "italic".to_string(),
            Mark::Underline => "underline".to_string(),
            Mark::Strikethrough => "strikethrough".to_string(),
            Mark::ForeColor(color) => format!("fore_color{}", color.to_string()),
            Mark::BackColor(color) => format!("back_color{}", color.to_string()),
        }
    }

    /// eg: "fore_color(0, 0, 0, 1) || back_color(0, 0, 0, 1)"
    pub fn color_mark_from_str(mark: &str) -> Result<Mark, StepError> {
        let color = mark.split("(").last().ok_or(StepError(format!("Invalid Mark: {}", mark)))?;
        let color = color.split(")").next().ok_or(StepError(format!("Invalid Mark: {}", mark)))?;
        let color = color.split(",").collect::<Vec<&str>>();
        let color = color.iter().map(|c| {
            let c = c.trim();
            c.parse::<u8>().map_err(|_| StepError(format!("Invalid Mark: {}", mark)))
        }).collect::<Result<Vec<u8>, StepError>>()?;
        if color.len() != 4 {
            return Err(StepError(format!("Color should have 4 numbers. Got: {}", mark)))
        }
        let color = Color::from(color[0], color[1], color[2], color[3]);
        if mark.contains("fore_color") {
            Ok(Mark::ForeColor(color))
        } else if mark.contains("back_color") {
            Ok(Mark::BackColor(color))
        } else {
            Err(StepError(format!("Invalid Mark: {}", mark)))
        }
    }


    pub fn is_same_type(&self, other_mark: &Mark) -> bool {
        match self {
            Mark::ForeColor(_) => match other_mark {
                Mark::ForeColor(_) => true,
                _ => false
            },
            Mark::BackColor(_) => match other_mark {
                Mark::BackColor(_) => true,
                _ => false
            },
            _ => self == other_mark
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub fn from(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    pub fn to_string(&self) -> String {
        format!("({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}