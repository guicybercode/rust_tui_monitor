use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub background: [u8; 3],
    pub foreground: [u8; 3],
    pub primary: [u8; 3],
    pub secondary: [u8; 3],
    pub success: [u8; 3],
    pub warning: [u8; 3],
    pub danger: [u8; 3],
    pub border: [u8; 3],
}

impl Theme {
    pub fn default() -> Self {
        Self {
            background: [26, 27, 38],
            foreground: [198, 208, 245],
            primary: [137, 180, 250],
            secondary: [203, 166, 247],
            success: [166, 227, 161],
            warning: [249, 226, 175],
            danger: [243, 139, 168],
            border: [88, 91, 112],
        }
    }

    pub fn nord() -> Self {
        Self {
            background: [46, 52, 64],
            foreground: [236, 239, 244],
            primary: [136, 192, 208],
            secondary: [129, 161, 193],
            success: [163, 190, 140],
            warning: [235, 203, 139],
            danger: [191, 97, 106],
            border: [67, 76, 94],
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            background: [40, 40, 40],
            foreground: [235, 219, 178],
            primary: [131, 165, 152],
            secondary: [211, 134, 155],
            success: [184, 187, 38],
            warning: [250, 189, 47],
            danger: [251, 73, 52],
            border: [102, 92, 84],
        }
    }

    pub fn dracula() -> Self {
        Self {
            background: [40, 42, 54],
            foreground: [248, 248, 242],
            primary: [139, 233, 253],
            secondary: [189, 147, 249],
            success: [80, 250, 123],
            warning: [241, 250, 140],
            danger: [255, 85, 85],
            border: [68, 71, 90],
        }
    }

    pub fn monokai() -> Self {
        Self {
            background: [39, 40, 34],
            foreground: [248, 248, 240],
            primary: [102, 217, 239],
            secondary: [174, 129, 255],
            success: [166, 226, 46],
            warning: [244, 191, 117],
            danger: [249, 38, 114],
            border: [73, 72, 62],
        }
    }

    pub fn cyberpunk() -> Self {
        Self {
            background: [16, 18, 27],
            foreground: [255, 0, 255],
            primary: [0, 255, 255],
            secondary: [255, 0, 128],
            success: [0, 255, 0],
            warning: [255, 255, 0],
            danger: [255, 0, 100],
            border: [138, 43, 226],
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            background: [26, 27, 38],
            foreground: [192, 202, 245],
            primary: [122, 162, 247],
            secondary: [187, 154, 247],
            success: [158, 206, 106],
            warning: [224, 175, 104],
            danger: [247, 118, 142],
            border: [65, 72, 104],
        }
    }

    pub fn solarized_dark() -> Self {
        Self {
            background: [0, 43, 54],
            foreground: [131, 148, 150],
            primary: [38, 139, 210],
            secondary: [108, 113, 196],
            success: [133, 153, 0],
            warning: [181, 137, 0],
            danger: [220, 50, 47],
            border: [7, 54, 66],
        }
    }

    pub fn solarized_light() -> Self {
        Self {
            background: [253, 246, 227],
            foreground: [101, 123, 131],
            primary: [38, 139, 210],
            secondary: [108, 113, 196],
            success: [133, 153, 0],
            warning: [181, 137, 0],
            danger: [220, 50, 47],
            border: [238, 232, 213],
        }
    }

    pub fn one_dark() -> Self {
        Self {
            background: [40, 44, 52],
            foreground: [171, 178, 191],
            primary: [97, 175, 239],
            secondary: [198, 120, 221],
            success: [152, 195, 121],
            warning: [229, 192, 123],
            danger: [224, 108, 117],
            border: [76, 82, 99],
        }
    }

    pub fn material() -> Self {
        Self {
            background: [38, 50, 56],
            foreground: [236, 239, 241],
            primary: [128, 203, 196],
            secondary: [255, 202, 40],
            success: [195, 232, 141],
            warning: [255, 213, 79],
            danger: [239, 83, 80],
            border: [69, 90, 100],
        }
    }

    pub fn ayu_dark() -> Self {
        Self {
            background: [10, 14, 20],
            foreground: [230, 237, 243],
            primary: [89, 181, 249],
            secondary: [223, 142, 255],
            success: [186, 230, 126],
            warning: [255, 180, 84],
            danger: [242, 119, 122],
            border: [15, 20, 25],
        }
    }

    pub fn rosepine() -> Self {
        Self {
            background: [25, 23, 36],
            foreground: [224, 222, 244],
            primary: [156, 207, 216],
            secondary: [196, 167, 231],
            success: [49, 116, 143],
            warning: [246, 193, 119],
            danger: [235, 111, 146],
            border: [33, 32, 46],
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "nord" => Self::nord(),
            "gruvbox" => Self::gruvbox(),
            "dracula" => Self::dracula(),
            "monokai" => Self::monokai(),
            "cyberpunk" => Self::cyberpunk(),
            "tokyo-night" | "tokyo_night" => Self::tokyo_night(),
            "solarized-dark" | "solarized_dark" => Self::solarized_dark(),
            "solarized-light" | "solarized_light" => Self::solarized_light(),
            "one-dark" | "one_dark" => Self::one_dark(),
            "material" => Self::material(),
            "ayu-dark" | "ayu_dark" => Self::ayu_dark(),
            "rosepine" | "rose-pine" | "rose_pine" => Self::rosepine(),
            _ => Self::default(),
        }
    }

    pub fn bg(&self) -> Color {
        Color::Rgb(self.background[0], self.background[1], self.background[2])
    }

    pub fn fg(&self) -> Color {
        Color::Rgb(self.foreground[0], self.foreground[1], self.foreground[2])
    }

    pub fn primary(&self) -> Color {
        Color::Rgb(self.primary[0], self.primary[1], self.primary[2])
    }

    pub fn secondary(&self) -> Color {
        Color::Rgb(self.secondary[0], self.secondary[1], self.secondary[2])
    }

    pub fn success(&self) -> Color {
        Color::Rgb(self.success[0], self.success[1], self.success[2])
    }

    pub fn warning(&self) -> Color {
        Color::Rgb(self.warning[0], self.warning[1], self.warning[2])
    }

    pub fn danger(&self) -> Color {
        Color::Rgb(self.danger[0], self.danger[1], self.danger[2])
    }

    pub fn border(&self) -> Color {
        Color::Rgb(self.border[0], self.border[1], self.border[2])
    }

    pub fn usage_color(&self, percent: f32) -> Color {
        if percent < 50.0 {
            self.success()
        } else if percent < 80.0 {
            self.warning()
        } else {
            self.danger()
        }
    }
}

