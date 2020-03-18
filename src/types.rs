use oculussdk_sys::{ovrInputState_, ovrPoseStatef, ovrPoseStatef_};
use smallvec::{smallvec, SmallVec};
use std::mem::transmute;

pub struct PoseState {
    /// x, y, z, w
    pub rotation: [f32; 4],
    pub translation: [f32; 3],
    pub angular_velocity: [f32; 3],
    pub linear_velocity: [f32; 3],
    pub angular_acceleration: [f32; 3],
    pub linear_acceleration: [f32; 3],
}

impl From<ovrPoseStatef> for PoseState {
    fn from(state: ovrPoseStatef) -> Self {
        unsafe {
            Self {
                rotation: transmute(state.ThePose.Orientation),
                translation: transmute(state.ThePose.Position),
                angular_velocity: transmute(state.AngularVelocity),
                linear_velocity: transmute(state.LinearVelocity),
                angular_acceleration: transmute(state.AngularAcceleration),
                linear_acceleration: transmute(state.LinearAcceleration),
            }
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Button {
    A = 1,
    B = 2,
    RThumb = 4,
    RThumbRest = 8,
    RIndexTrigger = 16,
    X = 256,
    Y = 512,
    LThumb = 1024,
    LThumbRest = 2048,
    LIndexTrigger = 4096,
    RIndexPointing = 32,
    RThumbUp = 64,
    LIndexPointing = 8192,
    LThumbUp = 16384,
    Menu = 1048576,
    Home = 16777216,
}

#[derive(Default, Debug, Clone)]
pub struct InputState {
    pub pressed: SmallVec<[Button; 16]>,
    pub touched: SmallVec<[Button; 16]>,
    pub index_trigger: [f32; 2],
    pub hand_trigger: [f32; 2],
    pub thumbstick: [[f32; 2]; 2],
}

impl From<ovrInputState_> for InputState {
    fn from(raw: ovrInputState_) -> Self {
        let pressed = (0..24)
            .filter_map(|i| {
                if (raw.Buttons >> i) & 0b1 == 1 {
                    Some(unsafe { transmute((1 << i)) })
                } else {
                    None
                }
            })
            .collect();
        let touched = (0..24)
            .filter_map(|i| {
                if (raw.Touches >> i) & 0b1 == 1 {
                    Some(unsafe { transmute((1 << i)) })
                } else {
                    None
                }
            })
            .collect();

        Self {
            pressed,
            touched,
            index_trigger: raw.IndexTrigger,
            hand_trigger: raw.HandTrigger,
            thumbstick: unsafe { std::mem::transmute(raw.Thumbstick) },
        }
    }
}
