publisher := "slipwayhq"
name := "jsx_transpile"

build configuration="debug": && package
  rm -rf components
  mkdir -p components/{{publisher}}.{{name}}

  slipway wit > wit/slipway.wit

  cd src && cargo build --target wasm32-wasip2 {{ if configuration == "release" { "--release" } else { "" } }}

  cp target/wasm32-wasip2/{{configuration}}/slipway_{{name}}.wasm components/{{publisher}}.{{name}}/run.wasm
  cp src/slipway_component.json components/{{publisher}}.{{name}}

build-ci configuration="debug": (build configuration) && package-ci

package:
  slipway package components/{{publisher}}.{{name}}

package-ci:
  docker run --rm -v "$(pwd)/components":/workspace -w /workspace slipwayhq/slipway:latest slipway package {{publisher}}.{{name}}

test: build
  cargo test

release version:
  git tag -a "{{version}}" -m "Release {{version}}"
  git push origin "{{version}}"
