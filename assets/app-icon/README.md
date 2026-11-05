# Icono de @Pictures

Sistema de logos de la familia Aify (opción C, 2026-10-30): cada app es una teja con su inicial
en Super Jello, la tipografía de Aify. @Pictures es una «P» blanca sobre el azul de Aify
(`#1D5FE0`) con un punto lima (`#D7FF3D`). La «P» va como trazado vectorial (sacado de la fuente),
así que el SVG no necesita la tipografía instalada. Teja de 512 unidades con `rx=112`, como el icono original de PhotoCraft, para que
encaje en las mismas plantillas de macOS, Windows y Linux.

`photocraft.svg` es el original; `photocraft-1024.png` y `hicolor/` salen de él. Los nombres
de archivo se mantienen para no tocar el código que los carga. `photocraft.ico` (Windows) y
`photocraft.icns` (Mac, con el margen de la rejilla de Apple: la teja a 824 de 1024 px) salen de
`photocraft-1024.png` (2026-11-05); `packaging/icons.sh` los vuelve a sacar del SVG si hace falta.
