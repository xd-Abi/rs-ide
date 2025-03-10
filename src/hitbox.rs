use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::window::{CursorIcon, ResizeDirection};

#[derive(Debug, Clone, Copy)]
pub enum HitBox {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    North,
    South,
    West,
    East,
    TitleBar,
    None,
}

impl HitBox {
    pub fn from_position(
        position: PhysicalPosition<f64>,
        size: LogicalSize<f64>,
        border_threshold: f64,
        title_bar_thickness: f64,
    ) -> Self {
        if position.x < border_threshold && position.y < border_threshold {
            HitBox::NorthWest
        } else if position.x > size.width - border_threshold && position.y < border_threshold {
            HitBox::NorthEast
        } else if position.x < border_threshold && position.y > size.height - border_threshold {
            HitBox::SouthWest
        } else if position.x > size.width - border_threshold
            && position.y > size.height - border_threshold
        {
            HitBox::SouthEast
        } else if position.x < border_threshold {
            HitBox::West
        } else if position.x > size.width - border_threshold {
            HitBox::East
        } else if position.y < border_threshold {
            HitBox::North
        } else if position.y > size.height - border_threshold {
            HitBox::South
        } else if position.y < title_bar_thickness {
            HitBox::TitleBar
        } else {
            HitBox::None
        }
    }
}

impl Into<CursorIcon> for HitBox {
    fn into(self) -> CursorIcon {
        match self {
            HitBox::NorthWest => CursorIcon::NwResize,
            HitBox::NorthEast => CursorIcon::NeResize,
            HitBox::SouthWest => CursorIcon::SwResize,
            HitBox::SouthEast => CursorIcon::SeResize,
            HitBox::West => CursorIcon::WResize,
            HitBox::East => CursorIcon::EResize,
            HitBox::North => CursorIcon::NResize,
            HitBox::South => CursorIcon::SResize,
            HitBox::TitleBar => CursorIcon::Default,
            HitBox::None => CursorIcon::Default,
        }
    }
}

impl Into<Option<ResizeDirection>> for HitBox {
    fn into(self) -> Option<ResizeDirection> {
        match self {
            HitBox::NorthWest => Some(ResizeDirection::NorthWest),
            HitBox::NorthEast => Some(ResizeDirection::NorthEast),
            HitBox::SouthWest => Some(ResizeDirection::SouthWest),
            HitBox::SouthEast => Some(ResizeDirection::SouthEast),
            HitBox::West => Some(ResizeDirection::West),
            HitBox::East => Some(ResizeDirection::East),
            HitBox::North => Some(ResizeDirection::North),
            HitBox::South => Some(ResizeDirection::South),
            _ => None,
        }
    }
}
