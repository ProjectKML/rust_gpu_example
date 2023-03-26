#![cfg_attr(target_arch = "spirv", no_std)]

use spirv_std::spirv;

#[spirv(compute(threads(1)))]
pub fn my_compute_shader() {

}