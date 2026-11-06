// lib.rs - Universal WebAssembly Android Runtime (Java + ARM64)
use wasm_bindgen::prelude::*;

// 外部モジュールの宣言
mod dex_processor;
mod arm64_cpu;

use dex_processor::DexProcessor;
use arm64_cpu::PracticalARM64CPU;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // JS側のCanvas/WebGLへ画面を描画するためのブリッジ関数
    #[wasm_bindgen(js_name = jsRenderScreen)]
    fn js_render_screen(ptr: *const u8, size: usize);
}

// 統合エンジン本体
#[wasm_bindgen]
pub struct UniversalAndroidEngine {
    dex_engine: DexProcessor,
    arm64_core: PracticalARM64CPU,
    screen_buffer: Vec<u8>,
}

#[wasm_bindgen]
impl UniversalAndroidEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("[Universal Engine] Initializing Java + ARM64 WASM Core... 🚀");
        Self {
            dex_engine: DexProcessor::new(),
            arm64_core: PracticalARM64CPU::new(),
            screen_buffer: vec![0; 1280 * 720 * 4], // 720p RGBAフレームバッファ
        }
    }

    // 1. APKSから抽出した classes.dex をロードして解析
    pub fn load_and_parse_dex(&mut self, dex_bytes: &[u8]) -> bool {
        log("[Universal Engine] Loading classes.dex into Dalvik VM layer...");
        self.dex_engine.load_dex_bytes(dex_bytes)
    }

    // 2. APKSから抽出したネイティブライブラリ (.so) の機械語をセットして実行
    pub fn step_arm64_instruction(&mut self, instruction: u32) {
        // ARM64の1命令をCPUコアで実行
        self.arm64_core.step(instruction);
    }

    // 3. 描画フレームをJavaScript側（Canvas）へ転送
    pub fn flush_graphics_to_js(&self) {
        unsafe {
            js_render_screen(self.screen_buffer.as_ptr(), self.screen_buffer.len());
        }
    }

    // 4. CPUのレジスタ状態をJS側にデバッグ出力（状態確認用）
    pub fn get_cpu_register(&self, index: usize) -> u64 {
        self.arm64_core.get_register(index)
    }
}
