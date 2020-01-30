#!/bin/bash

rm *.AppImage
rm -rf sand.AppDir
cargo build --release
mkdir -p sand.AppDir

cp target/release/sand sand.AppDir/sand
cp art.png sand.AppDir/art.png
ln -s sand.AppDir/sand AppRun

cd sand.AppDir

echo '[Desktop Entry]' > sand.desktop
echo 'Name=sand' >> sand.desktop
echo 'Exec=sand' >> sand.desktop
echo 'Icon=art' >> sand.desktop
echo 'Type=Application' >> sand.desktop
echo 'Categories=Game;' >> sand.desktop

cd ..
linuxdeployqt-6-x86_64.AppImage sand.AppDir/sand -appimage -executable=sand.AppDir/sand -unsupported-allow-new-glibc
appimagetool-x86_64.AppImage sand.AppDir