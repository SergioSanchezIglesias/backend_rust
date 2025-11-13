# Contexto del Proyecto: Sistema de Gestión Financiera

## 📋 Resumen Ejecutivo

Sistema integral para la gestión financiera de retiros bianuales, desarrollado en Rust. Permite el registro, análisis y visualización de ingresos/gastos de cada retiro con capacidades de comparación histórica entre eventos.

## 🎯 Objetivos del Proyecto

### Objetivo Principal
Crear un sistema completo de gestión financiera para retiros que permita:
- Registro eficiente de transacciones (ingresos/gastos) por retiro
- Análisis y balance automático de cada evento
- Visualización de estadísticas y comparativas entre retiros
- Acceso tanto local (desktop) como remoto (web)

### Objetivos Específicos
1. **Gestión de datos**: Almacenamiento seguro y estructurado en base de datos
2. **Interfaz intuitiva**: Aplicación desktop fácil de usar para entrada de datos
3. **API robusta**: Backend REST para acceso programático a los datos
4. **Visualización web**: Dashboard web para análisis avanzado

## 🏗️ Arquitectura del Sistema

### Componente 1: Aplicación Desktop (Rust)
**Propósito**: Interfaz principal para gestión de datos
**Responsabilidades**:
- Entrada y validación de transacciones
- Visualización básica de balances
- Gestión de retiros (crear, editar, eliminar)
- Conexión directa a base de datos local
- Exportación/importación de datos

**Tecnologías sugeridas**:
- Framework UI: `tauri`
- Base de datos: `SQLite` con `sqlx`
- Validación: `validator`

### Componente 2: Backend API (Rust)
**Propósito**: Servicio web para acceso remoto a datos
**Responsabilidades**:
- Endpoints REST para CRUD de transacciones
- Endpoints de análisis y estadísticas
- Autenticación y autorización
- Validación de datos de entrada
- Documentación API automática

**Tecnologías sugeridas**:
- Framework web: `axum` o `actix-web`
- Base de datos: Compartida con desktop app
- Serialización: `serde` + `serde_json`
- Documentación: `utoipa` (OpenAPI)

### Componente 3: Frontend Web (Angular)
**Propósito**: Dashboard web para análisis avanzado
**Responsabilidades**:
- Visualización de gráficos y estadísticas
- Comparativas entre retiros
- Reportes exportables
- Interfaz responsive

## 📊 Modelo de Datos

### Entidades Principales
1. **Retiro**
   - ID único
   - Nombre/descripción del retiro
   - Fecha de inicio/fin
   - Ubicación
   - Número de participantes
   - Estado (planificación, activo, finalizado)

2. **Transacción**
   - ID único
   - Tipo (ingreso/gasto)
   - Categoría
   - Monto
   - Descripción
   - Fecha
   - Retiro asociado

3. **Categoría**
   - ID único
   - Nombre
   - Tipo (ingreso/gasto)
   - Color (para visualización)

## 🚀 Casos de Uso Principales

1. **Gestión de Retiros**
   - Crear nuevo retiro semestral
   - Configurar categorías de ingresos/gastos específicas
   - Definir ubicación y fechas del evento
   - Cerrar retiro y generar reporte final

2. **Registro de Transacciones**
   - Añadir ingresos (inscripciones, donaciones, patrocinios, etc.)
   - Registrar gastos (alojamiento, comida, materiales, transporte, etc.)
   - Editar/eliminar transacciones existentes
   - Asociar gastos a participantes específicos

3. **Análisis y Reportes**
   - Balance actual del retiro en curso
   - Comparativa con retiros anteriores
   - Gráficos de distribución por categorías
   - Análisis de costo por participante
   - Exportación de reportes (PDF, Excel)

## 🔧 Consideraciones Técnicas

### Base de Datos
- **Tipo**: SQLite para simplicidad y portabilidad
- **Migraciones**: Usar `sqlx-cli` para versionado de esquema
- **Backup**: Implementar sistema de respaldo automático

