// basically ported from c++
// - used as a reference:
//   [ https://github.com/Beastwick18/gltest/blob/main/src/renderer/Frustum.cpp ]
//   - original code: 
//     [ https://gist.github.com/podgorskiy/e698d18879588ada9014768e3e82a644 ]
//     - which uses cube vs frustum intersection code from:
//       [ http://iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm ]
// three layers of stolen code, yay!

use glam::{Vec3, Vec4, Vec4Swizzles};

use super::Camera;


#[repr(usize)]
enum FrustumPlane {
  Left,
  Right,
  Bottom,
  Top,
  Near,
  Far,
}
const PLANE_COUNT: usize = 6;
const PLANE_COMBINATIONS: usize = PLANE_COUNT * (PLANE_COUNT - 1) / 2;

struct Frustum {
  planes: [Vec4; PLANE_COUNT],
  crosses: [Vec3; PLANE_COMBINATIONS],
}
impl Frustum {
  pub fn compute(camera: &Camera) -> Self {
    //compute transposed view-projection matrix
    let mat = (camera.perspective_matrix * camera.view_matrix).transpose();

    // compute planes
    let mut planes = [Vec4::default(); PLANE_COUNT];
    planes[FrustumPlane::Left   as usize] = mat.w_axis + mat.x_axis;
    planes[FrustumPlane::Right  as usize] = mat.w_axis - mat.x_axis;
    planes[FrustumPlane::Bottom as usize] = mat.w_axis + mat.y_axis;
    planes[FrustumPlane::Top    as usize] = mat.w_axis - mat.y_axis;
    planes[FrustumPlane::Near   as usize] = mat.w_axis + mat.z_axis;
    planes[FrustumPlane::Far    as usize] = mat.w_axis - mat.z_axis;

    //compute crosses
    let crosses = [
      planes[FrustumPlane::Left as usize].xyz().cross(planes[FrustumPlane::Right as usize].xyz()),
      planes[FrustumPlane::Left as usize].xyz().cross(planes[FrustumPlane::Bottom as usize].xyz()),
      planes[FrustumPlane::Left as usize].xyz().cross(planes[FrustumPlane::Top as usize].xyz()),
      planes[FrustumPlane::Left as usize].xyz().cross(planes[FrustumPlane::Near as usize].xyz()),
      planes[FrustumPlane::Left as usize].xyz().cross(planes[FrustumPlane::Far as usize].xyz()),
      planes[FrustumPlane::Right as usize].xyz().cross(planes[FrustumPlane::Bottom as usize].xyz()),
      planes[FrustumPlane::Right as usize].xyz().cross(planes[FrustumPlane::Top as usize].xyz()),
      planes[FrustumPlane::Right as usize].xyz().cross(planes[FrustumPlane::Near as usize].xyz()),
      planes[FrustumPlane::Right as usize].xyz().cross(planes[FrustumPlane::Far as usize].xyz()),
      planes[FrustumPlane::Bottom as usize].xyz().cross(planes[FrustumPlane::Top as usize].xyz()),
      planes[FrustumPlane::Bottom as usize].xyz().cross(planes[FrustumPlane::Near as usize].xyz()),
      planes[FrustumPlane::Bottom as usize].xyz().cross(planes[FrustumPlane::Far as usize].xyz()),
      planes[FrustumPlane::Top as usize].xyz().cross(planes[FrustumPlane::Near as usize].xyz()),
      planes[FrustumPlane::Top as usize].xyz().cross(planes[FrustumPlane::Far as usize].xyz()),
      planes[FrustumPlane::Near as usize].xyz().cross(planes[FrustumPlane::Far as usize].xyz()),
    ];
    
    Self { planes, crosses }
  }
}
