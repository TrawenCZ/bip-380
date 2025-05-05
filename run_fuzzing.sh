cargo install cargo-fuzz

cargo-fuzz build

cargo-fuzz run derive_key_fuzz -- -dict=fuzz/dict/dict_derive_key.dict -max_total_time=20
cargo-fuzz run key_expression_fuzz -- -dict=fuzz/dict/dict_key_expression.dict -max_total_time=20
cargo-fuzz run script_expression_fuzz -- -dict=fuzz/dict/dict_script_expression.dict -max_total_time=20