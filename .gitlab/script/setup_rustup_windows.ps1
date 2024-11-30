Remove-Item Alias:curl
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
./rustup-init.exe -y --profile minimal --default-toolchain $CHANNEL -t $TARGET
$ENV:Path += ";$CI_PROJECT_DIR/cargo/bin"
