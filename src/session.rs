use oculussdk_sys::{ovrTextureSwapChainDesc_, ovr_Create, ovr_CreateTextureSwapChainVk, ovr_Destroy, ovr_Initialize, ovr_GetTextureSwapChainBufferVk, ovr_GetTextureSwapChainLength, ovr_GetSessionPhysicalDeviceVk};
use {
    crate::{Error, Result},
    libc::c_char,
    oculussdk_sys::{
        ovrGraphicsLuid, ovrInitFlags, ovrInitParams, ovrSession, ovrTextureSwapChainDesc,
    },
    std::{ffi::CStr, mem::{MaybeUninit, transmute}},
};


fn check_error(er: i32) -> Result<()> {
    if er == 0 {
        Ok(())
    } else {
        Err(er.into())
    }
}

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
    ) -> Result<(ash::vk::SwapchainKHR, Vec<ash::vk::Image>)> {
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
           // ash::vk::Format::R8G8B8A8_UNORM_SRGB => ovrTextureFormat__OVR_FORMAT_R8G8B8A8_UNORM_SRGB,
            ash::vk::Format::B8G8R8A8_UNORM => ovrTextureFormat__OVR_FORMAT_B8G8R8A8_UNORM,
            //ash::vk::Format::B8G8R8A8_UNORM_SRGB => ovrTextureFormat__OVR_FORMAT_B8G8R8A8_UNORM_SRGB,
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
        let swapchain = unsafe{
            swapchain.assume_init()
        };

        let mut swapchain_images = 0;

        unsafe{
            check_error(
                ovr_GetTextureSwapChainLength(self.raw, swapchain, &mut swapchain_images)
            )?;
        }

        let mut images = Vec::with_capacity(swapchain_images as usize);
        for i in 0..swapchain_images as usize{
            let mut image = unsafe{
                MaybeUninit::uninit()
            };

            unsafe{
                check_error(
                    ovr_GetTextureSwapChainBufferVk(self.raw, swapchain, i as _, image.as_mut_ptr())
                )?;
            }

            images.push(
                unsafe{
                    transmute(image.assume_init())
                }
            );
        }



        let swapchain = unsafe{
           transmute(swapchain)
        };

        Ok((swapchain, images))
    }

    pub fn get_physical_device(&self, instance: ash::vk::Instance) -> Result<ash::vk::PhysicalDevice>{

        let mut out = unsafe{
            MaybeUninit::uninit()
        };

        unsafe{
            check_error(
                ovr_GetSessionPhysicalDeviceVk(self.raw, self.luid, transmute(instance), out.as_mut_ptr())
            )?;
        }

        Ok(
            unsafe{
                transmute(out.assume_init())
            }
        )
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { ovr_Destroy(self.raw) }
    }
}
