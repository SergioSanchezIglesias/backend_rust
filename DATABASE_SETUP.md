# Configuración de Base de Datos

## 🗄️ Base de Datos SQLite

Este proyecto usa SQLite como base de datos. El archivo de base de datos (`retiros.db`) no se incluye en el repositorio por razones de seguridad y para evitar conflictos.

## 🚀 Configuración Inicial

### 1. Instalar sqlx-cli (si no lo tienes)

```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

### 2. Crear la base de datos

```bash
# Crear la base de datos vacía
sqlx database create

# Ejecutar las migraciones
sqlx migrate run
```

### 3. Verificar la instalación

```bash
# Ver el estado de las migraciones
sqlx migrate info

# Verificar que las tablas se crearon correctamente
sqlite3 retiros.db ".schema"
```

## 📊 Esquema de Base de Datos

El proyecto incluye las siguientes tablas:

- **categorias**: Tipos de ingresos y gastos
- **retiros**: Información de cada evento/retiro
- **transacciones**: Registro de movimientos financieros

## 🔧 Variables de Entorno

Asegúrate de tener un archivo `.env` con:

```
DATABASE_URL=sqlite:./retiros.db
RUST_LOG=debug
```

## 🧪 Datos de Prueba

Al ejecutar `cargo run`, el sistema creará automáticamente algunas categorías de ejemplo para probar la funcionalidad.

## 📝 Migraciones

Las migraciones se encuentran en el directorio `migrations/` y se ejecutan automáticamente con `sqlx migrate run`.

Para crear una nueva migración:

```bash
sqlx migrate add nombre_de_la_migracion
```
