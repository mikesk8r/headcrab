use bitflags::bitflags;

// bitflags! {
//     pub struct VTFBitFlags: u32 {
const VTF_POINTSAMPLE: u32 = 0x00000001;
const VTF_TRILINEAR: u32 = 0x00000002;
const VTF_CLAMPS: u32 = 0x00000004;
const VTF_CLAMPT: u32 = 0x00000008;
const VTF_ANISOTROPIC: u32 = 0x00000010;
const VTF_HINT_DXT5: u32 = 0x00000020;
const VTF_PWL_CORRECTED: u32 = 0x00000040;
const VTF_NORMAL: u32 = 0x00000080;
const VTF_NOMIP: u32 = 0x00000100;
const VTF_NOLOD: u32 = 0x00000200;
const VTF_ALL_MIPS: u32 = 0x00000400;
const VTF_PROCEDURAL: u32 = 0x00000800;
const VTF_ONEBITALPHA: u32 = 0x00001000;
const VTF_EIGHTBITALPHA: u32 = 0x00002000;
const VTF_ENVMAP: u32 = 0x00004000;
const VTF_RENDERTARGET: u32 = 0x00008000;
const VTF_DEPTHRENDERTARGET: u32 = 0x00010000;
const VTF_NODEBUGOVERRIDE: u32 = 0x00020000;
const VTF_SINGLECOPY: u32 = 0x00040000;
const VTF_PRE_SRGB: u32 = 0x00080000;
const VTF_NODEPTHBUFFER: u32 = 0x00800000;
const VTF_CLAMPU: u32 = 0x02000000;
const VTF_VERTEXTEXTURE: u32 = 0x04000000;
const VTF_SSBUMP: u32 = 0x08000000;
const VTF_BORDER: u32 = 0x20000000;
//     }
// }

#[derive(Debug, Default, PartialEq)]
pub struct VTFFlags {
    pub all_mips: bool,
    pub anisotropic: bool,
    pub border: bool,
    pub clamp_s: bool,
    pub clamp_t: bool,
    pub clamp_u: bool,
    pub cubemap: bool,
    pub depth_render_target: bool,
    pub eight_bit_alpha: bool,
    pub no_debug_override: bool,
    pub no_depth_buffer: bool,
    pub no_mips: bool,
    pub no_lod: bool,
    pub normal_map: bool,
    pub one_bit_alpha: bool,
    pub pointsample: bool,
    pub pre_srgb: bool,
    pub procedural: bool,
    pub pwl_corrected: bool,
    pub single_copy: bool,
    pub ssbump: bool,
    pub trilinear: bool,
    pub render_target: bool,
    pub vertex_texture: bool,
}

impl VTFFlags {
    pub fn from_flags(raw: u32) -> Self {
        Self {
            all_mips: raw & VTF_ALL_MIPS != 0,
            anisotropic: raw & VTF_ANISOTROPIC != 0,
            border: raw & VTF_BORDER != 0,
            clamp_s: raw & VTF_CLAMPS != 0,
            clamp_t: raw & VTF_CLAMPT != 0,
            clamp_u: raw & VTF_CLAMPU != 0,
            cubemap: raw & VTF_ENVMAP != 0,
            depth_render_target: raw & VTF_DEPTHRENDERTARGET != 0,
            eight_bit_alpha: raw & VTF_EIGHTBITALPHA != 0,
            no_debug_override: raw & VTF_NODEBUGOVERRIDE != 0,
            no_depth_buffer: raw & VTF_NODEPTHBUFFER != 0,
            no_mips: raw & VTF_NOMIP != 0,
            no_lod: raw & VTF_NOLOD != 0,
            normal_map: raw & VTF_NORMAL != 0,
            one_bit_alpha: raw & VTF_ONEBITALPHA != 0,
            pointsample: raw & VTF_POINTSAMPLE != 0,
            pre_srgb: raw & VTF_PRE_SRGB != 0,
            procedural: raw & VTF_PROCEDURAL != 0,
            pwl_corrected: raw & VTF_PWL_CORRECTED != 0,
            single_copy: raw & VTF_SINGLECOPY != 0,
            ssbump: raw & VTF_SSBUMP != 0,
            trilinear: raw & VTF_TRILINEAR != 0,
            render_target: raw & VTF_RENDERTARGET != 0,
            vertex_texture: raw & VTF_VERTEXTEXTURE != 0,
        }
    }
}
