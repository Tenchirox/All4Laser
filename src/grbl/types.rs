use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacStatus {
    Disconnected,
    Connecting,
    Idle,
    Run,
    Hold,
    Jog,
    Alarm,
    Door,
    Check,
    Home,
    Sleep,
}

impl fmt::Display for MacStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disconnected => write!(f, "Disconnected"),
            Self::Connecting => write!(f, "Connecting"),
            Self::Idle => write!(f, "Idle"),
            Self::Run => write!(f, "Run"),
            Self::Hold => write!(f, "Hold"),
            Self::Jog => write!(f, "Jog"),
            Self::Alarm => write!(f, "Alarm"),
            Self::Door => write!(f, "Door"),
            Self::Check => write!(f, "Check"),
            Self::Home => write!(f, "Home"),
            Self::Sleep => write!(f, "Sleep"),
        }
    }
}

impl MacStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Idle" => Self::Idle,
            "Run" => Self::Run,
            "Hold" | "Hold:0" | "Hold:1" => Self::Hold,
            "Jog" => Self::Jog,
            "Alarm" => Self::Alarm,
            "Door" | "Door:0" | "Door:1" | "Door:2" | "Door:3" => Self::Door,
            "Check" => Self::Check,
            "Home" => Self::Home,
            "Sleep" => Self::Sleep,
            _ => Self::Disconnected,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GPoint {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self::default()
    }
}

impl std::ops::Sub for GPoint {
    type Output = GPoint;
    fn sub(self, rhs: Self) -> Self::Output {
        GPoint::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JogDirection {
    N, S, E, W, NE, NW, SE, SW,
    Zup, Zdown, Home,
}

#[derive(Debug, Clone)]
pub struct GrblState {
    pub status: MacStatus,
    pub mpos: GPoint,
    pub wpos: GPoint,
    pub wco: GPoint,
    pub feed_rate: f32,
    pub spindle_speed: f32,
    pub override_feed: i32,
    pub override_rapid: i32,
    pub override_spindle: i32,
    pub buffer_plan: i32,
    pub buffer_rx: i32,
}

impl Default for GrblState {
    fn default() -> Self {
        Self {
            status: MacStatus::Disconnected,
            mpos: GPoint::zero(),
            wpos: GPoint::zero(),
            wco: GPoint::zero(),
            feed_rate: 0.0,
            spindle_speed: 0.0,
            override_feed: 100,
            override_rapid: 100,
            override_spindle: 100,
            buffer_plan: 0,
            buffer_rx: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GrblResponse {
    Ok,
    Error(i32),
    Alarm(i32),
    Status(GrblState),
    GrblVersion(String),
    Message(String),
    Setting(i32, String),
}
