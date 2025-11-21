# src02 - Sistema de E-Commerce en Rust

Un proyecto Rust funcional y asincrónico que demuestra modelado de dominio, persistencia y patrones de arquitectura limpia para una plataforma de e-commerce. Construido con **Tokio**, **SQLx**, **Serde** y **Clap**.

## 📑 Tabla de Contenidos

1. [Descripción General](#descripción-general)
2. [Estructura del Proyecto](#estructura-del-proyecto)
3. [Requisitos Previos](#requisitos-previos)
4. [Inicio Rápido](#inicio-rápido)
5. [Compilación del Proyecto](#compilación-del-proyecto)
6. [Ejecutar el Proyecto](#ejecutar-el-proyecto)
7. [Pruebas](#pruebas)
8. [Errores Comunes de Compilación & Soluciones](#errores-comunes-de-compilación--soluciones)
9. [Arquitectura y Diseño](#arquitectura-y-diseño)
10. [Documentación de API](#documentación-de-api)
11. [Ejemplos](#ejemplos)
12. [Contribuir](#contribuir)

---

## Descripción General

**src02** es una plantilla lista para producción para construir sistemas de e-commerce en Rust. Demuestra:

- Estilo de **Programación Funcional** con valores inmutables y funciones puras
- Patrones **Async/Await** usando Tokio
- **Persistencia de Bases de Datos** con SQLx (soporte SQLite, fácilmente extensible a PostgreSQL)
- **Modelado de Dominio Type-Safe** usando el sistema de tipos de Rust
- **Gestión de Configuración** vía flags CLI y archivos `.env`
- **Pruebas Exhaustivas** con tests unitarios e integración
- **Código Limpio**: sin código muerto, mínimas advertencias

### Casos de Uso

- Aprender patrones de programación funcional en Rust
- Construir un backend de e-commerce escalable
- Entender operaciones asincrónicas con bases de datos
- Practicar estructuras de datos inmutables y gestión de estado
- Plantilla para microservicios en producción

---

## Estructura del Proyecto

```
src02/
├── Cargo.toml                 # Manifiesto del proyecto y dependencias
├── Makefile                   # Automatización de compilación
├── README.md                  # Documentación en inglés
├── ReadmeES.md                # Este archivo (Español)
├── ReadmeITA.md               # Documentación en italiano
├── .env.example               # Ejemplo de configuración
├── src/
│   ├── lib.rs                 # Raíz de librería, exports de módulos
│   ├── models.rs              # Tipos de dominio: User, Product, Service, Payment
│   ├── catalog.rs             # Consultas de catálogo
│   ├── usage.rs               # Registro de uso y resolución de pagos
│   ├── persistence.rs         # Operaciones de BD con SQLx
│   ├── main.rs                # (deprecado, usar bin/main.rs)
│   └── bin/
│       └── main.rs            # Punto de entrada binario con CLI
├── tests/
│   ├── integration_tests.rs   # Tests de lógica principal
│   ├── catalog_tests.rs       # Tests específicos de catálogo
│   └── persistence_tests.rs   # Tests de persistencia
└── target/                    # Artefactos compilados (generados)
```

---

## Requisitos Previos

### Requeridos

- **Rust 1.91.0 o posterior** — [Instalar Rust](https://rustup.rs/)
- **Cargo** — viene con Rust
- **Make** — para ejecutar targets del Makefile (opcional, pero recomendado)

### Opcionales

- **SQLite 3.x** — para base de datos con archivo (normalmente preinstalado en Linux/macOS)
- **Git** — para control de versiones

### Verificar Instalación

```bash
# Verificar versiones de Rust y Cargo
rustc --version
cargo --version

# Verificar Make (si es necesario)
make --version
```

---

## Inicio Rápido

### 1. Clonar / Configurar el Proyecto

```bash
# Navegar al proyecto
cd /home/dev01/projects/weekly77/app/src02

# Verificar estructura
ls -la
```

### 2. Ver Comandos Disponibles

```bash
make help
```

Mostrará todos los targets disponibles del Makefile.

### 3. Ejecutar la Demo (En Memoria)

```bash
make demo
```

Salida esperada: lista de productos, servicios y registros de uso.

### 4. Ejecutar Pruebas

```bash
make test
```

Los 4 tests de integración deberían pasar.

### 5. Crear e Inicializar una Base de Datos con Archivo

```bash
make run-file-create
```

Crea `shop_demo.db` en la raíz del proyecto.

---

## Compilación del Proyecto

### Compilar (Debug)

```bash
make build
# o directamente:
cargo build
```

**Salida:** artefactos compilados en `target/debug/`

### Compilar (Release - Optimizado)

```bash
make build-release
# o directamente:
cargo build --release
```

**Salida:** binario optimizado en `target/release/src02`

### Limpiar Artefactos de Compilación

```bash
make clean
```

Elimina directorio `target/` y archivos de base de datos.

---

## Ejecutar el Proyecto

### Opción 1: Modo Demo (En Memoria, Sin BD)

```bash
make demo
```

Ejecuta datos de ejemplo en memoria e imprime resultados. Útil para entender el flujo de datos sin configurar base de datos.

### Opción 2: Base de Datos en Memoria

```bash
make run-inmemory
```

Inicializa una base de datos SQLite en memoria y ejecuta la aplicación.

### Opción 3: Base de Datos con Archivo

**Primera vez:**

```bash
make run-file-create
```

Crea `shop_demo.db` e inicializa el esquema.

**Ejecuciones posteriores:**

```bash
make run-file
```

### Opción 4: Usando Variables de Entorno

Crear archivo `.env`:

```bash
echo "DB_URL=sqlite:mi_custom.db" > .env
```

Luego ejecutar:

```bash
cargo run --bin src02 -- --init-db
```

### Opción 5: Argumentos de Línea de Comandos Personalizados

```bash
cargo run --bin src02 -- --help
```

Muestra opciones CLI disponibles.

---

## Pruebas

### Ejecutar Todas las Pruebas

```bash
make test
```

Ejecuta tests unitarios e integración secuencialmente.

### Ejecutar Solo Tests Unitarios

```bash
make unit-test
```

Tests solo del código de librería.

### Ejecutar Solo Tests de Integración

```bash
make integration-test
```

Tests de base de datos y escenarios end-to-end.

### Ejecutar Tests con Salida Detallada

```bash
make test-verbose
```

Muestra salida detallada para cada test.

---

## Errores Comunes de Compilación & Soluciones

### Error 1: `DATABASE_URL` no establecido

**Síntoma:**
```
error: set `DATABASE_URL` to use query macros online
```

**Solución:**
Nuestro proyecto usa funciones `sqlx::query()` en tiempo de ejecución, no macros `query!`. Si migra a consultas verificadas sin conexión, establezca:

```bash
export DATABASE_URL="sqlite:shop_demo.db"
cargo build
```

---

### Error 2: Versión de Rust Muy Antigua

**Síntoma:**
```
error[E0658]: use of unstable feature
```

**Solución:**
Actualizar Rust a 1.91.0 o posterior:

```bash
rustup update stable
rustc --version  # Verificar
```

---

### Error 3: Herramientas de Compilación Faltantes (Windows)

**Síntoma:**
```
error: linker `link.exe` not found
```

**Solución (Windows):**

Instalar Visual C++ build tools:

```bash
# Via Visual Studio Installer or standalone:
https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

Luego recompilar:

```bash
cargo clean
cargo build
```

---

### Error 4: Encabezados de Desarrollo de SQLite Faltantes (Linux)

**Síntoma:**
```
error: failed to run custom build command for `libsqlite3-sys`
```

**Solución (Ubuntu/Debian):**

```bash
sudo apt-get update
sudo apt-get install libsqlite3-dev
cargo clean
cargo build
```

**Solución (Fedora/RHEL):**

```bash
sudo dnf install sqlite-devel
cargo build
```

---

### Error 5: Advertencias de Clippy Durante la Compilación

**Síntoma:**
```
warning: manual implementation of Option::map
```

**Solución:**
Ejecutar formateador y linter:

```bash
make fmt     # Auto-formatea código
make check   # Ejecuta clippy con warnings-as-errors
```

---

## Arquitectura y Diseño

### Modelo de Dominio

El proyecto usa estructuras de datos funcionales e inmutables.

#### **Modelos** (`src/models.rs`)

- **User** — Representa un cliente
- **PaymentMethod** — Enum de tipos de pago (Tarjeta, PayPal, etc.)
- **Product** — Artículo individual comprable
- **Service** — Colección de productos
- **ServiceUsage** — Registro de uso de servicio/producto por usuario

#### **Catálogo** (`src/catalog.rs`)

Operaciones puras y solo de consulta:

- `with_service(service)` — Añade un servicio
- `list_all_products()` — Devuelve todos los productos
- `list_products_for_service(id)` — Devuelve productos de un servicio

#### **Uso** (`src/usage.rs`)

Gestiona registros de uso y resolución de pagos:

- `UsageLog` — Colección inmutable de registros
- `add_usage(usage)` — Retorna nuevo UsageLog con uso añadido
- `resolve_payment_for_usage(user, payment)` — Aplica jerarquía de pago

#### **Persistencia** (`src/persistence.rs`)

Operaciones asincrónicas de base de datos con SQLx:

- `init_db(pool)` — Crea tablas
- `save_user()`, `save_service()`, `save_usage()`
- `get_users()`, `get_services()`, `get_usages_for_user()`

---

## Documentación de API

Generar documentación Rust:

```bash
make doc
```

---

## Ejemplos

Ver sección "Examples" en README.md para ejemplos de código completos.

---

## Contribuir

### Calidad de Código

Antes de hacer commit:

```bash
make fmt      # Formatea código
make check    # Verifica estilo
make test     # Ejecuta tests
```

---

**¡Feliz codificación! 🚀**
