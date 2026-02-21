icon:
	cd assets && \
	mkdir -p icon.iconset && \
	rsvg-convert -w 16   -h 16   icon.svg -o icon.iconset/icon_16x16.png && \
	rsvg-convert -w 32   -h 32   icon.svg -o icon.iconset/icon_16x16@2x.png && \
	rsvg-convert -w 32   -h 32   icon.svg -o icon.iconset/icon_32x32.png && \
	rsvg-convert -w 64   -h 64   icon.svg -o icon.iconset/icon_32x32@2x.png && \
	rsvg-convert -w 128  -h 128  icon.svg -o icon.iconset/icon_128x128.png && \
	rsvg-convert -w 256  -h 256  icon.svg -o icon.iconset/icon_128x128@2x.png && \
	rsvg-convert -w 256  -h 256  icon.svg -o icon.iconset/icon_256x256.png && \
	rsvg-convert -w 512  -h 512  icon.svg -o icon.iconset/icon_256x256@2x.png && \
	rsvg-convert -w 512  -h 512  icon.svg -o icon.iconset/icon_512x512.png && \
	rsvg-convert -w 1024 -h 1024 icon.svg -o icon.iconset/icon_512x512@2x.png && \
	iconutil -c icns icon.iconset --output icon.icns && \
	rm -rf icon.iconset

bundle:
	dx bundle --release
	cp assets/icon.icns target/dx/GroupCtrl/release/macos/GroupCtrl.app/Contents/Resources/icon.icns

screenshot:
	cd assets && \
	cp screenshot.png screenshot.bak.png && \
	magick screenshot.png -channel A -threshold 99% +channel -trim +repage screenshot.png && \
	magick screenshot.png -bordercolor transparent -border 50x25 screenshot.png
