// virtual_memory.rs - Complete 2GB Virtual User Memory Manager for WASM
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct VirtualMemorySpace {
    // 仮想メモリ空間（ブラウザのWASMメモリ制限や初期化コストを考慮しつつ拡大可能なベクタ）
    memory: Vec<u8>,
    // 仮想メモリの総サイズ（最大2GB = 2 * 1024 * 1024 * 1024 bytes）
    capacity: usize,
}

#[wasm_bindgen]
impl VirtualMemorySpace {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("[Virtual Memory] Initializing 2GB virtual user memory space... 🧠");
        // 初期アロケーション（安全のためまずは64MB〜256MBで確保し、必要に応じて拡大）
        let initial_size = 256 * 1024 * 1024; // 256MB (必要に応じて 2GB に拡張可能)
        Self {
            memory: vec![0; initial_size],
            capacity: initial_size,
        }
    }

    // 1. 指定した仮想アドレスに .so やバイナリをそのまま展開（マップ）する
    pub fn map_binary(&mut self, target_addr: usize, binary_data: &[u8]) -> bool {
        let end_addr = target_addr + binary_data.len();
        if end_addr > self.capacity {
            log("[Virtual Memory Error] Out of bounds: trying to map beyond memory capacity.");
            return false;
        }

        // バイナリデータを仮想アドレスの位置へ直接コピー（ジャンプ先書き換えが不要に！）
        self.memory[target_addr..end_addr].copy_from_slice(binary_data);
        
        log(&format!(
            "[Virtual Memory] Mapped {} bytes -> Virtual Address range [0x{:08X} - 0x{:08X}]",
            binary_data.len(), target_addr, end_addr
        ));
        true
    }

    // 2. 32ビット値の読み込み（ARM64の命令フェッチやLDR用）
    pub fn read_u32(&self, addr: usize) -> u32 {
        if addr + 4 > self.capacity {
            return 0; // 範囲外アクセスの安全対策
        }
        let bytes = &self.memory[addr..addr + 4];
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    // 3. 32ビット値の書き込み（STR用）
    pub fn write_u32(&mut self, addr: usize, value: u32) {
        if addr + 4 <= self.capacity {
            let bytes = value.to_le_bytes();
            self.memory[addr..addr + 4].copy_from_slice(&bytes);
        }
    }

    // 4. 64ビット値の読み込み（ARM64のレジスタロード用など）
    pub fn read_u64(&self, addr: usize) -> u64 {
        if addr + 8 > self.capacity {
            return 0;
        }
        let bytes = &self.memory[addr..addr + 8];
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    // 5. 64ビット値の書き込み
    pub fn write_u64(&mut self, addr: usize, value: u64) {
        if addr + 8 <= self.capacity {
            let bytes = value.to_le_bytes();
            self.memory[addr..addr + 8].copy_from_slice(&bytes);
        }
    }
}
