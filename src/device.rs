use bitflags::bitflags;
use std::mem;
use std::sync;

use crate::Format;
use crate::ImageMipArraySlice;
use crate::StringView;

#[repr(u32)]
pub enum DeviceType {
    Other = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_OTHER,
    IntegratedGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_INTEGRATED_GPU,
    DiscreteGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_DISCRETE_GPU,
    VirtualGpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_VIRTUAL_GPU,
    Cpu = daxa_sys::daxa_DeviceType_DAXA_DEVICE_TYPE_CPU,
}

bitflags! {
    #[derive(Default)]
    pub struct DeviceFlags: u64 {
        const BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
        const CONSERVATIVE_RASTERIZATION = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_CONSERVATIVE_RASTERIZATION;
        const MESH_SHADER_BIT = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_MESH_SHADER_BIT;
        const SHADER_ATOMIC64 = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_SHADER_ATOMIC64;
        const IMAGE_ATOMIC64 = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_IMAGE_ATOMIC64;
        const VK_MEMORY_MODEL = daxa_sys::daxa_DeviceFlagBits_DAXA_DEVICE_FLAG_VK_MEMORY_MODEL;
    }
}

pub type DeviceSelector = extern "C" fn(*const VkPhysicalDeviceProperties) -> i32;

pub extern "C" fn default_device_selector(properties: *const VkPhysicalDeviceProperties) -> i32 {
    unsafe { daxa_sys::daxa_default_device_score(properties) }
}

pub const VK_UUID_SIZE: usize = 16;

pub type VkPhysicalDeviceLimits = daxa_sys::VkPhysicalDeviceLimits;
pub type VkPhysicalDeviceSparseProperties = daxa_sys::VkPhysicalDeviceSparseProperties;

pub struct VkPhysicalDeviceProperties<'a> {
    api_version: u32,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    device_type: DeviceType,
    device_name: StringView<'a>,
    pipeline_cache_uuid: [u8; VK_UUID_SIZE],
    limits: VkPhysicalDeviceLimits,
    sparse_properties: VkPhysicalDeviceSparseProperties,
}

pub struct DeviceInfo<'a> {
    selector: DeviceSelector,
    flags: DeviceFlags,
    max_allowed_images: u32,
    max_allowed_buffers: u32,
    max_allowed_samplers: u32,
    name: StringView<'a>,
}

impl Default for DeviceInfo<'_> {
    fn default() -> Self {
        Self {
            selector: default_device_selector,
            flags: DeviceFlags::BUFFER_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
            max_allowed_images: 10000,
            max_allowed_buffers: 10000,
            max_allowed_samplers: 10000,
            name: StringView::from(b""),
        }
    }
}

pub type BinarySemaphore = daxa_sys::daxa_BinarySemaphore;
pub type CommandRecorder = daxa_sys::daxa_CommandRecorder;
pub type Swapchain = daxa_sys::daxa_Swapchain;

