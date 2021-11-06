use crate::parameter_manager::ParameterManager;
use dsp_lab::utils::math::{x_fade, var_clip};
use dsp_lab::utils::conversion::db_to_gain;
use dsp_lab::traits::Process;
use dsp_lab::core::lin_filter::{DiffFwd, IntegLeaky, DcBlock};
use std::sync::Arc;
use std::collections::VecDeque;

pub struct EffectProcessor {
    sr: f64,
    scale: f64,
    inv_scale: f64,
    xl_z1: f64,
    xr_z1: f64,
    block_l: DcBlock,
    block_r: DcBlock,
    diff_l: DiffFwd,
    diff_r: DiffFwd,
    integ_l: IntegLeaky,
    integ_r: IntegLeaky,
}

const RESOLUTION: usize = 512;

impl EffectProcessor {
    pub fn new() -> Self { 
        return Self {
            sr: 44100.0,
            scale: 1.0,
            inv_scale: 1.0,
            xl_z1: 1.0,
            xr_z1: 1.0,
            block_l: DcBlock::new(),
            block_r: DcBlock::new(),
            diff_l: DiffFwd::new(),
            diff_r: DiffFwd::new(),
            integ_l: IntegLeaky::new(),
            integ_r: IntegLeaky::new(),
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.sr = sr;
        self.scale = sr / 44100.0;
        self.inv_scale = 44100.0 / sr;
        self.block_l.set_sr(sr);
        self.block_r.set_sr(sr);
        self.diff_l.set_sr(sr);
        self.diff_r.set_sr(sr);
        self.integ_l.set_sr(sr);
        self.integ_r.set_sr(sr);
    }

    pub fn process_effects(&mut self, param_mngr: Arc<ParameterManager>, l: f64, r: f64) -> (f64, f64) {
        // === parameter mappings ===
        let mut erase = 1.0 - param_mngr.params[0].filtered.get() as f64 * 0.999;
        erase *= erase * erase;
        let hardness = param_mngr.params[1].filtered.get() as f64 * 0.99;
        let post = db_to_gain((param_mngr.params[2].filtered.get() as f64) * 24.0 - 12.0);
        let dry_wet  = param_mngr.params[3].filtered.get() as f64;
        let dry = if dry_wet <= 0.5 { 1.0 } else { 2.0 - dry_wet * 2.0 };
        let wet = dry_wet * 2.0 - 1.0;
        let erase_comp = var_clip(erase, hardness);     // erase knob gain compensation
        let hard_comp = 1.0 / var_clip(1.0, hardness);  // hardness knob gain compensation

        // === process ===
        let dl = (l - self.xl_z1) / erase * self.scale;
        let dr = (r - self.xr_z1) / erase * self.scale;
        self.xl_z1 = self.xl_z1 * 0.999 + var_clip(dl, hardness) * erase_comp * hard_comp * self.inv_scale;
        self.xr_z1 = self.xr_z1 * 0.999 + var_clip(dr, hardness) * erase_comp * hard_comp * self.inv_scale;
        return ((self.xl_z1 * wet + l * dry) * post, (self.xr_z1 * wet + r * dry) * post);
    }
}