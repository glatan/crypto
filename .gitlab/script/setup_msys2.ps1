Param($Env)

$FileVersion = $MSYS2_VERSION -split '-' -join ''
wget.exe -nv -O msys2.exe "https://github.com/msys2/msys2-installer/releases/download/$MSYS2_VERSION/msys2-base-x86_64-$FileVersion.sfx.exe"
./msys2.exe -y -oC:\
Remove-Item msys2.exe
$env:CHERE_INVOKING = 'yes'
$env:MSYSTEM = $Env # https://www.msys2.org/docs/environments/
