<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/ai.storyteller.photocraft.png" width="112" height="112" alt="Icono de @Pictures">
</p>

<h1 align="center">@Pictures</h1>

<p align="center">
  <b>El editor de imágenes gratis y de código abierto para gestores de redes.</b><br>
  Capas, máscaras, capas de ajuste, estilos, texto, vectores y archivos PSD de verdad, en el navegador.<br>
  <a href="https://pictures.useaify.app">pictures.useaify.app</a> · parte de la familia <a href="https://www.useaify.app/demo">Aify</a>
</p>

## De dónde sale

@Pictures es un **fork de [PhotoCraft](https://github.com/storytold/photocraft)**, el editor de
código abierto escrito desde cero en Rust del ArtCraft Team y su comunidad. Partimos de su trabajo
(licencia MIT o Apache-2.0) y lo vamos mejorando para quien crea contenido para redes:

- **En español y con el azul de Aify**, sin cuentas ni instalaciones: se abre en el navegador.
- **Pensado para Instagram y TikTok**: formatos 4:5, 9:16 y 1:1 y zona segura de reels e historias.
- **Conectado a [Aify Social](https://www.useaify.app)**: lo que creas se programa en el calendario de tu cliente.
- **Con pizarra**: deja referencias e indicaciones, y tu propio Claude las convierte en la pieza.

Seguimos las novedades de PhotoCraft y, cuando una mejora nuestra sirve para todos, la proponemos allí.

## Mejóralo con nosotros

Es de código abierto para que lo mejoremos entre todos los que lo usan:

- **Ideas y fallos**: abre una issue en este repositorio (qué esperabas, qué pasó y una captura).
- **Código**: lee [`AGENTS.md`](AGENTS.md) y [`docs/development.md`](docs/development.md) y manda un PR.
- **Cómo funciona por dentro**: [`docs/`](docs/) y el [README original de PhotoCraft](docs/README-photocraft.md).

```sh
# Versión web (necesita Rust estable, el destino wasm32 y Trunk)
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cd apps/photocraft-web && trunk serve
```

## Licencia y créditos

Como PhotoCraft, @Pictures tiene licencia [MIT](LICENSE-MIT) o [Apache-2.0](LICENSE-APACHE), a
elegir. Los avisos obligatorios están en [NOTICE](NOTICE) y las fuentes, iconos y demás recursos de
terceros, con su autor y su licencia, en [ATTRIBUTION.md](ATTRIBUTION.md).

PhotoCraft es obra del ArtCraft Team y de los colaboradores de PhotoCraft. @Pictures es un
proyecto independiente: no está respaldado por ArtCraft, y el nombre y los logotipos de ArtCraft se
han retirado de este fork, como piden sus condiciones.

<sub>Adobe y Photoshop son marcas de Adobe Inc. @Pictures no está afiliado a Adobe ni respaldado por Adobe.</sub>
