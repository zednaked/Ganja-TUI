//! Despeja a arte da planta para as seeds e os dias das fixtures do plugin QML.
//!
//! O plugin `omarchy-ganja` e uma traducao a mao de `src/ascii/art.rs` para
//! JavaScript. As tres camadas de teste de la comparam o plugin com ele mesmo em
//! dois motores (Node e o V4 do QML); nenhuma compara com ESTE codigo, que e a
//! referencia. Este exemplo fecha esse buraco.
//!
//!     cargo run --release --example dump_frames -- <diretorio>
//!
//! Escreve `<seed>-<dia>.txt` no diretorio, no mesmo formato de
//! `omarchy-ganja/test/frames/`: 28 linhas de 70 caracteres, frame 0.

// O crate e so binario (nao ha `lib.rs`), entao o exemplo monta os dois modulos
// de que precisa a partir dos mesmos arquivos que o jogo usa. Sem copia: e o
// codigo de producao, compilado aqui dentro.
#[path = "../src/domain/mod.rs"]
mod domain;
#[path = "../src/ascii/mod.rs"]
mod ascii;

use ascii::get_plant_ascii;
use domain::{GrowthStage, Plant};
use std::fs;
use std::path::PathBuf;

/// As mesmas oito seeds de `omarchy-ganja/test/run.js`, em hexadecimal.
const SEEDS: [&str; 8] = [
    "0000000000000001",
    "00000000deadbeef",
    "123456789abcdef0",
    "ffffffffffffffff",
    "0f1e2d3c4b5a6978",
    "a5a5a5a5a5a5a5a5",
    "00000000000f4240",
    "7fffffffffffffff",
];

/// Um dia por estagio e um por transicao.
const DAYS: [u32; 8] = [1, 5, 15, 30, 46, 53, 70, 90];

fn main() {
    let dir: PathBuf = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "frames-rust".to_string())
        .into();
    fs::create_dir_all(&dir).expect("criar o diretorio de saida");

    let mut n = 0;
    for seed_hex in SEEDS {
        let seed = u64::from_str_radix(seed_hex, 16).expect("seed em hexadecimal");
        for day in DAYS {
            let stage: GrowthStage = Plant::calculate_stage(day);
            let linhas = get_plant_ascii(stage, day, seed, 0);
            let texto = linhas.join("\n") + "\n";
            fs::write(dir.join(format!("{seed_hex}-{day}.txt")), texto).expect("escrever o frame");
            n += 1;
        }
    }
    println!("{n} frames em {}", dir.display());
}
