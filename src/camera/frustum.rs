// basically ported from c++
// - used as a reference:
//   [ https://github.com/Beastwick18/gltest/blob/main/src/renderer/Frustum.cpp ]
//   - original code: 
//     [ https://gist.github.com/podgorskiy/e698d18879588ada9014768e3e82a644 ]
//     - which uses cube vs frustum intersection code from:
//       [ http://iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm ]
// three layers of stolen code, yay!

use glam::{Vec3A, Vec4, Mat3A, vec3a};

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
const POINT_COUNT: usize = 8;

struct Frustum {
  planes: [Vec4; PLANE_COUNT],
  points: [Vec3A; POINT_COUNT]
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
      Vec3A::from(planes[FrustumPlane::Left as usize]).cross(planes[FrustumPlane::Right as usize].into()),
      Vec3A::from(planes[FrustumPlane::Left as usize]).cross(planes[FrustumPlane::Bottom as usize].into()),
      Vec3A::from(planes[FrustumPlane::Left as usize]).cross(planes[FrustumPlane::Top as usize].into()),
      Vec3A::from(planes[FrustumPlane::Left as usize]).cross(planes[FrustumPlane::Near as usize].into()),
      Vec3A::from(planes[FrustumPlane::Left as usize]).cross(planes[FrustumPlane::Far as usize].into()),
      Vec3A::from(planes[FrustumPlane::Right as usize]).cross(planes[FrustumPlane::Bottom as usize].into()),
      Vec3A::from(planes[FrustumPlane::Right as usize]).cross(planes[FrustumPlane::Top as usize].into()),
      Vec3A::from(planes[FrustumPlane::Right as usize]).cross(planes[FrustumPlane::Near as usize].into()),
      Vec3A::from(planes[FrustumPlane::Right as usize]).cross(planes[FrustumPlane::Far as usize].into()),
      Vec3A::from(planes[FrustumPlane::Bottom as usize]).cross(planes[FrustumPlane::Top as usize].into()),
      Vec3A::from(planes[FrustumPlane::Bottom as usize]).cross(planes[FrustumPlane::Near as usize].into()),
      Vec3A::from(planes[FrustumPlane::Bottom as usize]).cross(planes[FrustumPlane::Far as usize].into()),
      Vec3A::from(planes[FrustumPlane::Top as usize]).cross(planes[FrustumPlane::Near as usize].into()),
      Vec3A::from(planes[FrustumPlane::Top as usize]).cross(planes[FrustumPlane::Far as usize].into()),
      Vec3A::from(planes[FrustumPlane::Near as usize]).cross(planes[FrustumPlane::Far as usize].into()),
    ];

    //compute points
    let points = [
      intersection::<{FrustumPlane::Left as usize},  {FrustumPlane::Bottom as usize}, {FrustumPlane::Near as usize}>(&planes, &crosses),
      intersection::<{FrustumPlane::Left as usize},  {FrustumPlane::Top as usize},    {FrustumPlane::Near as usize}>(&planes, &crosses),
      intersection::<{FrustumPlane::Right as usize}, {FrustumPlane::Bottom as usize}, {FrustumPlane::Near as usize}>(&planes, &crosses),
      intersection::<{FrustumPlane::Right as usize}, {FrustumPlane::Top as usize},    {FrustumPlane::Near as usize}>(&planes, &crosses),
      intersection::<{FrustumPlane::Left as usize},  {FrustumPlane::Bottom as usize}, {FrustumPlane::Far as usize}>(&planes, &crosses),
      intersection::<{FrustumPlane::Left as usize},  {FrustumPlane::Top as usize},    {FrustumPlane::Far as usize}>(&planes, &crosses),
      intersection::<{FrustumPlane::Right as usize}, {FrustumPlane::Bottom as usize}, {FrustumPlane::Far as usize}>(&planes, &crosses),
      intersection::<{FrustumPlane::Right as usize}, {FrustumPlane::Top as usize},    {FrustumPlane::Far as usize}>(&planes, &crosses),
    ];

    Self { planes, points }
  }
}


const fn ij2k<const I: usize, const J: usize>() -> usize {
  I * (9 - I) / 2 + J - 1 
}
fn intersection<const A: usize, const B: usize, const C: usize>(planes: &[Vec4; PLANE_COUNT], crosses: &[Vec3A; PLANE_COMBINATIONS]) -> Vec3A {
	let d = Vec3A::from(planes[A]).dot(crosses[ij2k::<B, C>()]);
	let res = Mat3A::from_cols(
    crosses[ij2k::<B, C>()], 
    crosses[ij2k::<A, C>()], 
    crosses[ij2k::<A, B>()],
  ) * vec3a(planes[A].w, planes[B].w, planes[C].w);
  res * (-1. / d)
}
