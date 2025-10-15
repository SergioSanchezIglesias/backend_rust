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

## 📝 Próximos Pasos

1. **Fase 1**: Configuración inicial del proyecto Rust
2. **Fase 2**: Diseño y creación del esquema de base de datos para retiros
3. **Fase 3**: Desarrollo de la aplicación desktop básica con Tauri
4. **Fase 4**: Implementación del backend API
5. **Fase 5**: Desarrollo del frontend web
6. **Fase 6**: Testing e integración completa

---

**Nota para Agentes IA**: Este proyecto está en desarrollo activo. Priorizar buenas prácticas de Rust, código limpio y arquitectura escalable. Usar las librerías recomendadas en AGENTS.md para mantener consistencia.