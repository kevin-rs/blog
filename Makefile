run:
	dx serve
build:
	dx build --release
ssg:
	PORT=3000 dx build --ssg --release
