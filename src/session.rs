use crate::InputState;
use oculussdk_sys::{ovrControllerType__ovrControllerType_Touch, ovrEyeType__ovrEye_Left, ovrEyeType__ovrEye_Right, ovrLayerHeader_, ovrLayerType__ovrLayerType_EyeFov, ovrLayer_Union_, ovrMatrix4f_Projection, ovrRecti, ovrTextureSwapChainDesc_, ovrTrackingOrigin__ovrTrackingOrigin_FloorLevel, ovr_BeginFrame, ovr_CommitTextureSwapChain, ovr_Create, ovr_CreateTextureSwapChainVk, ovr_Destroy, ovr_DestroyTextureSwapChain, ovr_EndFrame, ovr_GetEyePoses, ovr_GetFovTextureSize, ovr_GetHmdDesc, ovr_GetInputState, ovr_GetPredictedDisplayTime, ovr_GetRenderDesc, ovr_GetSessionPhysicalDeviceVk, ovr_GetTextureSwapChainBufferVk, ovr_GetTextureSwapChainCurrentIndex, ovr_GetTextureSwapChainLength, ovr_GetTrackingState, ovr_Initialize, ovr_SetSynchronizationQueueVk, ovr_SetTrackingOriginType, ovr_WaitToBeginFrame, ovr_GetInstanceExtensionsVk, ovr_GetDeviceExtensionsVk};
use {
    crate::{Error, PoseState, Result},
    libc::c_char,
    oculussdk_sys::{
        ovrFovPort, ovrGraphicsLuid, ovrInitFlags, ovrInitParams, ovrLayerEyeFov, ovrLayerHeader,
        ovrLayerType, ovrPosef, ovrQuatf, ovrSession, ovrSizei, ovrTextureSwapChain,
        ovrTextureSwapChainDesc, ovrVector2i, ovrVector3f,
    },
    std::{
        ffi::{CStr, CString},
        mem::{transmute, MaybeUninit},
    },
};

