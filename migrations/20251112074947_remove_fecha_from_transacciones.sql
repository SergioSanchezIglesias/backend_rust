-- Eliminar la columna fecha de la tabla transacciones
-- La información temporal se mantendrá en created_at y updated_at

-- Primero eliminar el índice que hace referencia a la columna fecha
DROP INDEX IF EXISTS idx_transacciones_fecha;

-- Luego eliminar la columna fecha
ALTER TABLE transacciones DROP COLUMN fecha;