### Seguridad
- Validación estricta de entrada de datos
- Sanitización de queries (usar prepared statements)
- Logs de auditoría para cambios importantes

### Performance
- Índices apropiados en tablas principales
- Paginación para listados grandes
- Cache de consultas frecuentes

## 📝 Estado Actual del Proyecto

### ✅ Completado

#### 1. Sistema CLI Completo Funcional
- ✅ **Configuración inicial**: Proyecto Rust con dependencias (tokio, sqlx, serde, clap, etc.)
- ✅ **Base de datos**: SQLite con migraciones (`sqlx-cli`) para 3 tablas principales
- ✅ **Modelos de datos**: Retiro, Transacción, Categoría con validación completa
- ✅ **Repositorios**: CRUD completo para todas las entidades
- ✅ **CLI profesional**: Comandos para gestión completa del sistema
  - `categoria`: crear, listar, mostrar, actualizar, eliminar
  - `retiro`: crear, listar, mostrar, actualizar, estado, eliminar, buscar
  - `transaccion`: crear, listar, mostrar, eliminar, balance
- ✅ **Cálculos financieros**: Balance automático, resúmenes por retiro
- ✅ **Interfaz colorida**: Output profesional con `colored`

#### 2. Aplicación Desktop con Tauri (✅ COMPLETA)
- ✅ **Framework Tauri**: Integración completa con feature flag `desktop`
- ✅ **Frontend HTML/CSS/JS**: Interfaz moderna y responsive
- ✅ **Comandos Tauri**: API completa para todas las operaciones CRUD
- ✅ **Dashboard interactivo**: Resumen de retiros activos, balances, estadísticas
- ✅ **Gestión de Retiros**: Listado, creación, edición, eliminación, cambio de estado
- ✅ **Gestión de Categorías**: CRUD completo con filtros por tipo (Ingreso/Gasto)
- ✅ **Gestión de Transacciones**: CRUD completo con filtrado por retiro
- ✅ **UI/UX moderna**: Sidebar de navegación, modales, notificaciones toast, diseño responsive
- ✅ **Sistema de notificaciones**: Feedback visual para todas las operaciones
- ✅ **Validación en frontend**: Formularios con validación antes de enviar

### 🗂️ Estructura de Archivos Actual
```
src/
├── main.rs                    # Entry point (detecta CLI vs Desktop)
├── lib.rs                     # Módulos principales
├── errors.rs                  # Manejo de errores
├── database/
│   ├── mod.rs
│   └── connection.rs          # Pool de conexiones SQLite
├── models/                    # Entidades de datos
│   ├── mod.rs
│   ├── retiro.rs             # Modelo Retiro + validación
│   ├── transaccion.rs        # Modelo Transacción + validación
│   └── categoria.rs          # Modelo Categoría + validación
├── repositories/             # Capa de acceso a datos
│   ├── mod.rs
│   ├── retiro_repository.rs  # CRUD + consultas especializadas
│   ├── transaccion_repository.rs # CRUD + cálculos financieros
│   └── categoria_repository.rs   # CRUD básico
├── cli/                      # Interfaz de línea de comandos
│   ├── mod.rs                # Dispatcher principal
│   ├── commands.rs
│   ├── retiro_commands.rs    # Comandos de retiros
│   ├── transaccion_commands.rs # Comandos de transacciones
│   └── categoria_commands.rs # Comandos de categorías
└── desktop/                  # Aplicación Desktop (Tauri)
    ├── mod.rs                # Configuración de Tauri
    └── commands.rs            # Comandos Tauri (API backend)

dist/                         # Frontend de la aplicación desktop
├── index.html                # HTML principal con todas las secciones
├── styles.css                # Estilos modernos y responsive
└── app.js                    # Lógica JavaScript completa

migrations/                   # Esquema de base de datos
├── 20251016075318_create_categorias_table.sql
├── 20251016075332_create_retiros_table.sql
├── 20251016075336_create_transacciones_table.sql
└── 20251112074947_remove_fecha_from_transacciones.sql

src-tauri/                    # Configuración de Tauri
├── icons/                    # Iconos de la aplicación
└── tauri.conf.json           # Configuración de Tauri

tauri.conf.json               # Configuración principal de Tauri
```

