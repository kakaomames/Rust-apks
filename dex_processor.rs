// dex_processor.rs - Dalvik Executable (.dex) Parser & Bytecode Runner in WASM
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// DEXヘッダの構造体
pub struct DexHeader {
    pub file_size: u32,
    pub string_ids_size: u32,
    pub string_ids_off: u32,
    pub type_ids_size: u32,
    pub type_ids_off: u32,
    pub method_ids_size: u32,
    pub method_ids_off: u32,
    pub class_defs_size: u32,
    pub class_defs_off: u32,
}

#[wasm_bindgen]
pub struct DexProcessor {
    header: Option<DexHeader>,
    raw_data: Vec<u8>,
}

#[wasm_bindgen]
impl DexProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("[DEX Processor] Initialized inside WASM core. ☕");
        Self {
            header: None,
            raw_data: Vec::new(),
        }
    }

    // 1. JS側から渡された .dex のバイト配列をWASMメモリに取り込む
    pub fn load_dex_bytes(&mut self, data: &[u8]) -> bool {
        if data.len() < 112 {
            log("[DEX Error] Provided data is too small to be a valid DEX file.");
            return false;
        }

        // マジックナンバーの検証 ("dex\n035\0" など)
        if &data[0..4] != b"dex\n" {
            log("[DEX Error] Invalid DEX magic number!");
            return false;
        }

        let magic_version = &data[4..8];
        log(&format!("[DEX Processor] Valid DEX detected. Version: {:02X?}", magic_version));

        // バイナリから各種オフセットとサイズをリトルエンディアンで抽出
        let file_size = u32::from_le_bytes(data[32..36].try_into().unwrap());
        let string_ids_size = u32::from_le_bytes(data[56..60].try_into().unwrap());
        let string_ids_off = u32::from_le_bytes(data[60..64].try_into().unwrap());
        let type_ids_size = u32::from_le_bytes(data[70..74].try_into().unwrap()); // 調整位置
        let type_ids_off = u32::from_le_bytes(data[74..78].try_into().unwrap());
        let method_ids_size = u32::from_le_bytes(data[88..92].try_into().unwrap());
        let method_ids_off = u32::from_le_bytes(data[92..96].try_into().unwrap());
        let class_defs_size = u32::from_le_bytes(data[96..100].try_into().unwrap());
        let class_defs_off = u32::from_le_bytes(data[100..104].try_into().unwrap());

        log(&format!(
            "[DEX Info] Size: {}, Strings: {}, Methods: {}, Classes: {}",
            file_size, string_ids_size, method_ids_size, class_defs_size
        ));

        self.header = Some(DexHeader {
            file_size,
            string_ids_size,
            string_ids_off,
            type_ids_size,
            type_ids_off,
            method_ids_size,
            method_ids_off,
            class_defs_size,
            class_defs_off,
        });

        self.raw_data = data.to_vec();
        true
    }

    // 2. Dalvikバイトコード（メソッドの中身）を解釈・実行するシミュレーションループ
    pub fn execute_dalvik_method(&self, bytecode_ptr: usize, length: usize) -> bool {
        if bytecode_ptr + length > self.raw_data.len() {
            log("[DEX Execution Error] Bytecode pointer out of bounds.");
            return false;
        }

        let code = &self.raw_data[bytecode_ptr..bytecode_ptr + length];
        let mut pc = 0;

        log("[DEX VM] Starting Dalvik bytecode execution loop...");

        while pc < code.len() {
            let opcode = code[pc];

            // Dalvikレジスタベースのオぺコードをディスパッチ
            match opcode {
                0x00 => {
                    // nop
                    pc += 1;
                },
                0x01 => {
                    // move vA, vB
                    pc += 2;
                },
                0x12 => {
                    // const/4 vA, #+B (定数代入)
                    pc += 2;
                },
                0x6e => {
                    // invoke-virtual (メソッド呼び出し)
                    pc += 3;
                },
                _ => {
                    // その他のオぺコードは1ワード(2バイト単位)進める
                    pc += 1;
                }
            }
        }

        log("[DEX VM] Bytecode execution finished successfully.");
        true
    }
}
