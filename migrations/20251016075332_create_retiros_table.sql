-- Tabla de retiros para gestionar eventos
CREATE TABLE retiros (
    id TEXT PRIMARY KEY NOT NULL,
    nombre TEXT NOT NULL,
    descripcion TEXT,
    fecha_inicio TEXT NOT NULL, -- ISO 8601 format
    fecha_fin TEXT NOT NULL,    -- ISO 8601 format
    ubicacion TEXT,
    numero_participantes INTEGER NOT NULL CHECK (numero_participantes > 0),
    estado TEXT NOT NULL CHECK (estado IN ('Planificacion', 'Activo', 'Finalizado')) DEFAULT 'Planificacion',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Índices para optimizar consultas
CREATE INDEX idx_retiros_estado ON retiros(estado);
CREATE INDEX idx_retiros_fecha_inicio ON retiros(fecha_inicio);
CREATE INDEX idx_retiros_nombre ON retiros(nombre);

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER update_retiros_updated_at 
    AFTER UPDATE ON retiros
    FOR EACH ROW
BEGIN
    UPDATE retiros SET updated_at = datetime('now') WHERE id = NEW.id;
END;