//! Data conversion among computer vision libraries.
//!
//! # Version Selection
//!
//! This crate supports multiple dependency versions to choose from.
//! The choices of dependency versions are named accordingly as Cargo features.
//! For example, the feature `nalgebra_0-30` enables nalgebra 0.30.x.
//! It allows to list crate version selections in `Cargo.toml`.
//!
//! ```toml
//! [dependencies.cv-convert]
//! version = 'x.y.z'
//! features = [
//!     'image_0-23',
//!     'opencv_0-62',
//!     'tch_0-6',
//!     'nalgebra_0-30',
//!     'ndarray_0-15',
//! ]
//! ```
//!
//! Enable `full` feature if you wish to enable all crates with up-to-date versions.
//!
//! # Traits
//!
//! The traits [FromCv] and [IntoCv] provide `.from_cv()` and `.into_cv()`, and
//! traits [TryFromCv] and [TryIntoCv] provide `.try_from_cv()` and `.try_into_cv()` methods respectively.
//! Just like std's [From], [Into], [TryFromCv] and [TryIntoCv].
//!
//! ```rust
//! # use cv_convert::{nalgebra, opencv};
//! use cv_convert::{FromCv, IntoCv, TryFromCv, TryIntoCv};
//! use nalgebra as na;
//! use opencv as cv;
//!
//! // FromCv
//! let cv_point = cv::core::Point2d::new(1.0, 3.0);
//! let na_points = na::Point2::<f64>::from_cv(&cv_point);
//!
//! // IntoCv
//! let cv_point = cv::core::Point2d::new(1.0, 3.0);
//! let na_points: na::Point2<f64> = cv_point.into_cv();
//!
//! // TryFromCv
//! let na_mat = na::DMatrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
//! let cv_mat = cv::core::Mat::try_from_cv(&na_mat).unwrap();
//!
//! // TryIntoCv
//! let na_mat = na::DMatrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
//! let cv_mat: cv::core::Mat = na_mat.try_into_cv().unwrap();
//! ```
//! //!
//!
//! # Supported conversions
//!
//! The notations are used for simplicity.
//!
//! - `S -> T` suggests the conversion is defined by non-fallible [FromCv].
//! - `S ->? T` suggests the conversion is defined by fallible [TryFromCv].
//! - `(&)T` means the type can be either owned or borrowed.
//! - `&'a S -> &'a T` suggests that the target type borrows the source type.
//!
//! ## opencv -> opencv
//!
//! - [(&)Mat](opencv::core::Mat) ->? [Point_<T>](opencv::core::Point_)
//! - [(&)Mat](opencv::core::Mat) ->? [Point3_<T>](opencv::core::Point3_)
//! - [(&)Point_<T>](opencv::core::Point_) ->? [Mat](opencv::core::Mat)
//! - [(&)Point3_<T>](opencv::core::Point3_) ->? [Mat](opencv::core::Mat)
//!
//! ## std -> tch
//!
//! - owned/borrowed multi-dimensional [array](array) ->? [Tensor](tch::Tensor)
//!   - [(&)\[T; N\]](array) ->? [Tensor](tch::Tensor)
//!   - [(&)\[\[T; N2\]; N1\]](array) ->? [Tensor](tch::Tensor)
//!   - ... and so on up to 6 dimensions
//!
//! ## tch -> std
//!
//! - &'a [Tensor](tch::Tensor) -> &'a multi-dimensional [array](array)
//!   - &'a [Tensor](tch::Tensor) -> &'a [\[T; N\]](array)
//!   - &'a [Tensor](tch::Tensor) -> &'a [\[\[T; N2\]; N1\]](array)
//!   - ... and so on up to 6 dimensions
//! - [(&)Tensor](tch::Tensor) -> owned multi-dimensional [array](array)
//!   - [(&)Tensor](tch::Tensor) ->? [\[T; N\]](array)
//!   - [(&)Tensor](tch::Tensor) ->? [\[\[T; N2\]; N1\]](array)
//!   - ... and so on up to 6 dimensions
//!
//! ## tch -> ndarray
//!
//! - &[Tensor](tch::Tensor) ->? [Array](ndarray::Array)
//!
//! ## ndarray -> tch
//!
//! - &[Array](ndarray::Array) ->? [Tensor](tch::Tensor)
//!
//! ## image -> tch
//!
//! - [(&)ImageBuffer](image::ImageBuffer) ->? [Tensor](tch::Tensor)
//! - [(&)DynamicImage](image::DynamicImage) ->? [Tensor](tch::Tensor)
//!
//! ## image -> opencv
//!
//! - [(&)ImageBuffer](image::ImageBuffer) ->? [Mat](opencv::core::Mat)
//! - [(&)DynamicImage](image::DynamicImage) ->? [Mat](opencv::core::Mat)
//!
//! ## opencv -> nalgebra
//!
//! - [(&)Mat](opencv::core::Mat) ->? [OMatrix](nalgebra::OMatrix)
//! - [(&)Point_<T>](opencv::core::Point_) -> [Point2<T>](nalgebra::Point2)
//! - [(&)Point3_<T>](opencv::core::Point3_) -> [Point2<T>](nalgebra::Point3)
//! - [(&)OpenCvPose<(&)Point3d>](OpenCvPose) ->? [Isometry3<f64>](nalgebra::Isometry3)
//! - [(&)OpenCvPose<(&)Mat>](OpenCvPose) ->? [Isometry3<f64>](nalgebra::Isometry3)
//!
//! ## nalgebra -> opencv
//!
//! - [(&)OMatrix](nalgebra::OMatrix) ->? [Mat](opencv::core::Mat)
//! - [(&)Point2<T>](nalgebra::Point2) -> [Point_<T>](opencv::core::Point_)
//! - [(&)Point3<T>](nalgebra::Point3) -> [Point3_<T>](opencv::core::Point3_)
//! - [(&)Translation<N, D>](nalgebra::Translation) ->? [Mat](opencv::core::Mat)
//! - [(&)Isometry3<T>](nalgebra::Isometry3) ->? [OpenCvPose<Point3_<T>>](OpenCvPose)
//! - [(&)Isometry3<f64>](nalgebra::Isometry3) ->? [OpenCvPose<Mat>](OpenCvPose)
//! - [(&)Isometry3<f32>](nalgebra::Isometry3) ->? [OpenCvPose<Mat>](OpenCvPose)
//!
//! ## opencv -> tch
//!
//! - [(&)Mat](opencv::core::Mat) ->? [Tensor](tch::Tensor)
//! - [Mat](opencv::core::Mat) ->? [TensorFromMat]
//!
//! ## tch -> opencv
//!
//! - [(&)Tensor](tch::Tensor) ->? [Mat](opencv::core::Mat)
//! - [(&)TensorAsImage](TensorAsImage) ->? [Mat](opencv::core::Mat)
//!
//! # OpenCV
//! If your system requires `opencv/clang-runtime` to build, enable the `opencv_0-62-clang-runtime` feature to solve.
//! Other versions are named accordingly.

