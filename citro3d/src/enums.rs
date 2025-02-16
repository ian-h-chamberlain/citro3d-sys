use ctru_sys;

/// Texture filters.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_FILTER_PARAM")]
pub enum TextureFilterParam {
    #[doc(alias = "GPU_NEAREST")]
    Nearest = ctru_sys::GPU_NEAREST,
    #[doc(alias = "GPU_LINEAR")]
    Linear = ctru_sys::GPU_LINEAR,
}

/// Texture wrap modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_WRAP_PARAM")]
pub enum TextureWrapParam {
    #[doc(alias = "GPU_CLAMP_TO_EDGE")]
    ClampToEdge = ctru_sys::GPU_CLAMP_TO_EDGE,
    #[doc(alias = "GPU_CLAMP_TO_BORDER")]
    ClampToBorder = ctru_sys::GPU_CLAMP_TO_BORDER,
    #[doc(alias = "GPU_REPEAT")]
    Repeat = ctru_sys::GPU_REPEAT,
    #[doc(alias = "GPU_MIRRORED_REPEAT")]
    MirroredRepeat = ctru_sys::GPU_MIRRORED_REPEAT,
}

/// Texture modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_MODE_PARAM")]
pub enum TextureModeParam {
    #[doc(alias = "GPU_TEX_2D")]
    Tex2D = ctru_sys::GPU_TEX_2D,
    #[doc(alias = "GPU_TEX_CUBE_MAP")]
    CubeMap = ctru_sys::GPU_TEX_CUBE_MAP,
    #[doc(alias = "GPU_TEX_SHADOW_2D")]
    Shadow2D = ctru_sys::GPU_TEX_SHADOW_2D,
    #[doc(alias = "GPU_TEX_PROJECTION")]
    Projection = ctru_sys::GPU_TEX_PROJECTION,
    #[doc(alias = "GPU_TEX_SHADOW_CUBE")]
    ShadowCube = ctru_sys::GPU_TEX_SHADOW_CUBE,
    #[doc(alias = "GPU_TEX_DISABLED")]
    Disabled = ctru_sys::GPU_TEX_DISABLED,
}

/// Supported texture units.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXUNIT")]
pub enum TexureUnit {
    #[doc(alias = "GPU_TEXUNIT0")]
    Unit0 = ctru_sys::GPU_TEXUNIT0,
    #[doc(alias = "GPU_TEXUNIT1")]
    Unit1 = ctru_sys::GPU_TEXUNIT1,
    #[doc(alias = "GPU_TEXUNIT2")]
    Unit2 = ctru_sys::GPU_TEXUNIT2,
}

/// Supported texture formats.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXCOLOR")]
pub enum TextureColorFormat {
    /// 8-bit Red + 8-bit Green + 8-bit Blue + 8-bit Alpha
    #[doc(alias = "GPU_RGBA8")]
    Rgba8 = ctru_sys::GPU_RGBA8,
    /// 8-bit Red + 8-bit Green + 8-bit Blue    #[doc(alias = "GPU_RGB8")]
    Rgb8 = ctru_sys::GPU_RGB8,
    /// 5-bit Red + 5-bit Green + 5-bit Blue + 1-bit Alpha
    #[doc(alias = "GPU_RGBA5551")]
    Rgba5551 = ctru_sys::GPU_RGBA5551,
    /// 5-bit Red + 6-bit Green + 5-bit Blue
    #[doc(alias = "GPU_RGB565")]
    Rgb565 = ctru_sys::GPU_RGB565,
    /// 4-bit Red + 4-bit Green + 4-bit Blue + 4-bit Alpha
    #[doc(alias = "GPU_RGBA4")]
    Rgba4 = ctru_sys::GPU_RGBA4,
    /// 8-bit Luminance + 8-bit Alpha
    #[doc(alias = "GPU_LA8")]
    La8 = ctru_sys::GPU_LA8,
    /// 8-bit Hi + 8-bit Lo
    #[doc(alias = "GPU_HILO8")]
    Hilo8 = ctru_sys::GPU_HILO8,
    /// 8-bit Luminance
    #[doc(alias = "GPU_L8")]
    L8 = ctru_sys::GPU_L8,
    /// 8-bit Alpha
    #[doc(alias = "GPU_A8")]
    A8 = ctru_sys::GPU_A8,
    /// 4-bit Luminance + 4-bit Alpha
    #[doc(alias = "GPU_LA4")]
    La4 = ctru_sys::GPU_LA4,
    /// 4-bit Luminance
    #[doc(alias = "GPU_L4")]
    L4 = ctru_sys::GPU_L4,
    /// 4-bit Alpha
    #[doc(alias = "GPU_A4")]
    A4 = ctru_sys::GPU_A4,
    /// ETC1 texture compression
    #[doc(alias = "GPU_ETC1")]
    Etc1 = ctru_sys::GPU_ETC1,
    /// ETC1 texture compression + 4-bit Alpha
    #[doc(alias = "GPU_ETC1A4")]
    Etc1A4 = ctru_sys::GPU_ETC1A4,
}

