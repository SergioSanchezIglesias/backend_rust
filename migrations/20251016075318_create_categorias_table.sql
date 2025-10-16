-- Tabla de categorías para clasificar ingresos y gastos
CREATE TABLE categorias (
    id TEXT PRIMARY KEY NOT NULL,
    nombre TEXT NOT NULL,
    tipo TEXT NOT NULL CHECK (tipo IN ('Ingreso', 'Gasto')),
    color TEXT NOT NULL CHECK (LENGTH(color) = 7 AND color LIKE '#%'),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Índices para optimizar consultas
CREATE INDEX idx_categorias_tipo ON categorias(tipo);
CREATE INDEX idx_categorias_nombre ON categorias(nombre);

-- Trigger para actualizar updated_at automáticamente
CREATE TRIGGER update_categorias_updated_at 
    AFTER UPDATE ON categorias
    FOR EACH ROW
BEGIN
    UPDATE categorias SET updated_at = datetime('now') WHERE id = NEW.id;
END;