fn check_error(er: i32) -> Result<()> {
    if er == 0 {
        Ok(())
    } else {
        Err(er.into())
    }
}
#[derive(Clone)]
pub struct Session {
    raw: ovrSession,
    luid: ovrGraphicsLuid,
}
unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Session {
    pub fn initialize(
        user_data: Option<usize>,
        callback: Option<unsafe extern "C" fn(usize, i32, *const c_char) -> ()>,
    ) -> Result<Self> {
        let params: ovrInitParams = ovrInitParams {
            Flags: 32,
            RequestedMinorVersion: 0,
            LogCallback: callback,
            UserData: user_data.unwrap_or(0),
            ConnectionTimeoutMS: 0,
            pad0: [0, 0, 0, 0],
        };

        unsafe { check_error(ovr_Initialize(&params))? };

        let mut raw = unsafe { MaybeUninit::zeroed() };
        let mut luid = unsafe { MaybeUninit::zeroed() };

        unsafe { check_error(ovr_Create(raw.as_mut_ptr(), luid.as_mut_ptr()))? }

        let (raw, luid) = unsafe { (raw.assume_init(), luid.assume_init()) };

        Ok(Self { raw, luid })
    }

    pub fn create_swapchain(
        &self,
        device: &ash::Device,
        format: ash::vk::Format,
        width: u32,
        height: u32,
        layers: u32,
        mips: u32,
        samples: ash::vk::SampleCountFlags,
    ) -> Result<(Swapchain, Vec<ash::vk::Image>)> {
        use oculussdk_sys::{
            ovrTextureBindFlags__ovrTextureBind_DX_UnorderedAccess,
            ovrTextureFormat__OVR_FORMAT_B4G4R4A4_UNORM,
            ovrTextureFormat__OVR_FORMAT_B5G5R5A1_UNORM, ovrTextureFormat__OVR_FORMAT_B5G6R5_UNORM,
            ovrTextureFormat__OVR_FORMAT_B8G8R8A8_UNORM,
            ovrTextureFormat__OVR_FORMAT_B8G8R8A8_UNORM_SRGB,
            ovrTextureFormat__OVR_FORMAT_B8G8R8X8_UNORM,
            ovrTextureFormat__OVR_FORMAT_B8G8R8X8_UNORM_SRGB,
            ovrTextureFormat__OVR_FORMAT_B8G8R8_UNORM, ovrTextureFormat__OVR_FORMAT_BC1_UNORM,
            ovrTextureFormat__OVR_FORMAT_BC1_UNORM_SRGB, ovrTextureFormat__OVR_FORMAT_BC2_UNORM,
            ovrTextureFormat__OVR_FORMAT_BC2_UNORM_SRGB, ovrTextureFormat__OVR_FORMAT_BC3_UNORM,
            ovrTextureFormat__OVR_FORMAT_BC3_UNORM_SRGB, ovrTextureFormat__OVR_FORMAT_BC6H_SF16,
            ovrTextureFormat__OVR_FORMAT_BC6H_UF16, ovrTextureFormat__OVR_FORMAT_BC7_UNORM,
            ovrTextureFormat__OVR_FORMAT_BC7_UNORM_SRGB, ovrTextureFormat__OVR_FORMAT_D16_UNORM,
            ovrTextureFormat__OVR_FORMAT_D24_UNORM_S8_UINT, ovrTextureFormat__OVR_FORMAT_D32_FLOAT,
            ovrTextureFormat__OVR_FORMAT_D32_FLOAT_S8X24_UINT, ovrTextureFormat__OVR_FORMAT_LAST,
            ovrTextureFormat__OVR_FORMAT_R11G11B10_FLOAT,
            ovrTextureFormat__OVR_FORMAT_R16G16B16A16_FLOAT,
            ovrTextureFormat__OVR_FORMAT_R8G8B8A8_UNORM,
            ovrTextureFormat__OVR_FORMAT_R8G8B8A8_UNORM_SRGB,
            ovrTextureMiscFlags__ovrTextureMisc_None, ovrTextureType__ovrTexture_2D,
        };

        let samples = match samples {
            ash::vk::SampleCountFlags::TYPE_1 => 1,
            ash::vk::SampleCountFlags::TYPE_2 => 2,
            ash::vk::SampleCountFlags::TYPE_4 => 4,
            ash::vk::SampleCountFlags::TYPE_8 => 8,
            ash::vk::SampleCountFlags::TYPE_16 => 16,
            ash::vk::SampleCountFlags::TYPE_32 => 32,
            ash::vk::SampleCountFlags::TYPE_64 => 64,
            _ => unreachable!(),
        };

        let format = match format {
            ash::vk::Format::R8G8B8A8_UNORM => ovrTextureFormat__OVR_FORMAT_R8G8B8A8_UNORM,
            ash::vk::Format::R8G8B8A8_SRGB => ovrTextureFormat__OVR_FORMAT_R8G8B8A8_UNORM_SRGB,
            ash::vk::Format::B8G8R8A8_UNORM => ovrTextureFormat__OVR_FORMAT_B8G8R8A8_UNORM,
            ash::vk::Format::B8G8R8A8_SRGB => ovrTextureFormat__OVR_FORMAT_B8G8R8A8_UNORM_SRGB,
            ash::vk::Format::B8G8R8_UNORM => ovrTextureFormat__OVR_FORMAT_B8G8R8_UNORM,
            ash::vk::Format::R16G16B16A16_SFLOAT => ovrTextureFormat__OVR_FORMAT_R16G16B16A16_FLOAT,
            ash::vk::Format::D16_UNORM => ovrTextureFormat__OVR_FORMAT_D16_UNORM,
            ash::vk::Format::D24_UNORM_S8_UINT => ovrTextureFormat__OVR_FORMAT_D24_UNORM_S8_UINT,
            ash::vk::Format::D32_SFLOAT => ovrTextureFormat__OVR_FORMAT_D32_FLOAT,
            ash::vk::Format::D24_UNORM_S8_UINT => ovrTextureFormat__OVR_FORMAT_D32_FLOAT_S8X24_UINT,
            ash::vk::Format::BC1_RGBA_UNORM_BLOCK => ovrTextureFormat__OVR_FORMAT_BC1_UNORM,
            // ash::vk::Format::BC1_UNORM_SRGB => ovrTextureFormat__OVR_FORMAT_BC1_UNORM_SRGB,
            ash::vk::Format::BC2_UNORM_BLOCK => ovrTextureFormat__OVR_FORMAT_BC2_UNORM,
            ash::vk::Format::BC2_SRGB_BLOCK => ovrTextureFormat__OVR_FORMAT_BC2_UNORM_SRGB,
            ash::vk::Format::BC3_UNORM_BLOCK => ovrTextureFormat__OVR_FORMAT_BC3_UNORM,
            ash::vk::Format::BC3_SRGB_BLOCK => ovrTextureFormat__OVR_FORMAT_BC3_UNORM_SRGB,
            ash::vk::Format::BC6H_UFLOAT_BLOCK => ovrTextureFormat__OVR_FORMAT_BC6H_UF16,
            ash::vk::Format::BC6H_SFLOAT_BLOCK => ovrTextureFormat__OVR_FORMAT_BC6H_SF16,
            ash::vk::Format::BC7_UNORM_BLOCK => ovrTextureFormat__OVR_FORMAT_BC7_UNORM,
            ash::vk::Format::BC7_SRGB_BLOCK => ovrTextureFormat__OVR_FORMAT_BC7_UNORM_SRGB,
            _ => panic!("Unsupported format"),
        };

        let description = ovrTextureSwapChainDesc {
            Type: ovrTextureType__ovrTexture_2D,
            Format: format,
            ArraySize: layers as _,
            Width: width as _,
            Height: height as _,
            MipLevels: mips as _,
            SampleCount: samples as _,
            StaticImage: 0, // ovrFalse
            MiscFlags: ovrTextureMiscFlags__ovrTextureMisc_None as _,
            BindFlags: ovrTextureBindFlags__ovrTextureBind_DX_UnorderedAccess as _,
        };

        let mut swapchain = MaybeUninit::uninit();
        unsafe {
            use ash::version::DeviceV1_0;
            check_error(ovr_CreateTextureSwapChainVk(
                self.raw,
                transmute(device.handle()),
                &description,
                swapchain.as_mut_ptr(),
            ))?;
        }
        let swapchain = unsafe { swapchain.assume_init() };

        let mut swapchain_images = 0;

        unsafe {
            check_error(ovr_GetTextureSwapChainLength(
                self.raw,
                swapchain,
                &mut swapchain_images,
            ))?;
        }

        let mut images = Vec::with_capacity(swapchain_images as usize);
        for i in 0..swapchain_images as usize {
            let mut image = unsafe { MaybeUninit::uninit() };

            unsafe {
                check_error(ovr_GetTextureSwapChainBufferVk(
                    self.raw,
                    swapchain,
                    i as _,
                    image.as_mut_ptr(),
                ))?;
            }

            images.push(unsafe { transmute(image.assume_init()) });
        }

        let swapchain = Swapchain::new(swapchain, self, width / 2, height);

        Ok((swapchain, images))
    }

    pub fn get_physical_device(
        &self,
        instance: ash::vk::Instance,
    ) -> Result<ash::vk::PhysicalDevice> {
        let mut out = unsafe { MaybeUninit::uninit() };

        unsafe {
            check_error(ovr_GetSessionPhysicalDeviceVk(
                self.raw,
                self.luid,
                transmute(instance),
                out.as_mut_ptr(),
            ))?;
        }

        Ok(unsafe { transmute(out.assume_init()) })
    }

    pub fn recommended_target_size(&self) -> (u32, u32) {
        let desc = unsafe { ovr_GetHmdDesc(self.raw) };

        let default_fov = desc.DefaultEyeFov[0];
        let size =
            unsafe { ovr_GetFovTextureSize(self.raw, ovrEyeType__ovrEye_Left, default_fov, 1.0) };

        (size.w as u32, size.h as u32)
    }

    pub fn desrtroy(self) {
        unsafe { ovr_Destroy(self.raw) };
    }
    pub fn set_synchronization_queue(&self, queue: ash::vk::Queue) {
        unsafe {
            ovr_SetSynchronizationQueueVk(self.raw, transmute(queue));
        }
    }
    pub fn get_default_fov(&self) -> [ovrFovPort; 2] {
        let desc = unsafe { ovr_GetHmdDesc(self.raw) };
        desc.DefaultEyeFov
    }
    pub fn eye_pose(
        &self,
        frame: u64,
        offsets: ([f32; 3], [f32; 3]),
    ) -> ([([f32; 4], [f32; 3]); 2], f64) {
        let mut eye_poses: [ovrPosef; 2] = unsafe { std::mem::zeroed() };
        let mut time = 0.0;
        let offset: [ovrVector3f; 2] = unsafe { transmute(offsets) };
        unsafe {
            ovr_GetEyePoses(
                self.raw,
                frame as _,
                1,
                offset.as_ptr(),
                eye_poses.as_mut_ptr(),
                &mut time,
            );
        }

        (unsafe { transmute(eye_poses) }, time)
    }
    pub fn predicted_display_time(&self, frame: u64) -> f64 {
        unsafe { ovr_GetPredictedDisplayTime(self.raw, frame as _) }
    }

    pub fn tracking_state(&self, prediction: f64) -> (PoseState, PoseState, PoseState) {
        let state = unsafe { ovr_GetTrackingState(self.raw, prediction, 1) };
        (
            state.HeadPose.into(),
            state.HandPoses[0].into(),
            state.HandPoses[1].into(),
        )
    }

    pub fn eye_transforms(&self) -> (([f32; 4], [f32; 3]), ([f32; 4], [f32; 3])) {
        let fov = self.get_default_fov();
        let left = unsafe { ovr_GetRenderDesc(self.raw, ovrEyeType__ovrEye_Left, fov[0]) };
        let right = unsafe { ovr_GetRenderDesc(self.raw, ovrEyeType__ovrEye_Right, fov[1]) };

        unsafe { (transmute(left.HmdToEyePose), transmute(right.HmdToEyePose)) }
    }

    pub fn projections(&self, near: f32, far: f32) -> ([f32; 16], [f32; 16]) {
        let fov = self.get_default_fov();
        let left = unsafe { ovrMatrix4f_Projection(fov[0], near, far, 0) };
        let right = unsafe { ovrMatrix4f_Projection(fov[1], near, far, 0) };

        unsafe { (transmute(left), transmute(right)) }
    }

    pub fn tracking_origin_floor(&self) {
        unsafe {
            ovr_SetTrackingOriginType(self.raw, ovrTrackingOrigin__ovrTrackingOrigin_FloorLevel);
        }
    }
    pub fn input_state(&self) -> Result<InputState> {
        let mut state = unsafe { std::mem::zeroed() };
        unsafe {
            check_error(ovr_GetInputState(
                self.raw,
                ovrControllerType__ovrControllerType_Touch,
                &mut state,
            ))?;
        }
        Ok(state.into())
    }

    pub fn get_instance_extensions(&self) -> Result<Vec<CString>> {
        let mut len = 1024;
        let mut exts = [0i8; 1024];
        unsafe{
            check_error(
                ovr_GetInstanceExtensionsVk(self.luid, exts.as_mut_ptr(), &mut len)
            )?;
        }
        if len > 1024{
            return Err(Error{
                message: "Instance extensions string is greater than 1024!".to_string(),
                code: -1,
            })
        }
        let exts = unsafe{ CStr::from_ptr(exts.as_ptr()) };
        let mut res = Vec::new();

        for ext in exts.to_str().unwrap().split_ascii_whitespace(){
            res.push(
                CString::new(ext).expect("Bad extension name")
            );
        }
        Ok(res)
    }
    pub fn get_device_extensions(&self) -> Result<Vec<CString>> {
        let mut len = 1024;
        let mut exts = [0i8; 1024];
        unsafe{
            check_error(
                ovr_GetDeviceExtensionsVk(self.luid, exts.as_mut_ptr(), &mut len)
            )?;
        }
        if len > 1024{
            return Err(Error{
                message: "Device extensions string is greater than 1024!".to_string(),
                code: -1,
            })
        }
        let exts = unsafe{ CStr::from_ptr(exts.as_ptr()) };
        let mut res = Vec::new();

        for ext in exts.to_str().unwrap().split_ascii_whitespace(){
            res.push(
                CString::new(ext).expect("Bad extension name")
            );
        }
        Ok(res)
    }
}