mod common;
mod traits;
mod utils;

pub mod prelude {
    pub use crate::traits::{FromCv, IntoCv, TryFromCv, TryIntoCv};
}

pub use traits::*;

use utils::*;

// opencv exports
#[cfg(feature = "opencv_0-63")]
pub use opencv_0_63 as opencv;

// image exports
#[cfg(feature = "image_0-23")]
pub use image_0_23 as image;

// nalgebra exports
#[cfg(feature = "nalgebra_0-30")]
pub use nalgebra_0_30 as nalgebra;

#[cfg(feature = "nalgebra_0-29")]
pub use nalgebra_0_29 as nalgebra;

#[cfg(feature = "nalgebra_0-28")]
pub use nalgebra_0_28 as nalgebra;

#[cfg(feature = "nalgebra_0-27")]
pub use nalgebra_0_27 as nalgebra;

#[cfg(feature = "nalgebra_0-26")]
pub use nalgebra_0_26 as nalgebra;

// ndarray exports
#[cfg(feature = "ndarray_0-15")]
pub use ndarray_0_15 as ndarray;

// tch exports
#[cfg(feature = "tch_0-6")]
pub use tch_0_6 as tch;

#[cfg(feature = "tch_0-5")]
pub use tch_0_5 as tch;

#[cfg(feature = "tch_0-4")]
pub use tch_0_4 as tch;

// modules
has_opencv! {
    mod with_opencv;
    pub use with_opencv::*;
}

has_tch! {
    mod with_tch;
    pub use with_tch::*;
}

has_tch! {
    has_image! {
        mod with_tch_image;
        pub use with_tch_image::*;
    }
}

has_tch! {
    has_ndarray! {
        mod with_tch_ndarray;
        pub use with_tch_ndarray::*;
    }
}

has_image! {
    has_opencv! {
        mod with_opencv_image;
        pub use with_opencv_image::*;
    }
}

has_nalgebra! {
    has_opencv! {
        mod with_opencv_nalgebra;
        pub use with_opencv_nalgebra::*;
    }
}

has_tch! {
    has_opencv! {
        mod with_opencv_tch;
        pub use with_opencv_tch::*;
    }
}