/// Texture faces.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXFACE")]
pub enum TextureFace {
    /// 2D face
    #[doc(alias = "GPU_TEXFACE_2D")]
    TwoD = ctru_sys::GPU_TEXFACE_2D,
    /// +X face
    #[doc(alias = "GPU_POSITIVE_X")]
    PositiveX = ctru_sys::GPU_POSITIVE_X,
    /// -X face
    #[doc(alias = "GPU_NEGATIVE_X")]
    NegativeX = ctru_sys::GPU_NEGATIVE_X,
    /// +Y face
    #[doc(alias = "GPU_POSITIVE_Y")]
    PositiveY = ctru_sys::GPU_POSITIVE_Y,
    /// -Y face
    #[doc(alias = "GPU_NEGATIVE_Y")]
    NegativeY = ctru_sys::GPU_NEGATIVE_Y,
    /// +Z face
    #[doc(alias = "GPU_POSITIVE_Z")]
    PositiveZ = ctru_sys::GPU_POSITIVE_Z,
    /// -Z face
    #[doc(alias = "GPU_NEGATIVE_Z")]
    NegativeZ = ctru_sys::GPU_NEGATIVE_Z,
}

/// Procedural texture clamp modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_CLAMP")]
pub enum ProceduralTextureClamp {
    /// Clamp to zero.
    #[doc(alias = "GPU_PT_CLAMP_TO_ZERO")]
    ClampToZero = ctru_sys::GPU_PT_CLAMP_TO_ZERO,
    /// Clamp to edge.
    #[doc(alias = "GPU_PT_CLAMP_TO_EDGE")]
    ClampToEdge = ctru_sys::GPU_PT_CLAMP_TO_EDGE,
    /// Symmetrical repeat.
    #[doc(alias = "GPU_PT_REPEAT")]
    Repeat = ctru_sys::GPU_PT_REPEAT,
    /// Mirrored repeat.
    #[doc(alias = "GPU_PT_MIRRORED_REPEAT")]
    MirroredRepeat = ctru_sys::GPU_PT_MIRRORED_REPEAT,
    /// Pulse.
    #[doc(alias = "GPU_PT_PULSE")]
    Pulse = ctru_sys::GPU_PT_PULSE,
}

/// Procedural texture mapping functions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_MAPFUNC")]
pub enum ProceduralTextureMappingFunction {
    /// U
    #[doc(alias = "GPU_PT_U")]
    U = ctru_sys::GPU_PT_U,

    /// U2
    #[doc(alias = "GPU_PT_U2")]
    U2 = ctru_sys::GPU_PT_U2,

    /// V
    #[doc(alias = "GPU_PT_V")]
    V = ctru_sys::GPU_PT_V,

    /// V2
    #[doc(alias = "GPU_PT_V2")]
    V2 = ctru_sys::GPU_PT_V2,

    /// U+V
    #[doc(alias = "GPU_PT_ADD")]
    Add = ctru_sys::GPU_PT_ADD,

    /// U2+V2
    #[doc(alias = "GPU_PT_ADD2")]
    Add2 = ctru_sys::GPU_PT_ADD2,

    /// sqrt(U2+V2)
    #[doc(alias = "GPU_PT_SQRT2")]
    Sqrt2 = ctru_sys::GPU_PT_SQRT2,

    /// min
    #[doc(alias = "GPU_PT_MIN")]
    Min = ctru_sys::GPU_PT_MIN,

    /// max
    #[doc(alias = "GPU_PT_MAX")]
    Max = ctru_sys::GPU_PT_MAX,

    /// rmax
    #[doc(alias = "GPU_PT_RMAX")]
    RMax = ctru_sys::GPU_PT_RMAX,
}

/// Procedural texture shift values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_SHIFT")]
pub enum ProceduralTextureShift {
    /// No shift.
    #[doc(alias = "GPU_PT_NONE")]
    None = ctru_sys::GPU_PT_NONE,

    /// Odd shift.
    #[doc(alias = "GPU_PT_ODD")]
    Odd = ctru_sys::GPU_PT_ODD,

    /// Even shift.
    #[doc(alias = "GPU_PT_EVEN")]
    Even = ctru_sys::GPU_PT_EVEN,
}

