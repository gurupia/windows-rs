#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInkAnalysisInkBullet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisInkDrawing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisInkWord(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisListItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisParagraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisWritingRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalyzer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalyzerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisDrawingKind(pub i32);
impl InkAnalysisDrawingKind {
    pub const Drawing: Self = Self(0i32);
    pub const Circle: Self = Self(1i32);
    pub const Ellipse: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
    pub const IsoscelesTriangle: Self = Self(4i32);
    pub const EquilateralTriangle: Self = Self(5i32);
    pub const RightTriangle: Self = Self(6i32);
    pub const Quadrilateral: Self = Self(7i32);
    pub const Rectangle: Self = Self(8i32);
    pub const Square: Self = Self(9i32);
    pub const Diamond: Self = Self(10i32);
    pub const Trapezoid: Self = Self(11i32);
    pub const Parallelogram: Self = Self(12i32);
    pub const Pentagon: Self = Self(13i32);
    pub const Hexagon: Self = Self(14i32);
}
impl ::core::marker::Copy for InkAnalysisDrawingKind {}
impl ::core::clone::Clone for InkAnalysisDrawingKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkAnalysisInkBullet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisInkDrawing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisInkWord(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisListItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisNodeKind(pub i32);
impl InkAnalysisNodeKind {
    pub const UnclassifiedInk: Self = Self(0i32);
    pub const Root: Self = Self(1i32);
    pub const WritingRegion: Self = Self(2i32);
    pub const Paragraph: Self = Self(3i32);
    pub const Line: Self = Self(4i32);
    pub const InkWord: Self = Self(5i32);
    pub const InkBullet: Self = Self(6i32);
    pub const InkDrawing: Self = Self(7i32);
    pub const ListItem: Self = Self(8i32);
}
impl ::core::marker::Copy for InkAnalysisNodeKind {}
impl ::core::clone::Clone for InkAnalysisNodeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkAnalysisParagraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisStatus(pub i32);
impl InkAnalysisStatus {
    pub const Updated: Self = Self(0i32);
    pub const Unchanged: Self = Self(1i32);
}
impl ::core::marker::Copy for InkAnalysisStatus {}
impl ::core::clone::Clone for InkAnalysisStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkAnalysisStrokeKind(pub i32);
impl InkAnalysisStrokeKind {
    pub const Auto: Self = Self(0i32);
    pub const Writing: Self = Self(1i32);
    pub const Drawing: Self = Self(2i32);
}
impl ::core::marker::Copy for InkAnalysisStrokeKind {}
impl ::core::clone::Clone for InkAnalysisStrokeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkAnalysisWritingRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalyzer(pub *mut ::core::ffi::c_void);