### 🎯 Funcionalidades Implementadas

#### CLI (Línea de Comandos)
- **Gestión completa de categorías** (ingresos/gastos con colores)
- **Gestión completa de retiros** (estados, participantes, fechas)
- **Gestión completa de transacciones** (registro, balance automático)
- **Cálculos financieros** (balance por retiro, resúmenes detallados)
- **Validación robusta** de todos los datos de entrada
- **CLI profesional** con ayuda contextual y colores

#### Desktop App (Tauri)
- **Dashboard interactivo**:
  - Muestra retiro activo con información detallada
  - Balance actual (ingresos, gastos, balance neto)
  - Total de transacciones con promedio por participante
  - Resumen general de retiros (totales, activos, finalizados)
  - Acciones rápidas para crear entidades
- **Gestión de Retiros**:
  - Listado completo en tabla
  - Crear nuevo retiro con modal
  - Editar retiro existente
  - Cambiar estado del retiro (Planificación/Activo/Finalizado)
  - Eliminar retiro con confirmación
- **Gestión de Categorías**:
  - Listado con indicadores de color
  - Filtrado por tipo (Ingreso/Gasto/Todas)
  - Crear nueva categoría con modal
  - Editar categoría existente
  - Eliminar categoría con confirmación
- **Gestión de Transacciones**:
  - Listado filtrado por retiro seleccionado
  - Selector de retiro para filtrar/crear transacciones
  - Crear nueva transacción con modal
  - Eliminar transacción con confirmación
  - Visualización de balance del retiro seleccionado
- **UI/UX**:
  - Navegación por sidebar con secciones
  - Modales para crear/editar entidades
  - Sistema de notificaciones toast
  - Diseño responsive y moderno
  - Estados de carga y feedback visual

### 🔧 Comandos Tauri Implementados

**Categorías:**
- `get_categorias()` - Obtener todas las categorías
- `create_categoria(data)` - Crear nueva categoría
- `update_categoria(id, data)` - Actualizar categoría
- `delete_categoria(id)` - Eliminar categoría

**Retiros:**
- `get_retiros()` - Obtener todos los retiros
- `create_retiro(data)` - Crear nuevo retiro
- `update_retiro(id, data)` - Actualizar retiro
- `update_retiro_estado(id, estado)` - Cambiar estado del retiro
- `delete_retiro(id)` - Eliminar retiro

**Transacciones:**
- `get_transacciones(retiro_id?)` - Obtener transacciones (opcionalmente filtradas por retiro)
- `create_transaccion(data)` - Crear nueva transacción
- `delete_transaccion(id)` - Eliminar transacción

**Estadísticas:**
- `get_balance_retiro(retiro_id)` - Obtener balance detallado de un retiro

### 🚀 Próximos Pasos Sugeridos

1. **API REST** con `axum` para acceso web remoto
2. **Tests unitarios** para asegurar calidad del código
3. **Frontend web** con dashboard y gráficos (Angular/React)
4. **Reportes avanzados** (exportación PDF/CSV)
5. **Mejoras en UI**: Gráficos de distribución, comparativas visuales entre retiros
6. **Funcionalidad de edición de transacciones** (actualmente solo se puede eliminar)

### 📦 Dependencias Principales

- **Core**: `tokio`, `sqlx`, `serde`, `serde_json`
- **Errores**: `thiserror`, `anyhow`
- **Validación**: `validator`
- **Utilidades**: `uuid`, `chrono`, `dotenvy`
- **CLI**: `clap`, `colored`
- **Desktop**: `tauri` (feature flag `desktop`)
- **Logging**: `tracing`, `tracing-subscriber`

---

**Estado**: Sistema CLI y aplicación Desktop completamente funcionales. La UI está completa y operativa. Base sólida para expansión a API REST y frontend web.