/// Procedural texture filter values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_FILTER")]
pub enum ProceduralTextureFilter {
    /// Nearest-neighbor
    #[doc(alias = "GPU_PT_NEAREST")]
    Nearest = ctru_sys::GPU_PT_NEAREST,

    /// Linear interpolation
    #[doc(alias = "GPU_PT_LINEAR")]
    Linear = ctru_sys::GPU_PT_LINEAR,

    /// Nearest-neighbor with mipmap using nearest-neighbor
    #[doc(alias = "GPU_PT_NEAREST_MIP_NEAREST")]
    NearestMipNearest = ctru_sys::GPU_PT_NEAREST_MIP_NEAREST,

    /// Linear interpolation with mipmap using nearest-neighbor
    #[doc(alias = "GPU_PT_LINEAR_MIP_NEAREST")]
    LinearMipNearest = ctru_sys::GPU_PT_LINEAR_MIP_NEAREST,

    /// Nearest-neighbor with mipmap using linear interpolation
    #[doc(alias = "GPU_PT_NEAREST_MIP_LINEAR")]
    NearestMipLinear = ctru_sys::GPU_PT_NEAREST_MIP_LINEAR,

    /// Linear interpolation with mipmap using linear interpolation
    #[doc(alias = "GPU_PT_LINEAR_MIP_LINEAR")]
    LinearMipLinear = ctru_sys::GPU_PT_LINEAR_MIP_LINEAR,
}

/// Procedural texture LUT IDs.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_LUTID")]
pub enum ProceduralTextureLutId {
    /// Noise table
    #[doc(alias = "GPU_LUT_NOISE")]
    Noise = ctru_sys::GPU_LUT_NOISE,

    /// RGB mapping function table
    #[doc(alias = "GPU_LUT_RGBMAP")]
    RGBMap = ctru_sys::GPU_LUT_RGBMAP,

    /// Alpha mapping function table
    #[doc(alias = "GPU_LUT_ALPHAMAP")]
    AlphaMap = ctru_sys::GPU_LUT_ALPHAMAP,

    /// Color table
    #[doc(alias = "GPU_LUT_COLOR")]
    Color = ctru_sys::GPU_LUT_COLOR,

    /// Color difference table
    #[doc(alias = "GPU_LUT_COLORDIF")]
    ColorDif = ctru_sys::GPU_LUT_COLORDIF,
}

/// Test functions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TESTFUNC")]
pub enum TestFunction {
    /// Never pass.
    #[doc(alias = "GPU_NEVER")]
    Never = ctru_sys::GPU_NEVER,

    /// Always pass.
    #[doc(alias = "GPU_ALWAYS")]
    Always = ctru_sys::GPU_ALWAYS,

    /// Pass if equal.
    #[doc(alias = "GPU_EQUAL")]
    Equal = ctru_sys::GPU_EQUAL,

    /// Pass if not equal.
    #[doc(alias = "GPU_NOTEQUAL")]
    NotEqual = ctru_sys::GPU_NOTEQUAL,

    /// Pass if less than.
    #[doc(alias = "GPU_LESS")]
    Less = ctru_sys::GPU_LESS,

    /// Pass if less than or equal.
    #[doc(alias = "GPU_LEQUAL")]
    LessOrEqual = ctru_sys::GPU_LEQUAL,

    /// Pass if greater than.
    #[doc(alias = "GPU_GREATER")]
    Greater = ctru_sys::GPU_GREATER,

    /// Pass if greater than or equal.
    #[doc(alias = "GPU_GEQUAL")]
    GreaterOrEqual = ctru_sys::GPU_GEQUAL,
}

/// Early depth test functions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_EARLYDEPTHFUNC")]
pub enum EarlyDepthFunction {
    /// Pass if greater than or equal.
    #[doc(alias = "GPU_EARLYDEPTH_GEQUAL")]
    GreaterOrEqual = ctru_sys::GPU_EARLYDEPTH_GEQUAL,

    /// Pass if greater than.
    #[doc(alias = "GPU_EARLYDEPTH_GREATER")]
    Greater = ctru_sys::GPU_EARLYDEPTH_GREATER,

    /// Pass if less than or equal.
    #[doc(alias = "GPU_EARLYDEPTH_LEQUAL")]
    LessOrEqual = ctru_sys::GPU_EARLYDEPTH_LEQUAL,

    /// Pass if less than.
    #[doc(alias = "GPU_EARLYDEPTH_LESS")]
    Less = ctru_sys::GPU_EARLYDEPTH_LESS,
}

/// Gas depth functions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_GASDEPTHFUNC")]
pub enum GasDepthFunction {
    /// Never pass (0).
    #[doc(alias = "GPU_GAS_NEVER")]
    Never = ctru_sys::GPU_GAS_NEVER,

