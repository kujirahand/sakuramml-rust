set TARGET_DIR=.\win-sakuramml-bin

cargo build --release
# copy
mkdir %TARGET_DIR%
copy README.md %TARGET_DIR%\
copy README_ja.md %TARGET_DIR%\
copy target\release\sakuramml.exe %TARGET_DIR%\

echo "OK"