bitflags! {
    #[derive(Default)]
    pub struct PipelineStageFlags: i32 {
        const TOP_OF_PIPE_BIT = daxa_sys::VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT;
        const DRAW_INDIRECT_BIT = daxa_sys::VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT;
        const VERTEX_INPUT_BIT = daxa_sys::VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT;
        const VERTEX_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT;
        const TESSELLATION_CONTROL_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT;
        const TESSELLATION_EVALUATION_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT;
        const GEOMETRY_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT;
        const FRAGMENT_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT;
        const EARLY_FRAGMENT_TESTS_BIT = daxa_sys::VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT;
        const LATE_FRAGMENT_TESTS_BIT = daxa_sys::VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT;
        const COLOR_ATTACHMENT_OUTPUT_BIT = daxa_sys::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT;
        const COMPUTE_SHADER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT;
        const TRANSFER_BIT = daxa_sys::VK_PIPELINE_STAGE_2_TRANSFER_BIT;
        const BOTTOM_OF_PIPE_BIT = daxa_sys::VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT;
        const HOST_BIT = daxa_sys::VK_PIPELINE_STAGE_2_HOST_BIT;
        const ALL_GRAPHICS_BIT = daxa_sys::VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT;
        const ALL_COMMANDS_BIT = daxa_sys::VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT;
        const NONE = daxa_sys::VK_PIPELINE_STAGE_2_NONE;
        const TRANSFORM_FEEDBACK_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT;
        const CONDITIONAL_RENDERING_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT;
        const ACCELERATION_STRUCTURE_BUILD_BIT_KHR = daxa_sys::VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
        const RAY_TRACING_SHADER_BIT_KHR = daxa_sys::VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR;
        const FRAGMENT_DENSITY_PROCESS_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = daxa_sys::VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
        const COMMAND_PREPROCESS_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV;
        const TASK_SHADER_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT;
        const MESH_SHADER_BIT_EXT = daxa_sys::VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT;
        const SHADING_RATE_IMAGE_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
        const RAY_TRACING_SHADER_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR;
        const ACCELERATION_STRUCTURE_BUILD_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
        const TASK_SHADER_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT;
        const MESH_SHADER_BIT_NV = daxa_sys::VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT;
        const NONE_KHR = VK_PIPELINE_STAGE_2_NONE;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct ImageViewType: i32 {
        const OneDim = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_1D;
        const TwoDim = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_2D;
        const ThreeDim = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_3D;
        const Cube = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_CUBE;
        const OneDimArray = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_1D_ARRAY;
        const TwoDimArray = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_2D_ARRAY;
        const CubeArray = daxa_sys::VkImageViewType_VK_IMAGE_VIEW_TYPE_CUBE_ARRAY;
    }
}

pub struct CommandSubmitInfo<'a> {
    wait_stages: PipelineStageFlags,
    cmd_recorders: &'a [CommandRecorder],
    wait_binary_semaphores: &'a [BinarySemaphore],
    signal_binary_semaphores: &'a [BinarySemaphore],
    wait_timeline_semaphores: &'a [BinarySemaphore],
    signal_timeline_semaphores: &'a [BinarySemaphore],
}

pub struct PresentInfo<'a> {
    wait_binary_semaphores: &'a [BinarySemaphore],
    swapchain: Swapchain,
}

pub struct Device(daxa_sys::daxa_Device);

pub type BufferInfo = daxa_sys::daxa_BufferInfo;
pub type ImageInfo = daxa_sys::daxa_ImageInfo;
pub type BufferId = daxa_sys::daxa_BufferId;
pub type ImageId = daxa_sys::daxa_ImageId;
pub type ImageViewId = daxa_sys::daxa_ImageViewId;
pub type SamplerId = daxa_sys::daxa_SamplerId;

pub type BufferDeviceAddress = daxa_sys::daxa_BufferDeviceAddress;

pub struct ImageViewInfo<'a> {
    ty: ImageViewType,
    format: Format,
    image: ImageId,
    slice: ImageMipArraySlice,
    name: crate::types::StringView<'a>,
}

pub struct SamplerInfo<'a> {
    ty: ImageViewType,
    format: Format,
    image: ImageId,
    slice: ImageMipArraySlice,
    name: crate::types::StringView<'a>,
}

#[derive(Clone)]
pub struct Buffer {
    device: Device,
    handle: BufferId,
}

impl Buffer {
    pub fn id(&self) -> BufferId {
        unsafe { self.handle }
    }
}

#[derive(Clone)]
struct Image {
    device: Device,
    handle: ImageId,
}

impl Image {
    pub fn id(&self) -> ImageId {
        self.handle
    }
}

#[derive(Clone)]
struct ImageView {
    device: Device,
    handle: ImageViewId,
}

impl ImageView {
    fn id(&self) -> ImageViewId {
        self.handle
    }
}

#[derive(Clone)]
pub struct Sampler {
    device: Device,
    handle: SamplerId,
}

impl Sampler {
    fn id(&self) -> SamplerId {
        self.handle
    }
}

impl Device {
    pub fn buffer_memory_requirements(&self, info: &[BufferInfo]) -> MemoryRequirements {
        unsafe {
            mem::transmute::<MemoryRequirements>(daxa_sys::daxa_dvc_buffer_memory_requirements(
                self.handle,
                info.as_ptr().cast::<daxa_sys::daxa_BufferInfo>(),
            ))
        }
    }

    pub fn image_memory_requirements(&self, info: &[ImageInfo]) -> MemoryRequirements {
        unsafe {
            mem::transmute::<MemoryRequirements>(daxa_sys::daxa_dvc_image_memory_requirements(
                self.handle,
                info.as_ptr().cast::<daxa_sys::daxa_ImageInfo>(),
            ))
        }
    }