    /// Always pass (1).
    #[doc(alias = "GPU_GAS_ALWAYS")]
    Always = ctru_sys::GPU_GAS_ALWAYS,

    /// Pass if greater than (1-X).
    #[doc(alias = "GPU_GAS_GREATER")]
    Greater = ctru_sys::GPU_GAS_GREATER,

    /// Pass if less than (X).
    #[doc(alias = "GPU_GAS_LESS")]
    Less = ctru_sys::GPU_GAS_LESS,
}

/// Scissor test modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_SCISSORMODE")]
pub enum ScissorMode {
    /// Disable.
    #[doc(alias = "GPU_SCISSOR_DISABLE")]
    Disable = ctru_sys::GPU_SCISSOR_DISABLE,

    /// Exclude pixels inside the scissor box.
    #[doc(alias = "GPU_SCISSOR_INVERT")]
    Invert = ctru_sys::GPU_SCISSOR_INVERT,

    /// Exclude pixels outside of the scissor box.
    #[doc(alias = "GPU_SCISSOR_NORMAL")]
    Normal = ctru_sys::GPU_SCISSOR_NORMAL,
}

/// Stencil operations.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_STENCILOP")]
pub enum StencilOperation {
    /// Keep old value. (old_stencil)
    #[doc(alias = "GPU_STENCIL_KEEP")]
    Keep = ctru_sys::GPU_STENCIL_KEEP,

    /// Zero. (0)
    #[doc(alias = "GPU_STENCIL_ZERO")]
    Zero = ctru_sys::GPU_STENCIL_ZERO,

    /// Replace value. (ref)
    #[doc(alias = "GPU_STENCIL_REPLACE")]
    Replace = ctru_sys::GPU_STENCIL_REPLACE,

    /// Increment value. (old_stencil + 1 saturated to [0, 255])
    #[doc(alias = "GPU_STENCIL_INCR")]
    Increment = ctru_sys::GPU_STENCIL_INCR,

    /// Decrement value. (old_stencil - 1 saturated to [0, 255])
    #[doc(alias = "GPU_STENCIL_DECR")]
    Decrement = ctru_sys::GPU_STENCIL_DECR,

    /// Invert value. (~old_stencil)
    #[doc(alias = "GPU_STENCIL_INVERT")]
    Invert = ctru_sys::GPU_STENCIL_INVERT,

    /// Increment value. (old_stencil + 1)
    #[doc(alias = "GPU_STENCIL_INCR_WRAP")]
    IncrementWrap = ctru_sys::GPU_STENCIL_INCR_WRAP,

    /// Decrement value. (old_stencil - 1)
    #[doc(alias = "GPU_STENCIL_DECR_WRAP")]
    DecrementWrap = ctru_sys::GPU_STENCIL_DECR_WRAP,
}

/// Pixel write mask.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_WRITEMASK")]
pub enum WriteMask {
    /// Write red.
    #[doc(alias = "GPU_WRITE_RED")]
    Red = ctru_sys::GPU_WRITE_RED,

    /// Write green.
    #[doc(alias = "GPU_WRITE_GREEN")]
    Green = ctru_sys::GPU_WRITE_GREEN,

    /// Write blue.
    #[doc(alias = "GPU_WRITE_BLUE")]
    Blue = ctru_sys::GPU_WRITE_BLUE,

    /// Write alpha.
    #[doc(alias = "GPU_WRITE_ALPHA")]
    Alpha = ctru_sys::GPU_WRITE_ALPHA,

    /// Write depth.
    #[doc(alias = "GPU_WRITE_DEPTH")]
    Depth = ctru_sys::GPU_WRITE_DEPTH,

    /// Write all color components.
    #[doc(alias = "GPU_WRITE_COLOR")]
    Color = ctru_sys::GPU_WRITE_COLOR,

    /// Write all components.
    #[doc(alias = "GPU_WRITE_ALL")]
    All = ctru_sys::GPU_WRITE_ALL,
}

/// Blend modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_BLENDEQUATION")]
pub enum BlendEquation {
    /// Add colors.
    #[doc(alias = "GPU_BLEND_ADD")]
    Add = ctru_sys::GPU_BLEND_ADD,

    /// Subtract colors.
    #[doc(alias = "GPU_BLEND_SUBTRACT")]
    Subtract = ctru_sys::GPU_BLEND_SUBTRACT,

    /// Reverse-subtract colors.
    #[doc(alias = "GPU_BLEND_REVERSE_SUBTRACT")]
    ReverseSubtract = ctru_sys::GPU_BLEND_REVERSE_SUBTRACT,

