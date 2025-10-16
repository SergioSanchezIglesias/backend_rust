-- Tabla de transacciones para registrar ingresos y gastos
CREATE TABLE transacciones (
    id TEXT PRIMARY KEY NOT NULL,
    retiro_id TEXT NOT NULL,
    categoria_id TEXT NOT NULL,
    tipo TEXT NOT NULL CHECK (tipo IN ('Ingreso', 'Gasto')),
    monto REAL NOT NULL CHECK (monto > 0),
    descripcion TEXT NOT NULL,
    fecha TEXT NOT NULL, -- ISO 8601 format
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    -- Claves foráneas
    FOREIGN KEY (retiro_id) REFERENCES retiros(id) ON DELETE CASCADE,
    FOREIGN KEY (categoria_id) REFERENCES categorias(id) ON DELETE RESTRICT
);

-- Índices para optimizar consultas
CREATE INDEX idx_transacciones_retiro_id ON transacciones(retiro_id);
CREATE INDEX idx_transacciones_categoria_id ON transacciones(categoria_id);
CREATE INDEX idx_transacciones_tipo ON transacciones(tipo);
CREATE INDEX idx_transacciones_fecha ON transacciones(fecha);
CREATE INDEX idx_transacciones_monto ON transacciones(monto);

-- Índice compuesto para consultas frecuentes
CREATE INDEX idx_transacciones_retiro_tipo ON transacciones(retiro_id, tipo);

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER update_transacciones_updated_at 
    AFTER UPDATE ON transacciones
    FOR EACH ROW
BEGIN
    UPDATE transacciones SET updated_at = datetime('now') WHERE id = NEW.id;
END;