pub struct Swapchain {
    raw: ovrTextureSwapChain,
    session: ovrSession,
    layer: ovrLayerEyeFov,
}

impl Swapchain {
    pub fn new(raw: ovrTextureSwapChain, session: &Session, width: u32, height: u32) -> Self {
        let rects = [
            ovrRecti {
                Pos: ovrVector2i { x: 0, y: 0 },
                Size: ovrSizei {
                    w: width as _,
                    h: height as _,
                },
            },
            ovrRecti {
                Pos: ovrVector2i {
                    x: width as _,
                    y: 0,
                },
                Size: ovrSizei {
                    w: width as _,
                    h: height as _,
                },
            },
        ];
        let pose = ovrPosef {
            Orientation: ovrQuatf {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            Position: ovrVector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let layer = ovrLayerEyeFov {
            Header: ovrLayerHeader {
                Type: ovrLayerType__ovrLayerType_EyeFov,
                Flags: 0,
                Reserved: [0; 128usize],
            },
            ColorTexture: [raw, std::ptr::null_mut()],
            Viewport: rects,
            Fov: session.get_default_fov(),
            RenderPose: [pose, pose],
            SensorSampleTime: 0.0,
        };
        Self {
            raw,
            layer,
            session: session.raw,
        }
    }
    pub fn acquire_next_image(&self) -> Result<usize> {
        let mut index = -1;
        unsafe {
            check_error(ovr_GetTextureSwapChainCurrentIndex(
                self.session,
                self.raw,
                &mut index,
            ))?
        };
        Ok(index as usize)
    }

    pub fn wait_to_begin_frame(&self, frame: u64) -> Result<()> {
        unsafe { check_error(ovr_WaitToBeginFrame(self.session, frame as _))? };
        Ok(())
    }

    pub fn begin_frame(&self, frame: u64) -> Result<()> {
        unsafe {
            check_error(ovr_BeginFrame(self.session, frame as _))?;
        }
        Ok(())
    }

    pub fn end_frame(&mut self, frame: u64, pose: [ovrPosef; 2], sample_time: f64) -> Result<()> {
        self.layer.RenderPose = pose;
        self.layer.SensorSampleTime = sample_time;
        unsafe {
            check_error(ovr_EndFrame(
                self.session,
                frame as _,
                std::ptr::null(),
                transmute(&&self.layer),
                1,
            ))?;
        }
        Ok(())
    }

    pub fn display_time(&self, frame: u64) -> f64 {
        unsafe { ovr_GetPredictedDisplayTime(self.session, frame as _) }
    }

    pub fn commit(&self) -> Result<()> {
        unsafe {
            check_error(ovr_CommitTextureSwapChain(self.session, self.raw))?;
        }
        Ok(())
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            ovr_DestroyTextureSwapChain(self.session, self.raw);
        }
    }
}

#[test]
fn test_layout() {
    use std::mem::size_of;
    let eye_poses = [
        ovrPosef {
            Orientation: ovrQuatf {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0,
            },
            Position: ovrVector3f {
                x: 5.0,
                y: 6.0,
                z: 7.0,
            },
        },
        ovrPosef {
            Orientation: ovrQuatf {
                x: 8.0,
                y: 9.0,
                z: 10.0,
                w: 11.0,
            },
            Position: ovrVector3f {
                x: 12.0,
                y: 13.0,
                z: 14.0,
            },
        },
    ];

    assert_eq!(size_of::<[ovrPosef; 2]>(), 14 * 4);
    let [(q1, p1), (q2, p2)]: [([f32; 4], [f32; 3]); 2] = unsafe { transmute(eye_poses) };

    assert_eq!(q1, [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(p1, [5.0, 6.0, 7.0]);
    assert_eq!(q2, [8.0, 9.0, 10.0, 11.0]);
    assert_eq!(p2, [12.0, 13.0, 14.0]);
}