    /// Use the minimum color.
    #[doc(alias = "GPU_BLEND_MIN")]
    Min = ctru_sys::GPU_BLEND_MIN,

    /// Use the maximum color.
    #[doc(alias = "GPU_BLEND_MAX")]
    Max = ctru_sys::GPU_BLEND_MAX,
}

/// Blend factors.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_BLENDFACTOR")]
pub enum BlendFactor {
    /// Zero.
    #[doc(alias = "GPU_ZERO")]
    Zero = ctru_sys::GPU_ZERO,

    /// One.
    #[doc(alias = "GPU_ONE")]
    One = ctru_sys::GPU_ONE,

    /// Source color.
    #[doc(alias = "GPU_SRC_COLOR")]
    SrcColor = ctru_sys::GPU_SRC_COLOR,

    /// Source color - 1.
    #[doc(alias = "GPU_ONE_MINUS_SRC_COLOR")]
    OneMinusSrcColor = ctru_sys::GPU_ONE_MINUS_SRC_COLOR,

    /// Destination color.
    #[doc(alias = "GPU_DST_COLOR")]
    DstColor = ctru_sys::GPU_DST_COLOR,

    /// Destination color - 1.
    #[doc(alias = "GPU_ONE_MINUS_DST_COLOR")]
    OneMinusDstColor = ctru_sys::GPU_ONE_MINUS_DST_COLOR,

    /// Source alpha.
    #[doc(alias = "GPU_SRC_ALPHA")]
    SrcAlpha = ctru_sys::GPU_SRC_ALPHA,

    /// Source alpha - 1.
    #[doc(alias = "GPU_ONE_MINUS_SRC_ALPHA")]
    OneMinusSrcAlpha = ctru_sys::GPU_ONE_MINUS_SRC_ALPHA,

    /// Destination alpha.
    #[doc(alias = "GPU_DST_ALPHA")]
    DstAlpha = ctru_sys::GPU_DST_ALPHA,

    /// Destination alpha - 1.
    #[doc(alias = "GPU_ONE_MINUS_DST_ALPHA")]
    OneMinusDstAlpha = ctru_sys::GPU_ONE_MINUS_DST_ALPHA,

    /// Constant color.
    #[doc(alias = "GPU_CONSTANT_COLOR")]
    ConstantColor = ctru_sys::GPU_CONSTANT_COLOR,

    /// Constant color - 1.
    #[doc(alias = "GPU_ONE_MINUS_CONSTANT_COLOR")]
    OneMinusConstantColor = ctru_sys::GPU_ONE_MINUS_CONSTANT_COLOR,

    /// Constant alpha.
    #[doc(alias = "GPU_CONSTANT_ALPHA")]
    ConstantAlpha = ctru_sys::GPU_CONSTANT_ALPHA,

    /// Constant alpha - 1.
    #[doc(alias = "GPU_ONE_MINUS_CONSTANT_ALPHA")]
    OneMinusConstantAlpha = ctru_sys::GPU_ONE_MINUS_CONSTANT_ALPHA,

    /// Saturated alpha.
    #[doc(alias = "GPU_SRC_ALPHA_SATURATE")]
    SrcAlphaSaturate = ctru_sys::GPU_SRC_ALPHA_SATURATE,
}

/// Logical operations.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_LOGICOP")]
pub enum LogicOperation {
    /// Clear.
    #[doc(alias = "GPU_LOGICOP_CLEAR")]
    Clear = ctru_sys::GPU_LOGICOP_CLEAR,

    /// Bitwise AND.
    #[doc(alias = "GPU_LOGICOP_AND")]
    And = ctru_sys::GPU_LOGICOP_AND,

    /// Reverse bitwise AND.
    #[doc(alias = "GPU_LOGICOP_AND_REVERSE")]
    AndReverse = ctru_sys::GPU_LOGICOP_AND_REVERSE,

    /// Copy.
    #[doc(alias = "GPU_LOGICOP_COPY")]
    Copy = ctru_sys::GPU_LOGICOP_COPY,

    /// Set.
    #[doc(alias = "GPU_LOGICOP_SET")]
    Set = ctru_sys::GPU_LOGICOP_SET,

    /// Inverted copy.
    #[doc(alias = "GPU_LOGICOP_COPY_INVERTED")]
    CopyInverted = ctru_sys::GPU_LOGICOP_COPY_INVERTED,

    /// No operation.
    #[doc(alias = "GPU_LOGICOP_NOOP")]
    Noop = ctru_sys::GPU_LOGICOP_NOOP,

    /// Invert.
    #[doc(alias = "GPU_LOGICOP_INVERT")]
    Invert = ctru_sys::GPU_LOGICOP_INVERT,