    pub fn create_memory(
        &self,
        info: &[MemoryBlockInfo],
    ) -> std::result::Result<MemoryBlock, crate::types::Result> {
        use crate::types::Result;
        use Result::*;
        unsafe {
            let mut out_memory_block = mem::zeroed();

            let c_result =
                daxa_sys::daxa_dvc_create_memory(self.handle, info.as_ptr().cast::<daxa_sys::daxa_MemoryBlockInfo>(), &mut out_memory_block);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(out_memory_block),
                _ => Err(c_result),
            }
        }
    }

    pub fn create_buffer(
        &self,
        info: &[BufferInfo],
    ) -> std::result::Result<Buffer, crate::types::Result> {
        use crate::types::Result;
        use Result::*;
        unsafe {
            let mut handle = mem::zeroed();

            let c_result =
                daxa_sys::daxa_dvc_create_buffer(self.handle, info.as_ptr().cast::<daxa_sys::daxa_BufferInfo>(), &mut handle);

            let buffer = Buffer {
                    handle,
                    device: self.clone(),
            };

            match mem::transmute::<Result>(c_result) {
                Success => Ok(buffer),
                _ => Err(c_result),
            }
        }
    }

    pub fn create_image(
        &self,
        info: &[ImageInfo],
    ) -> std::result::Result<Image, crate::types::Result> {
        use crate::types::Result;
        use Result::*;
        unsafe {
            let mut handle = mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_image(self.handle, info.as_ptr().cast::<daxa_sys::daxa_ImageInfo>(), &mut handle);

            let image = Buffer {
                    handle,
                    device: self.clone(),
            };

            match mem::transmute::<Result>(c_result) {
                Success => Ok(image),
                _ => Err(c_result),
            }
        }
    }

    pub fn create_image_view(
        &self,
        info: &[ImageViewInfo],
    ) -> std::result::Result<ImageView, crate::types::Result> {
        use crate::types::Result;
        use Result::*;
        unsafe {
            let mut handle = mem::zeroed();

            let c_result =
                daxa_sys::daxa_dvc_create_image_view(self.handle, info.as_ptr().cast::<daxa_sys::daxa_ImageViewInfo>(), &mut handle);

            let image_view = ImageView {
                    handle,
                    device: self.clone(),
            };

            match mem::transmute::<Result>(c_result) {
                Success => Ok(image_view),
                _ => Err(c_result),
            }
        }
    }

    pub fn create_sampler(
        &self,
        info: &[SamplerInfo],
    ) -> std::result::Result<Sampler, crate::types::Result> {
        use crate::types::Result;
        use Result::*;
        unsafe {
            let mut handle = mem::zeroed();

            let c_result =
                daxa_sys::daxa_dvc_create_sampler(self.handle, info.as_ptr().cast::<daxa_sys::daxa_SamplerInfo>(), &mut handle);

            let sampler = Sampler {
                    handle,
                    device: self.clone(),
            };

            match mem::transmute::<Result>(c_result) {
                Success => Ok(sampler),
                _ => Err(c_result),
            }
        }
    }

    pub fn is_buffer_valid(&self, buffer: BufferId) -> bool {
        unsafe { 
            daxa_sys::daxa_dvc_is_buffer_valid(self.handle, buffer)
        }
    }
    
    pub fn is_image_valid(&self, image: ImageId) -> bool {
        unsafe { 
            daxa_sys::daxa_dvc_is_image_valid(self.handle, image)
        }
    }
    
    pub fn is_image_view_valid(&self, image_view: ImageViewId) -> bool {
        unsafe { 
            daxa_sys::daxa_dvc_is_image_view_valid(self.handle, image_view)
        }
    }
    
    pub fn is_sampler_valid(&self, sampler: SamplerId) -> bool {
        unsafe { 
            daxa_sys::daxa_dvc_is_sampler_valid(self.handle, sampler)
        }
    }

    pub fn create_raster_pipeline(&self, info: &crate::pipeline::RasterPipelineInfo) -> std::result::Result<crate::pipeline::RasterPipeline, crate::types::Result>  {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_RasterPipelineInfo>();

            let mut raster_pipeline = std::mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_raster_pipeline(self.handle, c_info, &mut raster_pipeline);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(raster_pipeline),
                error => Err(error),
            }
        }
    }

    //compute
    pub fn create_compute_pipeline(&self, info: &crate::pipeline::ComputePipelineInfo) -> std::result::Result<crate::pipeline::ComputePipeline, crate::types::Result>  {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_ComputePipelineInfo>();

            let mut compute_pipeline = std::mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_compute_pipeline(self.handle, c_info, &mut compute_pipeline);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(compute_pipeline),
                error => Err(error),
            }
        }
    }

    //swapchain
    pub fn create_swapchain(&self, info: &crate::pipeline::SwapchainInfo) -> std::result::Result<crate::pipeline::Swapchain, crate::types::Result>  {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_SwapchainInfo>();

            let mut swapchain = std::mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_swapchain(self.handle, c_info, &mut swapchain);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(swapchain),
                error => Err(error),
            }
        }
    }

    //command recorder
    pub fn create_command_recorder(&self, info: &crate::pipeline::CommandRecorderInfo) -> std::result::Result<crate::pipeline::CommandRecorder, crate::types::Result>  {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_CommandRecorderInfo>();

            let mut command_recorder = std::mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_command_recorder(self.handle, c_info, &mut command_recorder);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(command_recorder),
                error => Err(error),
            }
        }
    }

    //binary semaphore
    pub fn create_binary_semaphore(&self, info: &crate::pipeline::BinarySemaphoreInfo) -> std::result::Result<crate::pipeline::BinarySemaphore, crate::types::Result>  {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_BinarySemaphoreInfo>();

            let mut binary_semaphore = std::mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_binary_semaphore(self.handle, c_info, &mut binary_semaphore);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(binary_semaphore),
                error => Err(error),
            }
        }
    }

    //timeline semaphore
    pub fn create_timeline_semaphore(&self, info: &crate::pipeline::TimelineSemaphoreInfo) -> std::result::Result<crate::pipeline::TimelineSemaphore, crate::types::Result>  {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_TimelineSemaphoreInfo>();

            let mut timeline_semaphore = std::mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_timeline_semaphore(self.handle, c_info, &mut timeline_semaphore);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(timeline_semaphore),
                error => Err(error),
            }
        }
    }

    //event
    pub fn create_event(&self, info: &crate::pipeline::EventInfo) -> std::result::Result<crate::pipeline::Event, crate::types::Result>  {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_EventInfo>();

            let mut event = std::mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_event(self.handle, c_info, &mut event);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(event),
                error => Err(error),
            }
        }
    }

    //timeline query_pool
    pub fn create_timeline_query_pool(&self, info: &crate::pipeline::TimelineQueryInfoInfo) -> std::result::Result<crate::pipeline::TimelineQueryInfo, crate::types::Result>  {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            let c_info = info.as_ptr().cast::<daxa_sys::daxa_TimelineQueryInfoInfo>();

            let mut timeline_query_pool = std::mem::zeroed();

            let c_result = daxa_sys::daxa_dvc_create_timeline_query_pool(self.handle, c_info, &mut timeline_query_pool);

            match mem::transmute::<Result>(c_result) {
                Success => Ok(timeline_query_pool),
                error => Err(error),
            }
        }
    }

    pub fn buffer_device_address(&self, buffer: BufferId) -> BufferDeviceAddress {
        unsafe {
            let mut address = mem::zeroed();
            daxa_sys::daxa_dvc_buffer_device_address(self.handle, buffer, &mut address);
            address
        }
    }

    pub fn buffer_host_address(&self, buffer: BufferId) -> *mut () {
        unsafe {
            let mut address = mem::zeroed();
            daxa_sys::daxa_dvc_buffer_device_address(self.handle, buffer, &mut address as _ as _);
            address
        }
    }

    pub fn info(&self) -> &DeviceInfo {
        unsafe {
            daxa_sys::daxa_dvc_info(self.handle).as_ref().unwrap()
        }
    }

    //TODO submit
    //TODO present

    pub fn wait_idle(&self) -> std::result::Result<(), crate::types::Result> {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            match mem::transmute::<Result>(daxa_sys::daxa_dvc_wait_idle(self.handle)) {
                Success => Ok(()),
                error => Err(error),
            }
        }
    }

    
    pub fn collect_garbage(&self) -> std::result::Result<(), crate::types::Result> {
        use crate::types::Result;
        use Result::Success;
        unsafe {
            match mem::transmute::<Result>(daxa_sys::daxa_dvc_collect_garbage(self.handle)) {
                Success => Ok(()),
                error => Err(error),
            }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            daxa_sys::daxa_destroy_device(self.handle);
        }
    }
}
