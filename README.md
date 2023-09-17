# Chart Charm

## Prerequisites

```sh
# System dependencies
# webkit2gtk

# Tauri CLI
cargo install --locked tauri-cli

# WASM target
rustup target add wasm32-unknown-unknown

# Trunk WASM bundler
cargo install --locked trunk

# `wasm-bindgen` for Apple M1 chips (required by Trunk)
cargo install --locked wasm-bindgen-cli

# `esbuild` as dependency of `tauri-sys` crate (used in UI)
npm install --global --save-exact esbuild

npm install --global tailwindcss
```

# Environment Variables

| Name       | Description                                                     | Default Value |
| ---------- | --------------------------------------------------------------- | ------------- |
| `LOGLEVEL` | Sets the log level for the program(INFO,WARN,ERROR,DEBUG,TRACE) | `Info`        |

# Running The App in Dev Mode

```sh
# Build and develop for desktop
cargo tauri dev

# Build and release for desktop
cargo tauri build
```

# License

    This file is part of ChartCharm.

    ChartCharm is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

    ChartCharm is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License along with ChartCharm. If not, see <https://www.gnu.org/licenses/>.