    /// Bitwise NAND.
    #[doc(alias = "GPU_LOGICOP_NAND")]
    Nand = ctru_sys::GPU_LOGICOP_NAND,

    /// Bitwise OR.
    #[doc(alias = "GPU_LOGICOP_OR")]
    Or = ctru_sys::GPU_LOGICOP_OR,

    /// Bitwise NOR.
    #[doc(alias = "GPU_LOGICOP_NOR")]
    Nor = ctru_sys::GPU_LOGICOP_NOR,

    /// Bitwise XOR.
    #[doc(alias = "GPU_LOGICOP_XOR")]
    Xor = ctru_sys::GPU_LOGICOP_XOR,

    /// Equivalent.
    #[doc(alias = "GPU_LOGICOP_EQUIV")]
    Equiv = ctru_sys::GPU_LOGICOP_EQUIV,

    /// Inverted bitwise AND.
    #[doc(alias = "GPU_LOGICOP_AND_INVERTED")]
    AndInverted = ctru_sys::GPU_LOGICOP_AND_INVERTED,

    /// Reverse bitwise OR.
    #[doc(alias = "GPU_LOGICOP_OR_REVERSE")]
    OrReverse = ctru_sys::GPU_LOGICOP_OR_REVERSE,

    /// Inverted bitwise OR.
    #[doc(alias = "GPU_LOGICOP_OR_INVERTED")]
    OrInverted = ctru_sys::GPU_LOGICOP_OR_INVERTED,
}

/// Fragment operation modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_FRAGOPMODE")]
pub enum FragmentOperationMode {
    /// OpenGL mode.
    #[doc(alias = "GPU_FRAGOPMODE_GL")]
    Gl = ctru_sys::GPU_FRAGOPMODE_GL,

    /// Gas mode (?).
    #[doc(alias = "GPU_FRAGOPMODE_GAS_ACC")]
    GasAcc = ctru_sys::GPU_FRAGOPMODE_GAS_ACC,

    /// Shadow mode (?).
    #[doc(alias = "GPU_FRAGOPMODE_SHADOW")]
    Shadow = ctru_sys::GPU_FRAGOPMODE_SHADOW,
}

/// Cull modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_CULLMODE")]
pub enum CullMode {
    /// Disabled.
    #[doc(alias = "GPU_CULL_NONE")]
    None = ctru_sys::GPU_CULL_NONE,

    /// Front, counter-clockwise.
    #[doc(alias = "GPU_CULL_FRONT_CCW")]
    FrontCounterClockwise = ctru_sys::GPU_CULL_FRONT_CCW,

    /// Back, counter-clockwise.
    #[doc(alias = "GPU_CULL_BACK_CCW")]
    BackCounterClockwise = ctru_sys::GPU_CULL_BACK_CCW,
}

/// Texture RGB combiner operands.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVOP_RGB")]
pub enum TextureRgbOperand {
    /// Source color.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_COLOR")]
    SrcColor = ctru_sys::GPU_TEVOP_RGB_SRC_COLOR,

    /// Source color - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR")]
    OneMinusSrcColor = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR,

    /// Source alpha.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_ALPHA")]
    SrcAlpha = ctru_sys::GPU_TEVOP_RGB_SRC_ALPHA,

    /// Source alpha - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA")]
    OneMinusSrcAlpha = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA,

    /// Source red.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_R")]
    SrcR = ctru_sys::GPU_TEVOP_RGB_SRC_R,

    /// Source red - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_R")]
    OneMinusSrcR = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_R,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x06")]
    _0x06 = ctru_sys::GPU_TEVOP_RGB_0x06,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x07")]
    UnknownHex07 = ctru_sys::GPU_TEVOP_RGB_0x07,

    /// Source green.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_G")]
    SrcG = ctru_sys::GPU_TEVOP_RGB_SRC_G,

    /// Source green - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_G")]
    OneMinusSrcG = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_G,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x0A")]
    UnknownHex0A = ctru_sys::GPU_TEVOP_RGB_0x0A,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x0B")]
    UnknownHex0B = ctru_sys::GPU_TEVOP_RGB_0x0B,

    /// Source blue.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_B")]
    SrcB = ctru_sys::GPU_TEVOP_RGB_SRC_B,

    /// Source blue - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_B")]
    OneMinusSrcB = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_B,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x0E")]
    UnknownHex0E = ctru_sys::GPU_TEVOP_RGB_0x0E,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x0F")]
    UnknownHex0F = ctru_sys::GPU_TEVOP_RGB_0x0F,
}

