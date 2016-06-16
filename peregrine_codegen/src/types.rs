use std::str::FromStr;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::u8;

#[derive(Debug)]
pub struct ParseInsError {
    description: String,
}

impl ParseInsError {
    fn new(s: &str) -> ParseInsError {
        ParseInsError { description: s.to_owned() }
    }
}

impl Display for ParseInsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for ParseInsError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub name: String,
    pub summary: String,
    pub forms: Vec<InstructionForm>,
}

impl Instruction {
    pub fn new(name: &String) -> Instruction {
        Instruction {
            name: name.to_owned(),
            summary: String::from(""),
            forms: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstructionForm {
    pub mmx_mode: MMXMode,
    pub xmm_mode: XMMMode,
    pub canceling_inputs: bool,
    pub isas: Vec<ISA>,
    pub implicit_operands: Vec<ImplicitOperand>,
    pub operands: Vec<Operand>,
    pub encodings: Vec<Encoding>,
}

impl InstructionForm {
    pub fn new() -> InstructionForm {
        InstructionForm {
            mmx_mode: MMXMode::NONE,
            xmm_mode: XMMMode::NONE,
            canceling_inputs: false,
            isas: Vec::new(),
            implicit_operands: Vec::new(),
            operands: Vec::new(),
            encodings: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Encoding {
    pub prefix: Option<Prefix>,
    pub rex: Option<REX>,
    pub vex: Option<VEX>,
    pub evex: Option<EVEX>,
    pub opcodes: Vec<Opcode>,
    pub modrm: Option<ModRM>,
    pub register_byte: Option<RegisterByte>,
    pub immediate: Option<Immediate>,
    pub data_offset: Option<DataOffset>,
    pub code_offset: Option<CodeOffset>,
}

impl Encoding {
    pub fn new() -> Encoding {
        Encoding {
            prefix: None,
            rex: None,
            vex: None,
            evex: None,
            opcodes: Vec::new(),
            modrm: None,
            register_byte: None,
            immediate: None,
            data_offset: None,
            code_offset: None,
        }
    }
}


#[derive(Clone, Debug)]
pub enum MMXMode {
    FPU,
    MMX,
    NONE,
}

impl FromStr for MMXMode {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FPU" => Ok(MMXMode::FPU),
            "MMX" => Ok(MMXMode::MMX),
            _ => Err(ParseInsError::new("Invalid mmx_mode")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum XMMMode {
    SSE,
    AVX,
    NONE,
}

impl FromStr for XMMMode {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SSE" => Ok(XMMMode::SSE),
            "AVX" => Ok(XMMMode::AVX),
            _ => Err(ParseInsError::new("Invalid xmm_mode")),
        }
    }
}


#[derive(Clone, Debug)]
pub struct ImplicitOperand {
    pub id: ImplicitRegister,
    pub input: bool,
    pub output: bool,
}

impl ImplicitOperand {
    pub fn new() -> ImplicitOperand {
        ImplicitOperand {
            id: ImplicitRegister::NONE,
            input: false,
            output: false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ImplicitRegister {
    AX,
    AL,
    DX,
    EAX,
    EBX,
    ECX,
    EDX,
    RAX,
    RBX,
    RCX,
    RDI,
    RDX,
    XMM0,
    NONE,
}

impl FromStr for ImplicitRegister {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ax" => Ok(ImplicitRegister::AX),
            "al" => Ok(ImplicitRegister::AL),
            "dx" => Ok(ImplicitRegister::DX),
            "eax" => Ok(ImplicitRegister::EAX),
            "ebx" => Ok(ImplicitRegister::EBX),
            "ecx" => Ok(ImplicitRegister::ECX),
            "edx" => Ok(ImplicitRegister::EDX),
            "rax" => Ok(ImplicitRegister::RAX),
            "rbx" => Ok(ImplicitRegister::RBX),
            "rcx" => Ok(ImplicitRegister::RCX),
            "rdx" => Ok(ImplicitRegister::RDX),
            "rdi" => Ok(ImplicitRegister::RDI),
            "xmm0" => Ok(ImplicitRegister::XMM0),
            _ => Err(ParseInsError::new(format!("Invalid implicit register: {}", s).as_str())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Operand {
    pub id: OperandId,
    pub input: bool,
    pub output: bool,
    pub extended_size: u64,
}

impl Operand {
    pub fn new() -> Operand {
        Operand {
            id: OperandId::NONE,
            input: false,
            output: false,
            extended_size: 0,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum OperandId {
    _1_,
    _3_,
    al,
    ax,
    eax,
    rax,
    cl,
    xmm0,
    rel8,
    rel32,
    imm4,
    imm8,
    imm16,
    imm32,
    imm64,
    r8,
    r16,
    r32,
    r64,
    mm,
    xmm,
    xmm_k_,
    xmm_k_z_,
    ymm,
    ymm_k_,
    ymm_k_z_,
    zmm,
    zmm_k_,
    zmm_k_z_,
    k,
    k_k_,
    m,
    m8,
    m16,
    m16_k_z_,
    m32,
    m32_k_,
    m32_k_z_,
    m64,
    m64_k_,
    m64_k_z_,
    m80,
    m128,
    m128_k_z_,
    m256,
    m256_k_z_,
    m512,
    m512_k_z_,
    m64__m32bcst,
    m128__m32bcst,
    m256__m32bcst,
    m512__m32bcst,
    m128__m64bcst,
    m256__m64bcst,
    m512__m64bcst,
    moffs32,
    moffs64,
    vm32x,
    vm32x_k_,
    vm32y,
    vm32y_k_,
    vm32z,
    vm32z_k_,
    vm64x,
    vm64x_k_,
    vm64y,
    vm64y_k_,
    vm64z,
    vm64z_k_,
    _sae_,
    _er_,
    NONE,
}

impl FromStr for OperandId {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(OperandId::_1_),
            "3" => Ok(OperandId::_3_),
            "al" => Ok(OperandId::al),
            "ax" => Ok(OperandId::ax),
            "eax" => Ok(OperandId::eax),
            "rax" => Ok(OperandId::rax),
            "cl" => Ok(OperandId::cl),
            "xmm0" => Ok(OperandId::xmm0),
            "rel8" => Ok(OperandId::rel8),
            "rel32" => Ok(OperandId::rel32),
            "imm4" => Ok(OperandId::imm4),
            "imm8" => Ok(OperandId::imm8),
            "imm16" => Ok(OperandId::imm16),
            "imm32" => Ok(OperandId::imm32),
            "imm64" => Ok(OperandId::imm64),
            "r8" => Ok(OperandId::r8),
            "r16" => Ok(OperandId::r16),
            "r32" => Ok(OperandId::r32),
            "r64" => Ok(OperandId::r64),
            "mm" => Ok(OperandId::mm),
            "xmm" => Ok(OperandId::xmm),
            "xmm{k}" => Ok(OperandId::xmm_k_),
            "xmm{k}{z}" => Ok(OperandId::xmm_k_z_),
            "ymm" => Ok(OperandId::ymm),
            "ymm{k}" => Ok(OperandId::ymm_k_),
            "ymm{k}{z}" => Ok(OperandId::ymm_k_z_),
            "zmm" => Ok(OperandId::zmm),
            "zmm{k}" => Ok(OperandId::zmm_k_),
            "zmm{k}{z}" => Ok(OperandId::zmm_k_z_),
            "k" => Ok(OperandId::k),
            "k{k}" => Ok(OperandId::k_k_),
            "m" => Ok(OperandId::m),
            "m8" => Ok(OperandId::m8),
            "m16" => Ok(OperandId::m16),
            "m16{k}{z}" => Ok(OperandId::m16_k_z_),
            "m32" => Ok(OperandId::m32),
            "m32{k}" => Ok(OperandId::m32_k_),
            "m32{k}{z}" => Ok(OperandId::m32_k_z_),
            "m64" => Ok(OperandId::m64),
            "m64{k}" => Ok(OperandId::m64_k_),
            "m64{k}{z}" => Ok(OperandId::m64_k_z_),
            "m80" => Ok(OperandId::m80),
            "m128" => Ok(OperandId::m128),
            "m128{k}{z}" => Ok(OperandId::m128_k_z_),
            "m256" => Ok(OperandId::m256),
            "m256{k}{z}" => Ok(OperandId::m256_k_z_),
            "m512" => Ok(OperandId::m512),
            "m512{k}{z}" => Ok(OperandId::m512_k_z_),
            "m64/m32bcst" => Ok(OperandId::m64__m32bcst),
            "m128/m32bcst" => Ok(OperandId::m128__m32bcst),
            "m256/m32bcst" => Ok(OperandId::m256__m32bcst),
            "m512/m32bcst" => Ok(OperandId::m512__m32bcst),
            "m128/m64bcst" => Ok(OperandId::m128__m64bcst),
            "m256/m64bcst" => Ok(OperandId::m256__m64bcst),
            "m512/m64bcst" => Ok(OperandId::m512__m64bcst),
            "moffs32" => Ok(OperandId::moffs32),
            "moffs64" => Ok(OperandId::moffs64),
            "vm32x" => Ok(OperandId::vm32x),
            "vm32x{k}" => Ok(OperandId::vm32x_k_),
            "vm32y" => Ok(OperandId::vm32y),
            "vm32y{k}" => Ok(OperandId::vm32y_k_),
            "vm32z" => Ok(OperandId::vm32z),
            "vm32z{k}" => Ok(OperandId::vm32z_k_),
            "vm64x" => Ok(OperandId::vm64x),
            "vm64x{k}" => Ok(OperandId::vm64x_k_),
            "vm64y" => Ok(OperandId::vm64y),
            "vm64y{k}" => Ok(OperandId::vm64y_k_),
            "vm64z" => Ok(OperandId::vm64z),
            "vm64z{k}" => Ok(OperandId::vm64z_k_),
            "{sae}" => Ok(OperandId::_sae_),
            "{er}" => Ok(OperandId::_er_),
            _ => {
                println!("{:?}", s);
                Err(ParseInsError::new("Unknown operand type"))
            }

        }
    }
}

#[derive(Clone, Debug)]
pub struct Prefix {
    pub mandatory: bool,
    pub byte: u8,
}

impl Prefix {
    pub fn new() -> Prefix {
        Prefix {
            mandatory: false,
            byte: 0x00,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Bit {
    Zero,
    One,
    NONE,
}

impl FromStr for Bit {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Bit::Zero),
            "1" => Ok(Bit::One),
            _ => Err(ParseInsError::new("Invalid bit")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BitRef {
    Zero,
    One,
    Ref(u8),
    NONE,
}

impl FromStr for BitRef {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(BitRef::Zero),
            "1" => Ok(BitRef::One),
            "#0" => Ok(BitRef::Ref(0)),
            "#1" => Ok(BitRef::Ref(1)),
            "#2" => Ok(BitRef::Ref(2)),
            "#3" => Ok(BitRef::Ref(3)),
            _ => {
                println!("{:?}", s);
                Err(ParseInsError::new("Invalid register reference in bit"))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum ZeroRef {
    Zero,
    Ref(u8),
    NONE,
    // hacked add on
    EVEX_b_ONE,
}

impl FromStr for ZeroRef {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ZeroRef::Zero),
            "000" => Ok(ZeroRef::Zero),
            "0000" => Ok(ZeroRef::Zero),
            "#0" => Ok(ZeroRef::Ref(0)),
            "#1" => Ok(ZeroRef::Ref(1)),
            "#2" => Ok(ZeroRef::Ref(2)),
            "#3" => Ok(ZeroRef::Ref(3)),
            _ => {
                println!("{:?}", s);
                Err(ParseInsError::new("Invalid ZeroRef value"))
            }
        }
    }
}


#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct REX {
    pub mandatory: bool,
    pub W: Bit,
    pub R: BitRef,
    pub B: BitRef,
    pub X: BitRef,
}

impl REX {
    pub fn new() -> REX {
        REX {
            mandatory: false,
            W: Bit::NONE,
            R: BitRef::NONE,
            B: BitRef::NONE,
            X: BitRef::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub enum VEXType {
    VEX,
    XOP,
    NONE,
}

impl FromStr for VEXType {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VEX" => Ok(VEXType::VEX),
            "XOP" => Ok(VEXType::XOP),
            _ => Err(ParseInsError::new("Invalid VEX type")),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct VEX {
    pub id: VEXType,
    pub mmmmm: u8,
    pub pp: u8,
    pub W: Bit,
    pub L: Bit,
    pub R: BitRef,
    pub B: BitRef,
    pub X: BitRef,
    pub vvvv: ZeroRef,
}

impl VEX {
    pub fn new() -> VEX {
        VEX {
            id: VEXType::NONE,
            mmmmm: 0,
            pp: 0,
            W: Bit::NONE,
            L: Bit::NONE,
            R: BitRef::NONE,
            B: BitRef::NONE,
            X: BitRef::NONE,
            vvvv: ZeroRef::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub enum LLBitRef {
    Zero,
    One,
    Two,
    LastRef(u8),
    NONE,
}

impl FromStr for LLBitRef {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "00" => Ok(LLBitRef::Zero),
            "01" => Ok(LLBitRef::One),
            "10" => Ok(LLBitRef::Two),
            "#2" => Ok(LLBitRef::LastRef(2)),
            "#3" => Ok(LLBitRef::LastRef(3)),
            _ => Err(ParseInsError::new("Invalid VEX type")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum NoneRef {
    NONE,
    Ref(u8),
}

impl FromStr for NoneRef {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#0" => Ok(NoneRef::Ref(0)),
            "#1" => Ok(NoneRef::Ref(1)),
            "#2" => Ok(NoneRef::Ref(2)),
            "#3" => Ok(NoneRef::Ref(3)),
            "#4" => Ok(NoneRef::Ref(4)),
            _ => {
                println!("{:?}", s);
                Err(ParseInsError::new("Invalid NoneRef reference value"))
            }
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct EVEX {
    pub mm: u8,
    pub pp: u8,
    pub W: Bit,
    pub LL: LLBitRef,
    pub RR: NoneRef,
    pub B: NoneRef,
    pub X: NoneRef,
    pub vvvv: ZeroRef,
    pub V: ZeroRef,
    pub b: ZeroRef,
    pub aaa: ZeroRef,
    pub z: ZeroRef,
    pub disp8xN: u8,
}

impl EVEX {
    pub fn new() -> EVEX {
        EVEX {
            mm: 0,
            pp: 0,
            W: Bit::NONE,
            LL: LLBitRef::NONE,
            RR: NoneRef::NONE,
            B: NoneRef::NONE,
            X: NoneRef::NONE,
            vvvv: ZeroRef::NONE,
            V: ZeroRef::NONE,
            b: ZeroRef::NONE,
            aaa: ZeroRef::NONE,
            z: ZeroRef::NONE,
            disp8xN: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Opcode {
    pub byte: u8,
    pub addend: NoneRef,
}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            byte: 0,
            addend: NoneRef::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AddressMode {
    Two,
    Ref(u8),
    NONE,
}

impl FromStr for AddressMode {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "11" => Ok(AddressMode::Two),
            "#0" => Ok(AddressMode::Ref(0)),
            "#1" => Ok(AddressMode::Ref(1)),
            "#2" => Ok(AddressMode::Ref(2)),
            "#3" => Ok(AddressMode::Ref(3)),
            _ => {
                println!("{:?}", s);
                Err(ParseInsError::new("Invalid AddressMode reference value"))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum IntOrRef {
    Extension(u8),
    Ref(u8),
    NONE,
}

impl FromStr for IntOrRef {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match u8::from_str(s) {
            Ok(res) => Ok(IntOrRef::Extension(res)),
            Err(_) => {
                match s {
                    "#0" => Ok(IntOrRef::Ref(0)),
                    "#1" => Ok(IntOrRef::Ref(1)),
                    "#2" => Ok(IntOrRef::Ref(2)),
                    "#3" => Ok(IntOrRef::Ref(3)),
                    "#4" => Ok(IntOrRef::Ref(4)),
                    _ => {
                        println!("{:?}", s);
                        Err(ParseInsError::new("Invalid IntOrRef value"))
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModRM {
    pub mode: AddressMode,
    pub rm: NoneRef,
    pub reg: IntOrRef,
}

impl ModRM {
    pub fn new() -> ModRM {
        ModRM {
            mode: AddressMode::NONE,
            rm: NoneRef::NONE,
            reg: IntOrRef::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RegisterByte {
    pub register: NoneRef,
    pub payload: NoneRef,
}

impl RegisterByte {
    pub fn new() -> RegisterByte {
        RegisterByte {
            register: NoneRef::NONE,
            payload: NoneRef::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Immediate {
    pub size: u8,
    pub value: IntOrRef,
}

impl Immediate {
    pub fn new() -> Immediate {
        Immediate {
            size: 0,
            value: IntOrRef::NONE,
        }
    }
}


#[derive(Clone, Debug)]
pub struct CodeOffset {
    pub size: u8,
    pub value: NoneRef,
}

impl CodeOffset {
    pub fn new() -> CodeOffset {
        CodeOffset {
            size: 0,
            value: NoneRef::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DataOffset {
    pub size: u8,
    pub value: NoneRef,
}

impl DataOffset {
    pub fn new() -> DataOffset {
        DataOffset {
            size: 0,
            value: NoneRef::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ISA {
    CPUID = 1,
    RDTSC = 5,
    RDTSCP = 6,
    CMOV = 20,
    MOVBE = 99,
    POPCNT = 100,
    LZCNT = 101,
    TBM = 102,
    BMI = 103,
    BMI2 = 104,
    ADX = 105,
    MMX = 30,
    MMXPLUS = 31,
    FEMMS = 40,
    NOW3D = 41,
    NOW3DPLUS = 42,
    SSE = 50,
    SSE2 = 51,
    SSE3 = 52,
    SSSE3 = 53,
    SSE4A = 54,
    SSE41 = 55,
    SSE42 = 56,
    FMA3 = 60,
    FMA4 = 61,
    XOP = 62,
    F16C = 63,
    AVX = 70,
    AVX2 = 71,
    AVX512F = 72,
    AVX512BW = 73,
    AVX512DQ = 74,
    AVX512VL = 75,
    AVX512PF = 76,
    AVX512ER = 77,
    AVX512CD = 78,
    AVX512VBMI = 79,
    AVX512IFMA = 80,
    RDRAND = 85,
    RDSEED = 86,
    PCLMULQDQ = 90,
    AES = 91,
    SHA = 92,
    UNSUPPORTED = 200,
    PRFCHW = 201,
    PREFETCHWT1 = 202,
}

impl FromStr for ISA {
    type Err = ParseInsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CPUID" => Ok(ISA::CPUID),
            "RDTSC" => Ok(ISA::RDTSC),
            "RDTSCP" => Ok(ISA::RDTSCP),
            "CMOV" => Ok(ISA::CMOV),
            "MOVBE" => Ok(ISA::MOVBE),
            "POPCNT" => Ok(ISA::POPCNT),
            "LZCNT" => Ok(ISA::LZCNT),
            "TBM" => Ok(ISA::TBM),
            "BMI" => Ok(ISA::BMI),
            "BMI2" => Ok(ISA::BMI2),
            "ADX" => Ok(ISA::ADX),
            "MMX" => Ok(ISA::MMX),
            "MMX+" => Ok(ISA::MMXPLUS),
            "FEMMS" => Ok(ISA::FEMMS),
            "3dnow!" => Ok(ISA::NOW3D),
            "3dnow!+" => Ok(ISA::NOW3DPLUS),
            "SSE" => Ok(ISA::SSE),
            "SSE2" => Ok(ISA::SSE2),
            "SSE3" => Ok(ISA::SSE3),
            "SSSE3" => Ok(ISA::SSSE3),
            "SSE4A" => Ok(ISA::SSE4A),
            "SSE4.1" => Ok(ISA::SSE41),
            "SSE4.2" => Ok(ISA::SSE42),
            "FMA3" => Ok(ISA::FMA3),
            "FMA4" => Ok(ISA::FMA4),
            "XOP" => Ok(ISA::XOP),
            "F16C" => Ok(ISA::F16C),
            "AVX" => Ok(ISA::AVX),
            "AVX2" => Ok(ISA::AVX2),
            "AVX512F" => Ok(ISA::AVX512F),
            "AVX512BW" => Ok(ISA::AVX512BW),
            "AVX512DQ" => Ok(ISA::AVX512DQ),
            "AVX512VL" => Ok(ISA::AVX512VL),
            "AVX512PF" => Ok(ISA::AVX512PF),
            "AVX512ER" => Ok(ISA::AVX512ER),
            "AVX512CD" => Ok(ISA::AVX512CD),
            "AVX512VBMI" => Ok(ISA::AVX512VBMI),
            "AVX512IFMA" => Ok(ISA::AVX512IFMA),
            "RDRAND" => Ok(ISA::RDRAND),
            "RDSEED" => Ok(ISA::RDSEED),
            "PCLMULQDQ" => Ok(ISA::PCLMULQDQ),
            "AES" => Ok(ISA::AES),
            "SHA" => Ok(ISA::SHA),
            // Unsupported but will still keep
            "PRFCHW" => Ok(ISA::PRFCHW),
            "PREFETCHWT1" => Ok(ISA::PREFETCHWT1),
            _ => Err(ParseInsError::new("Invalid ISA")),
        }
    }
}
