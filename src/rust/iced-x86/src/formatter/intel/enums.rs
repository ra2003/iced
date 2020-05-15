/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use core::fmt;

// GENERATOR-BEGIN: CtorKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum CtorKind {
	Previous,
	Normal_1,
	Normal_2,
	asz,
	AX,
	AY,
	bcst,
	bnd_1,
	bnd_2,
	DeclareData,
	fpu_ST_STi,
	fpu_STi_ST,
	imul,
	k1,
	k2,
	maskmovq,
	memsize,
	movabs,
	nop,
	os2,
	os3,
	os_bnd,
	CC_1,
	CC_2,
	CC_3,
	os_jcc_a_1,
	os_jcc_a_2,
	os_jcc_a_3,
	os_jcc_b_1,
	os_jcc_b_2,
	os_jcc_b_3,
	os_loopcc,
	os_loop,
	os_mem,
	pclmulqdq,
	pops,
	reg,
	Reg16,
	ST_STi,
	ST1_2,
	ST1_3,
	ST2,
	STi_ST,
	xbegin,
	YA,
	invlpga,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CTOR_KIND: [&str; 46] = [
	"Previous",
	"Normal_1",
	"Normal_2",
	"asz",
	"AX",
	"AY",
	"bcst",
	"bnd_1",
	"bnd_2",
	"DeclareData",
	"fpu_ST_STi",
	"fpu_STi_ST",
	"imul",
	"k1",
	"k2",
	"maskmovq",
	"memsize",
	"movabs",
	"nop",
	"os2",
	"os3",
	"os_bnd",
	"CC_1",
	"CC_2",
	"CC_3",
	"os_jcc_a_1",
	"os_jcc_a_2",
	"os_jcc_a_3",
	"os_jcc_b_1",
	"os_jcc_b_2",
	"os_jcc_b_3",
	"os_loopcc",
	"os_loop",
	"os_mem",
	"pclmulqdq",
	"pops",
	"reg",
	"Reg16",
	"ST_STi",
	"ST1_2",
	"ST1_3",
	"ST2",
	"STi_ST",
	"xbegin",
	"YA",
	"invlpga",
];
impl fmt::Debug for CtorKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CTOR_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for CtorKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CtorKind::Previous
	}
}
// GENERATOR-END: CtorKind

// GENERATOR-BEGIN: SizeOverride
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum SizeOverride {
	None,
	Size16,
	Size32,
	Size64,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_SIZE_OVERRIDE: [&str; 4] = [
	"None",
	"Size16",
	"Size32",
	"Size64",
];
impl fmt::Debug for SizeOverride {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_SIZE_OVERRIDE[*self as usize])?;
		Ok(())
	}
}
impl Default for SizeOverride {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		SizeOverride::None
	}
}
// GENERATOR-END: SizeOverride

// GENERATOR-BEGIN: BranchSizeInfo
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum BranchSizeInfo {
	None,
	Short,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_BRANCH_SIZE_INFO: [&str; 2] = [
	"None",
	"Short",
];
impl fmt::Debug for BranchSizeInfo {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_BRANCH_SIZE_INFO[*self as usize])?;
		Ok(())
	}
}
impl Default for BranchSizeInfo {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		BranchSizeInfo::None
	}
}
// GENERATOR-END: BranchSizeInfo

// GENERATOR-BEGIN: InstrOpInfoFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct InstrOpInfoFlags;
#[allow(dead_code)]
impl InstrOpInfoFlags {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const MEM_SIZE_NOTHING: u32 = 0x0000_0001;
	pub(crate) const SHOW_NO_MEM_SIZE_FORCE_SIZE: u32 = 0x0000_0002;
	pub(crate) const SHOW_MIN_MEM_SIZE_FORCE_SIZE: u32 = 0x0000_0004;
	pub(crate) const BRANCH_SIZE_INFO_SHIFT: u32 = 0x0000_0003;
	pub(crate) const BRANCH_SIZE_INFO_MASK: u32 = 0x0000_0001;
	pub(crate) const BRANCH_SIZE_INFO_SHORT: u32 = 0x0000_0008;
	pub(crate) const SIZE_OVERRIDE_MASK: u32 = 0x0000_0003;
	pub(crate) const OP_SIZE_SHIFT: u32 = 0x0000_0004;
	pub(crate) const OP_SIZE16: u32 = 0x0000_0010;
	pub(crate) const OP_SIZE32: u32 = 0x0000_0020;
	pub(crate) const OP_SIZE64: u32 = 0x0000_0030;
	pub(crate) const ADDR_SIZE_SHIFT: u32 = 0x0000_0006;
	pub(crate) const ADDR_SIZE16: u32 = 0x0000_0040;
	pub(crate) const ADDR_SIZE32: u32 = 0x0000_0080;
	pub(crate) const ADDR_SIZE64: u32 = 0x0000_00C0;
	pub(crate) const IGNORE_OP_MASK: u32 = 0x0000_0100;
	pub(crate) const FAR_MNEMONIC: u32 = 0x0000_0200;
	pub(crate) const JCC_NOT_TAKEN: u32 = 0x0000_0400;
	pub(crate) const JCC_TAKEN: u32 = 0x0000_0800;
	pub(crate) const BND_PREFIX: u32 = 0x0000_1000;
	pub(crate) const IGNORE_INDEX_REG: u32 = 0x0000_2000;
	pub(crate) const IGNORE_SEGMENT_PREFIX: u32 = 0x0000_4000;
	pub(crate) const MNEMONIC_IS_DIRECTIVE: u32 = 0x0000_8000;
}
// GENERATOR-END: InstrOpInfoFlags

// GENERATOR-BEGIN: InstrOpKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum InstrOpKind {
	Register,
	NearBranch16,
	NearBranch32,
	NearBranch64,
	FarBranch16,
	FarBranch32,
	Immediate8,
	Immediate8_2nd,
	Immediate16,
	Immediate32,
	Immediate64,
	Immediate8to16,
	Immediate8to32,
	Immediate8to64,
	Immediate32to64,
	MemorySegSI,
	MemorySegESI,
	MemorySegRSI,
	MemorySegDI,
	MemorySegEDI,
	MemorySegRDI,
	MemoryESDI,
	MemoryESEDI,
	MemoryESRDI,
	Memory64,
	Memory,
	DeclareByte,
	DeclareWord,
	DeclareDword,
	DeclareQword,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_INSTR_OP_KIND: [&str; 30] = [
	"Register",
	"NearBranch16",
	"NearBranch32",
	"NearBranch64",
	"FarBranch16",
	"FarBranch32",
	"Immediate8",
	"Immediate8_2nd",
	"Immediate16",
	"Immediate32",
	"Immediate64",
	"Immediate8to16",
	"Immediate8to32",
	"Immediate8to64",
	"Immediate32to64",
	"MemorySegSI",
	"MemorySegESI",
	"MemorySegRSI",
	"MemorySegDI",
	"MemorySegEDI",
	"MemorySegRDI",
	"MemoryESDI",
	"MemoryESEDI",
	"MemoryESRDI",
	"Memory64",
	"Memory",
	"DeclareByte",
	"DeclareWord",
	"DeclareDword",
	"DeclareQword",
];
impl fmt::Debug for InstrOpKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_INSTR_OP_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for InstrOpKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		InstrOpKind::Register
	}
}
// GENERATOR-END: InstrOpKind