/// Texture Alpha combiner operands.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVOP_A")]
pub enum TextureAlphaOperand {
    /// Source alpha.
    #[doc(alias = "GPU_TEVOP_A_SRC_ALPHA")]
    SrcAlpha = ctru_sys::GPU_TEVOP_A_SRC_ALPHA,

    /// Source alpha - 1.
    #[doc(alias = "GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA")]
    OneMinusSrcAlpha = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA,

    /// Source red.
    #[doc(alias = "GPU_TEVOP_A_SRC_R")]
    SrcRed = ctru_sys::GPU_TEVOP_A_SRC_R,

    /// Source red - 1.
    #[doc(alias = "GPU_TEVOP_A_ONE_MINUS_SRC_R")]
    OneMinusSrcRed = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_R,

    /// Source green.
    #[doc(alias = "GPU_TEVOP_A_SRC_G")]
    SrcGreen = ctru_sys::GPU_TEVOP_A_SRC_G,

    /// Source green - 1.
    #[doc(alias = "GPU_TEVOP_A_ONE_MINUS_SRC_G")]
    OneMinusSrcGreen = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_G,

    /// Source blue.
    #[doc(alias = "GPU_TEVOP_A_SRC_B")]
    SrcBlue = ctru_sys::GPU_TEVOP_A_SRC_B,

    /// Source blue - 1.
    #[doc(alias = "GPU_TEVOP_A_ONE_MINUS_SRC_B")]
    OneMinusSrcBlue = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_B,
}

/// Texture scale factors.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVSCALE")]
pub enum TextureScale {
    /// 1x scale
    #[doc(alias = "GPU_TEVSCALE_1")]
    Original = ctru_sys::GPU_TEVSCALE_1,

    /// 2x scale
    #[doc(alias = "GPU_TEVSCALE_2")]
    Double = ctru_sys::GPU_TEVSCALE_2,

    /// 4x scale
    #[doc(alias = "GPU_TEVSCALE_4")]
    Quadruple = ctru_sys::GPU_TEVSCALE_4,
}
/// Fresnel options.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_FRESNELSEL")]
pub enum FresnelSel {
    /// None.
    #[doc(alias = "GPU_NO_FRESNEL")]
    NoFresnel = ctru_sys::GPU_NO_FRESNEL,

    /// Primary alpha.
    #[doc(alias = "GPU_PRI_ALPHA_FRESNEL")]
    PrimaryAlpha = ctru_sys::GPU_PRI_ALPHA_FRESNEL,

    /// Secondary alpha.
    #[doc(alias = "GPU_SEC_ALPHA_FRESNEL")]
    SecondaryAlpha = ctru_sys::GPU_SEC_ALPHA_FRESNEL,

    /// Primary and secondary alpha.
    #[doc(alias = "GPU_PRI_SEC_ALPHA_FRESNEL")]
    PrimarySecondaryAlpha = ctru_sys::GPU_PRI_SEC_ALPHA_FRESNEL,
}

/// Bump map modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_BUMPMODE")]
pub enum BumpMappingMode {
    /// Disabled.
    #[doc(alias = "GPU_BUMP_NOT_USED")]
    NotUsed = ctru_sys::GPU_BUMP_NOT_USED,

    /// Bump as bump mapping.
    #[doc(alias = "GPU_BUMP_AS_BUMP")]
    AsBump = ctru_sys::GPU_BUMP_AS_BUMP,

    /// Bump as tangent/normal mapping.
    #[doc(alias = "GPU_BUMP_AS_TANG")]
    AsTangent = ctru_sys::GPU_BUMP_AS_TANG,
}

/// LUT IDs.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_LIGHTLUTID")]
pub enum LightLutId {
    /// D0 LUT.
    #[doc(alias = "GPU_LUT_D0")]
    Directional0 = ctru_sys::GPU_LUT_D0,

    /// D1 LUT.
    #[doc(alias = "GPU_LUT_D1")]
    Directional1 = ctru_sys::GPU_LUT_D1,

    /// Spotlight LUT.
    #[doc(alias = "GPU_LUT_SP")]
    Spotlight = ctru_sys::GPU_LUT_SP,

    /// Fresnel LUT.
    #[doc(alias = "GPU_LUT_FR")]
    Fresnel = ctru_sys::GPU_LUT_FR,

    /// Reflection-Blue LUT.
    #[doc(alias = "GPU_LUT_RB")]
    ReflectionBlue = ctru_sys::GPU_LUT_RB,

    /// Reflection-Green LUT.
    #[doc(alias = "GPU_LUT_RG")]
    ReflectionGreen = ctru_sys::GPU_LUT_RG,

    /// Reflection-Red LUT.
    #[doc(alias = "GPU_LUT_RR")]
    ReflectionRed = ctru_sys::GPU_LUT_RR,

    /// Distance attenuation LUT.
    #[doc(alias = "GPU_LUT_DA")]
    DistanceAttenuation = ctru_sys::GPU_LUT_DA,
}

/// LUT inputs.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_LIGHTLUTINPUT")]
pub enum LightLutInput {
    /// Normal*HalfVector.
    #[doc(alias = "GPU_LUTINPUT_NH")]
    NormalHalfVector = ctru_sys::GPU_LUTINPUT_NH,

    /// View*HalfVector.
    #[doc(alias = "GPU_LUTINPUT_VH")]
    ViewHalfVector = ctru_sys::GPU_LUTINPUT_VH,

    /// Normal*View.
    #[doc(alias = "GPU_LUTINPUT_NV")]
    NormalView = ctru_sys::GPU_LUTINPUT_NV,

    /// LightVector*Normal.
    #[doc(alias = "GPU_LUTINPUT_LN")]
    LightVectorNormal = ctru_sys::GPU_LUTINPUT_LN,

    /// -LightVector*SpotlightVector.
    #[doc(alias = "GPU_LUTINPUT_SP")]
    NegativeLightVectorSpotlightVector = ctru_sys::GPU_LUTINPUT_SP,

    /// Cosine of phi.
    #[doc(alias = "GPU_LUTINPUT_CP")]
    CosineOfPhi = ctru_sys::GPU_LUTINPUT_CP,
}

/// LUT scalers.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_LIGHTLUTSCALER")]
pub enum LightLutScaler {
    /// 1x scale.
    #[doc(alias = "GPU_LUTSCALER_1x")]
    OneX = ctru_sys::GPU_LUTSCALER_1x,

    /// 2x scale.
    #[doc(alias = "GPU_LUTSCALER_2x")]
    TwoX = ctru_sys::GPU_LUTSCALER_2x,

    /// 4x scale.
    #[doc(alias = "GPU_LUTSCALER_4x")]
    FourX = ctru_sys::GPU_LUTSCALER_4x,

    /// 8x scale.
    #[doc(alias = "GPU_LUTSCALER_8x")]
    EightX = ctru_sys::GPU_LUTSCALER_8x,

    /// 0.25x scale.
    #[doc(alias = "GPU_LUTSCALER_0_25x")]
    QuarterX = ctru_sys::GPU_LUTSCALER_0_25x,

    /// 0.5x scale.
    #[doc(alias = "GPU_LUTSCALER_0_5x")]
    HalfX = ctru_sys::GPU_LUTSCALER_0_5x,
}

/// LUT selection.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_LIGHTLUTSELECT")]
pub enum LightLutSelect {
    /// LUTs that are common to all lights.
    #[doc(alias = "GPU_LUTSELECT_COMMON")]
    Common = ctru_sys::GPU_LUTSELECT_COMMON,

    /// Spotlight LUT.
    #[doc(alias = "GPU_LUTSELECT_SP")]
    Spotlight = ctru_sys::GPU_LUTSELECT_SP,

    /// Distance attenuation LUT.
    #[doc(alias = "GPU_LUTSELECT_DA")]
    DistanceAttenuation = ctru_sys::GPU_LUTSELECT_DA,
}

/// Fog modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_FOGMODE")]
pub enum FogMode {
    /// Fog/Gas unit disabled.
    #[doc(alias = "GPU_NO_FOG")]
    NoFog = ctru_sys::GPU_NO_FOG,

    /// Fog/Gas unit configured in Fog mode.
    #[doc(alias = "GPU_FOG")]
    Fog = ctru_sys::GPU_FOG,

    /// Fog/Gas unit configured in Gas mode.
    #[doc(alias = "GPU_GAS")]
    Gas = ctru_sys::GPU_GAS,
}

/// Gas shading density source values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_GASMODE")]
pub enum GasMode {
    /// Plain density.
    #[doc(alias = "GPU_PLAIN_DENSITY")]
    PlainDensity = ctru_sys::GPU_PLAIN_DENSITY,

    /// Depth density.
    #[doc(alias = "GPU_DEPTH_DENSITY")]
    DepthDensity = ctru_sys::GPU_DEPTH_DENSITY,
}

/// Gas color LUT inputs.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_GASLUTINPUT")]
pub enum GasLutInput {
    /// Gas density used as input.
    #[doc(alias = "GPU_GAS_DENSITY")]
    Density = ctru_sys::GPU_GAS_DENSITY,

    /// Light factor used as input.
    #[doc(alias = "GPU_GAS_LIGHT_FACTOR")]
    LightFactor = ctru_sys::GPU_GAS_LIGHT_FACTOR,
}