// Función helper para obtener la API de Tauri de manera segura
function getTauriAPI() {
    // Método 1: API global moderna
    if (window.__TAURI__ && window.__TAURI__.tauri) {
        return window.__TAURI__.tauri;
    }
    
    // Método 2: API global directa (withGlobalTauri: true)
    if (window.__TAURI__ && window.__TAURI__.invoke) {
        return window.__TAURI__;
    }
    
    // Método 3: Tauri global
    if (window.tauri && window.tauri.invoke) {
        return window.tauri;
    }
    
    throw new Error('Tauri API no está disponible');
}

// Función helper para invoke
async function invoke(command, args = {}) {
    const tauri = getTauriAPI();
    return await tauri.invoke(command, args);
}

// Estado global de la aplicación
let appState = {
    currentSection: 'dashboard',
    retiros: [],
    categorias: [],
    transacciones: [],
    selectedRetiro: null
};

// ============================================================================
// INICIALIZACIÓN
// ============================================================================

// Esperar a que tanto el DOM como Tauri estén listos
async function initializeApp() {
    try {
        // Verificar que Tauri esté disponible
        getTauriAPI();
        
        setupNavigation();
        await loadInitialData();
        showSection('dashboard');
    } catch (error) {
        console.error('Error inicializando la aplicación:', error);
        showToast('Error inicializando la aplicación: ' + getErrorMessage(error), 'error');
        
        // Reintentar después de un momento
        setTimeout(initializeApp, 1000);
    }
}

// Inicializar cuando el DOM y Tauri estén listos
document.addEventListener('DOMContentLoaded', () => {
    let attempts = 0;
    const maxAttempts = 100; // 5 segundos máximo
    
    const checkTauri = () => {
        attempts++;
        
        if (window.__TAURI__ || window.tauri) {
            initializeApp();
        } else if (attempts >= maxAttempts) {
            showToast('Error: No se pudo conectar con la aplicación. Verifica que esté ejecutándose correctamente.', 'error');
        } else {
            setTimeout(checkTauri, 50);
        }
    };
    
    checkTauri();
});

// ============================================================================
// NAVEGACIÓN
// ============================================================================

function setupNavigation() {
    const navButtons = document.querySelectorAll('.nav-button');
    
    navButtons.forEach(button => {
        button.addEventListener('click', () => {
            const section = button.getAttribute('data-section');
            showSection(section);
            
            // Actualizar botones activos
            navButtons.forEach(btn => btn.classList.remove('active'));
            button.classList.add('active');
        });
    });
}

function showSection(sectionName) {
    // Ocultar todas las secciones
    const sections = document.querySelectorAll('.content-section');
    sections.forEach(section => section.classList.remove('active'));
    
    // Mostrar la sección seleccionada
    const targetSection = document.getElementById(sectionName);
    if (targetSection) {
        targetSection.classList.add('active');
        appState.currentSection = sectionName;
        
        // Cargar datos específicos de la sección
        loadSectionData(sectionName);
    }
}

// ============================================================================
// CARGA DE DATOS
// ============================================================================

async function loadInitialData() {
    try {
        showToast('Cargando datos iniciales...', 'info');
        
        // Cargar datos en paralelo
        const [retiros, categorias] = await Promise.all([
            invoke('get_retiros'),
            invoke('get_categorias')
        ]);
        
        appState.retiros = retiros;
        appState.categorias = categorias;
        
        // Seleccionar el primer retiro activo si existe
        const retiroActivo = retiros.find(r => r.estado === 'Activo');
        if (retiroActivo) {
            appState.selectedRetiro = retiroActivo.id;
        }
        
        showToast('Datos cargados correctamente', 'success');
    } catch (error) {
        console.error('Error cargando datos iniciales:', error);
        showToast('Error cargando datos: ' + getErrorMessage(error), 'error');
    }
}

async function loadSectionData(sectionName) {
    switch (sectionName) {
        case 'dashboard':
            await loadDashboard();
            break;
        case 'retiros':
            await loadRetiros();
            break;
        case 'categorias':
            await loadCategorias();
            break;
        case 'transacciones':
            await loadTransacciones();
            break;
    }
}

// ============================================================================
// DASHBOARD
// ============================================================================

async function loadDashboard() {
    try {
        // Cargar balance global (siempre visible, compacto)
        const balanceGlobal = await invoke('get_balance_global');
        const balanceGlobalDiv = document.getElementById('balance-global');
        
        const isPositive = balanceGlobal.balance >= 0;
        balanceGlobalDiv.className = `balance-global-compact ${isPositive ? 'balance-positive-bg' : 'balance-negative-bg'}`;
        
        balanceGlobalDiv.innerHTML = `
            <div class="balance-compact-content">
                <span class="balance-compact-label">Balance Global</span>
                <span class="balance-compact-amount">
                    ${isPositive ? '+' : ''}${balanceGlobal.balance.toFixed(2)}€
                </span>
            </div>
        `;
        
        // Cargar estadísticas administrativas
        const [estadisticas, retirosRecientes] = await Promise.all([
            invoke('get_estadisticas_admin'),
            invoke('get_retiros_finalizados_recientes')
        ]);
        
        // Estadísticas comparativas
        document.getElementById('estadisticas-comparativas').innerHTML = `
            <div class="balance-info">
                <p><strong>Balance promedio:</strong> 
                    <span class="${estadisticas.promedio_balance_por_retiro >= 0 ? 'text-success' : 'text-danger'}">
                        ${estadisticas.promedio_balance_por_retiro.toFixed(2)}€
                    </span>
                </p>
                <p><strong>Ingreso promedio:</strong> 
                    <span class="text-success">${estadisticas.promedio_ingreso_por_retiro.toFixed(2)}€</span>
                </p>
                <p><strong>Gasto promedio:</strong> 
                    <span class="text-danger">${estadisticas.promedio_gasto_por_retiro.toFixed(2)}€</span>
                </p>
                <p><strong>Retiros con datos:</strong> <span>${estadisticas.retiros_con_transacciones}</span></p>
            </div>
        `;
        
        // Top categorías de gastos
        const topCategoriasDiv = document.getElementById('top-categorias');
        if (estadisticas.top_categorias_gastos.length > 0) {
            topCategoriasDiv.innerHTML = `
                <div class="top-categorias-list">
                    ${estadisticas.top_categorias_gastos.map((cat, index) => `
                        <div class="top-categoria-item">
                            <div class="top-categoria-header">
                                <span class="top-categoria-rank">#${index + 1}</span>
                                <div class="color-indicator" style="background-color: ${cat.color}"></div>
                                <span class="top-categoria-name">${cat.nombre}</span>
                            </div>
                            <span class="top-categoria-amount text-danger">${cat.total.toFixed(2)}€</span>
                        </div>
                    `).join('')}
                </div>
            `;
        } else {
            topCategoriasDiv.innerHTML = '<p class="text-muted">No hay datos de gastos aún</p>';
        }
        
        // Retiros finalizados recientes
        const retirosRecientesDiv = document.getElementById('retiros-recientes');
        if (retirosRecientes.length > 0) {
            retirosRecientesDiv.innerHTML = `
                <div class="retiros-recientes-list">
                    ${retirosRecientes.map(retiro => `
                        <div class="retiro-reciente-item">
                            <div class="retiro-reciente-header">
                                <strong>${retiro.nombre}</strong>
                                <small class="text-muted">${retiro.fecha_fin}</small>
                            </div>
                            <div class="retiro-reciente-details">
                                <span>${retiro.numero_participantes} participantes</span>
                                <span class="text-danger">${retiro.total_gastos.toFixed(2)}€</span>
                                <span class="${retiro.balance >= 0 ? 'text-success' : 'text-danger'}">
                                    ${retiro.balance >= 0 ? '+' : ''}${retiro.balance.toFixed(2)}€
                                </span>
                            </div>
                        </div>
                    `).join('')}
                </div>
            `;
        } else {
            retirosRecientesDiv.innerHTML = '<p class="text-muted">No hay retiros finalizados aún</p>';
        }
        
        
    } catch (error) {
        console.error('Error cargando dashboard:', error);
        showToast('Error cargando dashboard: ' + getErrorMessage(error), 'error');
    }
}

// ============================================================================
// GESTIÓN DE RETIROS
// ============================================================================

async function loadRetiros() {
    const tbody = document.querySelector('#retiros-table tbody');
    
    if (appState.retiros.length === 0) {
        tbody.innerHTML = '<tr><td colspan="5">No hay retiros registrados</td></tr>';
        return;
    }
    
    tbody.innerHTML = appState.retiros.map(retiro => `
        <tr>
            <td>
                <strong>${retiro.nombre}</strong>
                ${retiro.descripcion ? `<br><small>${retiro.descripcion}</small>` : ''}
            </td>
            <td>
                <small>
                    ${formatDate(retiro.fecha_inicio)} - ${formatDate(retiro.fecha_fin)}
                </small>
            </td>
            <td>${retiro.numero_participantes}</td>
            <td>
                <span class="estado-badge estado-${retiro.estado.toLowerCase()}">
                    ${retiro.estado}
                </span>
            </td>
            <td>
                <button class="btn btn-small btn-secondary" onclick="editRetiro('${retiro.id}')">
                    ✏️ Editar
                </button>
                <button class="btn btn-small btn-danger" onclick="deleteRetiro('${retiro.id}')">
                    🗑️ Eliminar
                </button>
            </td>
        </tr>
    `).join('');
}

// ============================================================================
// GESTIÓN DE CATEGORÍAS
// ============================================================================

async function loadCategorias() {
    const tbody = document.querySelector('#categorias-table tbody');
    const filter = document.getElementById('categoria-filter').value;
    
    let categoriasToShow = appState.categorias;
    if (filter) {
        categoriasToShow = appState.categorias.filter(c => c.tipo === filter);
    }
    
    if (categoriasToShow.length === 0) {
        tbody.innerHTML = '<tr><td colspan="4">No hay categorías registradas</td></tr>';
        return;
    }
    
    tbody.innerHTML = categoriasToShow.map(categoria => `
        <tr>
            <td>
                <div class="color-indicator" style="background-color: ${categoria.color}"></div>
            </td>
            <td><strong>${categoria.nombre}</strong></td>
            <td>
                <span class="tipo-badge tipo-${categoria.tipo.toLowerCase()}">
                    ${categoria.tipo}
                </span>
            </td>
            <td>
                <button class="btn btn-small btn-secondary" onclick="editCategoria('${categoria.id}')">
                    ✏️ Editar
                </button>
                <button class="btn btn-small btn-danger" onclick="deleteCategoria('${categoria.id}')">
                    🗑️ Eliminar
                </button>
            </td>
        </tr>
    `).join('');
}

// ============================================================================
// GESTIÓN DE TRANSACCIONES
// ============================================================================

async function loadTransacciones() {
    // Si no hay retiro seleccionado, seleccionar el retiro activo automáticamente
    if (!appState.selectedRetiro) {
        const retiroActivo = appState.retiros.find(r => r.estado === 'Activo');
        if (retiroActivo) {
            appState.selectedRetiro = retiroActivo.id;
        }
    }
    
    // Cargar retiros en el filtro
    const retiroFilter = document.getElementById('transaccion-retiro-filter');
    retiroFilter.innerHTML = '<option value="">⚠️ Seleccionar retiro para ver/crear transacciones...</option>' +
        appState.retiros.map(retiro => 
            `<option value="${retiro.id}" ${retiro.id === appState.selectedRetiro ? 'selected' : ''}>
                ${retiro.nombre} (${retiro.estado}) ${retiro.estado === 'Activo' ? '🟢' : ''}
            </option>`
        ).join('');
    
    // Configurar evento del filtro
    retiroFilter.onchange = async (e) => {
        appState.selectedRetiro = e.target.value;
        updateRetiroInfo();
        await loadTransaccionesTable();
    };
    
    updateRetiroInfo();
    await loadTransaccionesTable();
}

function updateRetiroInfo() {
    const retiroInfo = document.getElementById('retiro-info');
    const retiroBadge = document.querySelector('.retiro-badge');
    
    if (appState.selectedRetiro) {
        const retiro = appState.retiros.find(r => r.id === appState.selectedRetiro);
        if (retiro) {
            retiroInfo.style.display = 'block';
            retiroBadge.textContent = `${retiro.nombre} - ${retiro.estado}`;
            retiroBadge.className = `retiro-badge ${retiro.estado.toLowerCase()}`;
        }
    } else {
        retiroInfo.style.display = 'none';
    }
}

async function loadTransaccionesTable() {
    const tbody = document.querySelector('#transacciones-table tbody');
    
    if (!appState.selectedRetiro) {
        tbody.innerHTML = '<tr><td colspan="6" class="no-retiro-selected">⚠️ Selecciona un retiro para ver sus transacciones y poder crear nuevas</td></tr>';
        return;
    }
    
    try {
        const transacciones = await invoke('get_transacciones', { 
            retiroId: appState.selectedRetiro 
        });
        
        if (transacciones.length === 0) {
            tbody.innerHTML = '<tr><td colspan="6">No hay transacciones para este retiro</td></tr>';
            return;
        }
        
        tbody.innerHTML = transacciones.map(transaccion => {
            const categoria = appState.categorias.find(c => c.id === transaccion.categoria_id);
            return `
                <tr>
                    <td>${formatDate(transaccion.created_at)}</td>
                    <td><strong>${transaccion.descripcion}</strong></td>
                    <td>
                        ${categoria ? `
                            <div class="color-indicator" style="background-color: ${categoria.color}"></div>
                            ${categoria.nombre}
                        ` : 'Sin categoría'}
                    </td>
                    <td>
                        <span class="tipo-badge tipo-${transaccion.tipo.toLowerCase()}">
                            ${transaccion.tipo}
                        </span>
                    </td>
                    <td class="${transaccion.tipo === 'Ingreso' ? 'text-success' : 'text-danger'}">
                        ${transaccion.tipo === 'Ingreso' ? '+' : '-'}${transaccion.monto.toFixed(2)}€
                    </td>
                    <td>
                        <button class="btn btn-small btn-danger" onclick="deleteTransaccion('${transaccion.id}')">
                            🗑️ Eliminar
                        </button>
                    </td>
                </tr>
            `;
        }).join('');
    } catch (error) {
        console.error('Error cargando transacciones:', error);
        tbody.innerHTML = '<tr><td colspan="6">Error cargando transacciones</td></tr>';
    }
}

// ============================================================================
// UTILIDADES
// ============================================================================

function getErrorMessage(error) {
    if (typeof error === 'string') {
        return error;
    } else if (error && error.message) {
        return error.message;
    } else if (error && typeof error === 'object') {
        return JSON.stringify(error);
    }
    return 'Error desconocido';
}

function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('es-ES', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
    });
}

function showToast(message, type = 'info') {
    // Snackbars deshabilitados - no mostrar notificaciones
}

function showModal(content) {
    const overlay = document.getElementById('modal-overlay');
    const modalContent = document.getElementById('modal-content');
    
    modalContent.innerHTML = content;
    overlay.classList.add('active');
    
    // Cerrar modal al hacer clic fuera
    overlay.onclick = (e) => {
        if (e.target === overlay) {
            hideModal();
        }
    };
}

function hideModal() {
    const overlay = document.getElementById('modal-overlay');
    overlay.classList.remove('active');
}

// ============================================================================
// FUNCIONES PLACEHOLDER PARA MODALES (se implementarán en siguientes pasos)
// ============================================================================

function showCreateRetiroModal() {
    const modalContent = `
        <div class="modal-header">
            <h3>Nuevo Retiro</h3>
            <button class="close-modal" onclick="hideModal()">×</button>
        </div>
        <form id="create-retiro-form">
            <div class="form-group">
                <label for="retiro-nombre">Nombre *</label>
                <input type="text" id="retiro-nombre" required maxlength="200">
            </div>
            <div class="form-group">
                <label for="retiro-descripcion">Descripción</label>
                <textarea id="retiro-descripcion" maxlength="500" placeholder="Descripción opcional del retiro"></textarea>
            </div>
            <div class="form-group">
                <label for="retiro-fecha-inicio">Fecha de Inicio *</label>
                <input type="datetime-local" id="retiro-fecha-inicio" required>
            </div>
            <div class="form-group">
                <label for="retiro-fecha-fin">Fecha de Fin *</label>
                <input type="datetime-local" id="retiro-fecha-fin" required>
            </div>
            <div class="form-group">
                <label for="retiro-ubicacion">Ubicación</label>
                <input type="text" id="retiro-ubicacion" maxlength="200" placeholder="Ubicación del retiro">
            </div>
            <div class="form-group">
                <label for="retiro-participantes">Número de Participantes *</label>
                <input type="number" id="retiro-participantes" min="1" required>
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="hideModal()">
                    Cancelar
                </button>
                <button type="submit" class="btn btn-primary">
                    Crear Retiro
                </button>
            </div>
        </form>
    `;
    
    showModal(modalContent);
    
    // Configurar fechas por defecto
    const now = new Date();
    const tomorrow = new Date(now);
    tomorrow.setDate(tomorrow.getDate() + 1);
    const nextWeek = new Date(now);
    nextWeek.setDate(nextWeek.getDate() + 7);
    
    document.getElementById('retiro-fecha-inicio').value = tomorrow.toISOString().slice(0, 16);
    document.getElementById('retiro-fecha-fin').value = nextWeek.toISOString().slice(0, 16);
    
    // Configurar el formulario
    document.getElementById('create-retiro-form').onsubmit = async (e) => {
        e.preventDefault();
        await createRetiro();
    };
}

function showCreateCategoriaModal() {
    const modalContent = `
        <div class="modal-header">
            <h3>Nueva Categoría</h3>
            <button class="close-modal" onclick="hideModal()">×</button>
        </div>
        <form id="create-categoria-form">
            <div class="form-group">
                <label for="categoria-nombre">Nombre *</label>
                <input type="text" id="categoria-nombre" required maxlength="100">
            </div>
            <div class="form-group">
                <label for="categoria-tipo">Tipo *</label>
                <select id="categoria-tipo" required>
                    <option value="">Seleccionar tipo...</option>
                    <option value="Ingreso">Ingreso</option>
                    <option value="Gasto">Gasto</option>
                </select>
            </div>
            <div class="form-group">
                <label for="categoria-color">Color *</label>
                <input type="color" id="categoria-color" value="#3b82f6" required>
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="hideModal()">
                    Cancelar
                </button>
                <button type="submit" class="btn btn-primary">
                    Crear Categoría
                </button>
            </div>
        </form>
    `;
    
    showModal(modalContent);
    
    // Configurar el formulario
    document.getElementById('create-categoria-form').onsubmit = async (e) => {
        e.preventDefault();
        await createCategoria();
    };
}

function showCreateTransaccionModal() {
    if (!appState.selectedRetiro) {
        showToast('Primero selecciona un retiro para crear transacciones', 'warning');
        return;
    }
    
    const retiroSeleccionado = appState.retiros.find(r => r.id === appState.selectedRetiro);
    
    const modalContent = `
        <div class="modal-header">
            <h3>Nueva Transacción</h3>
            <button class="close-modal" onclick="hideModal()">×</button>
        </div>
        <div class="retiro-context">
            <p><strong>Retiro:</strong> <span class="retiro-badge ${retiroSeleccionado.estado.toLowerCase()}">${retiroSeleccionado.nombre}</span></p>
        </div>
        <form id="create-transaccion-form">
            <div class="form-group">
                <label for="transaccion-tipo">Tipo *</label>
                <select id="transaccion-tipo" required>
                    <option value="">Seleccionar tipo...</option>
                    <option value="Ingreso">Ingreso</option>
                    <option value="Gasto">Gasto</option>
                </select>
            </div>
            <div class="form-group">
                <label for="transaccion-categoria">Categoría *</label>
                <select id="transaccion-categoria" required>
                    <option value="">Seleccionar categoría...</option>
                </select>
            </div>
            <div class="form-group">
                <label for="transaccion-monto">Monto (€) *</label>
                <input type="number" id="transaccion-monto" step="0.01" min="0" placeholder="0,00€" required>
            </div>
            <div class="form-group">
                <label for="transaccion-descripcion">Descripción *</label>
                <input type="text" id="transaccion-descripcion" required maxlength="200">
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="hideModal()">
                    Cancelar
                </button>
                <button type="submit" class="btn btn-primary">
                    Crear Transacción
                </button>
            </div>
        </form>
    `;
    
    showModal(modalContent);
    
    // Configurar el filtro de categorías por tipo
    const tipoSelect = document.getElementById('transaccion-tipo');
    const categoriaSelect = document.getElementById('transaccion-categoria');
    
    tipoSelect.onchange = () => {
        const tipoSeleccionado = tipoSelect.value;
        categoriaSelect.innerHTML = '<option value="">Seleccionar categoría...</option>';
        
        if (tipoSeleccionado) {
            const categoriasFiltradas = appState.categorias.filter(c => c.tipo === tipoSeleccionado);
            categoriasFiltradas.forEach(categoria => {
                const option = document.createElement('option');
                option.value = categoria.id;
                option.textContent = categoria.nombre;
                categoriaSelect.appendChild(option);
            });
        }
    };
    
    // Configurar el formulario
    document.getElementById('create-transaccion-form').onsubmit = async (e) => {
        e.preventDefault();
        await createTransaccion();
    };
}

async function createRetiro() {
    try {
        const nombre = document.getElementById('retiro-nombre').value.trim();
        const descripcion = document.getElementById('retiro-descripcion').value.trim() || null;
        const fechaInicioStr = document.getElementById('retiro-fecha-inicio').value;
        const fechaFinStr = document.getElementById('retiro-fecha-fin').value;
        const ubicacion = document.getElementById('retiro-ubicacion').value.trim() || null;
        const numeroParticipantes = parseInt(document.getElementById('retiro-participantes').value);
        
        // Validaciones del lado cliente
        if (!nombre) {
            showToast('El nombre del retiro es requerido', 'error');
            return;
        }
        
        if (nombre.length > 200) {
            showToast('El nombre no puede exceder 200 caracteres', 'error');
            return;
        }
        
        if (descripcion && descripcion.length > 500) {
            showToast('La descripción no puede exceder 500 caracteres', 'error');
            return;
        }
        
        if (!fechaInicioStr || !fechaFinStr) {
            showToast('Las fechas de inicio y fin son requeridas', 'error');
            return;
        }
        
        const fechaInicio = new Date(fechaInicioStr);
        const fechaFin = new Date(fechaFinStr);
        
        if (fechaInicio >= fechaFin) {
            showToast('La fecha de fin debe ser posterior a la fecha de inicio', 'error');
            return;
        }
        
        if (isNaN(numeroParticipantes) || numeroParticipantes < 1) {
            showToast('El número de participantes debe ser mayor a 0', 'error');
            return;
        }
        
        // Verificar si ya existe un retiro con el mismo nombre
        const retiroExistente = appState.retiros.find(r => 
            r.nombre.toLowerCase() === nombre.toLowerCase()
        );
        
        if (retiroExistente) {
            showToast('Ya existe un retiro con ese nombre', 'error');
            return;
        }
        
        const nuevoRetiro = await invoke('create_retiro', {
            data: {
                nombre,
                descripcion,
                fecha_inicio: fechaInicio.toISOString(),
                fecha_fin: fechaFin.toISOString(),
                ubicacion,
                numero_participantes: numeroParticipantes
            }
        });
        
        appState.retiros.push(nuevoRetiro);
        await loadRetiros();
        hideModal();
        showToast('Retiro creado exitosamente', 'success');
    } catch (error) {
        console.error('Error creando retiro:', error);
        showToast('Error creando retiro: ' + getErrorMessage(error), 'error');
    }
}

function editRetiro(id) {
    const retiro = appState.retiros.find(r => r.id === id);
    if (!retiro) return;
    
    const modalContent = `
        <div class="modal-header">
            <h3>Editar Retiro</h3>
            <button class="close-modal" onclick="hideModal()">×</button>
        </div>
        <form id="edit-retiro-form">
            <div class="form-group">
                <label for="edit-retiro-nombre">Nombre *</label>
                <input type="text" id="edit-retiro-nombre" value="${retiro.nombre}" required maxlength="200">
            </div>
            <div class="form-group">
                <label for="edit-retiro-descripcion">Descripción</label>
                <textarea id="edit-retiro-descripcion" maxlength="500">${retiro.descripcion || ''}</textarea>
            </div>
            <div class="form-group">
                <label for="edit-retiro-fecha-inicio">Fecha de Inicio *</label>
                <input type="datetime-local" id="edit-retiro-fecha-inicio" value="${new Date(retiro.fecha_inicio).toISOString().slice(0, 16)}" required>
            </div>
            <div class="form-group">
                <label for="edit-retiro-fecha-fin">Fecha de Fin *</label>
                <input type="datetime-local" id="edit-retiro-fecha-fin" value="${new Date(retiro.fecha_fin).toISOString().slice(0, 16)}" required>
            </div>
            <div class="form-group">
                <label for="edit-retiro-ubicacion">Ubicación</label>
                <input type="text" id="edit-retiro-ubicacion" value="${retiro.ubicacion || ''}" maxlength="200">
            </div>
            <div class="form-group">
                <label for="edit-retiro-participantes">Número de Participantes *</label>
                <input type="number" id="edit-retiro-participantes" value="${retiro.numero_participantes}" min="1" required>
            </div>
            <div class="form-group">
                <label for="edit-retiro-estado">Estado *</label>
                <select id="edit-retiro-estado" required>
                    <option value="Planificacion" ${retiro.estado === 'Planificacion' ? 'selected' : ''}>Planificación</option>
                    <option value="Activo" ${retiro.estado === 'Activo' ? 'selected' : ''}>Activo</option>
                    <option value="Finalizado" ${retiro.estado === 'Finalizado' ? 'selected' : ''}>Finalizado</option>
                </select>
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="hideModal()">
                    Cancelar
                </button>
                <button type="submit" class="btn btn-primary">
                    Actualizar Retiro
                </button>
            </div>
        </form>
    `;
    
    showModal(modalContent);
    
    document.getElementById('edit-retiro-form').onsubmit = async (e) => {
        e.preventDefault();
        await updateRetiro(id);
    };
}

async function updateRetiro(id) {
    try {
        const nombre = document.getElementById('edit-retiro-nombre').value;
        const descripcion = document.getElementById('edit-retiro-descripcion').value || null;
        const fechaInicio = new Date(document.getElementById('edit-retiro-fecha-inicio').value).toISOString();
        const fechaFin = new Date(document.getElementById('edit-retiro-fecha-fin').value).toISOString();
        const ubicacion = document.getElementById('edit-retiro-ubicacion').value || null;
        const numeroParticipantes = parseInt(document.getElementById('edit-retiro-participantes').value);
        const nuevoEstado = document.getElementById('edit-retiro-estado').value;
        
        // Obtener el retiro actual para comparar el estado
        const retiroActual = appState.retiros.find(r => r.id === id);
        
        // Actualizar los datos básicos del retiro
        const retiroActualizado = await invoke('update_retiro', {
            id: id,
            data: {
                nombre,
                descripcion,
                fecha_inicio: fechaInicio,
                fecha_fin: fechaFin,
                ubicacion,
                numero_participantes: numeroParticipantes
            }
        });
        
        let retiroFinal = retiroActualizado;
        
        // Si el estado cambió, actualizarlo por separado
        if (retiroActual && retiroActual.estado !== nuevoEstado) {
            const retiroConEstadoActualizado = await invoke('update_retiro_estado', {
                id: id,
                estado: nuevoEstado
            });
            retiroFinal = retiroConEstadoActualizado || retiroActualizado;
        }
        
        if (retiroFinal) {
            const index = appState.retiros.findIndex(r => r.id === id);
            if (index !== -1) {
                appState.retiros[index] = retiroFinal;
            }
            await loadRetiros();
            await loadDashboard(); // Actualizar dashboard si cambió el estado
            hideModal();
            showToast('Retiro actualizado exitosamente', 'success');
        } else {
            showToast('Retiro no encontrado', 'error');
        }
    } catch (error) {
        console.error('Error actualizando retiro:', error);
        showToast('Error actualizando retiro: ' + error, 'error');
    }
}

async function deleteRetiro(id) {
    const retiro = appState.retiros.find(r => r.id === id);
    if (!retiro) return;
    
    if (confirm(`¿Estás seguro de que quieres eliminar el retiro "${retiro.nombre}"?\n\nEsta acción también eliminará todas las transacciones asociadas.`)) {
        try {
            await invoke('delete_retiro', { id: id });
            
            appState.retiros = appState.retiros.filter(r => r.id !== id);
            
            // Si era el retiro seleccionado, limpiar selección
            if (appState.selectedRetiro === id) {
                appState.selectedRetiro = null;
            }
            
            await loadRetiros();
            showToast('Retiro eliminado exitosamente', 'success');
        } catch (error) {
            console.error('Error eliminando retiro:', error);
            showToast('Error eliminando retiro: ' + error, 'error');
        }
    }
}

async function createCategoria() {
    try {
        const nombre = document.getElementById('categoria-nombre').value.trim();
        const tipo = document.getElementById('categoria-tipo').value;
        const color = document.getElementById('categoria-color').value;
        
        // Validaciones del lado cliente
        if (!nombre) {
            showToast('El nombre de la categoría es requerido', 'error');
            return;
        }
        
        if (nombre.length > 100) {
            showToast('El nombre no puede exceder 100 caracteres', 'error');
            return;
        }
        
        if (!tipo) {
            showToast('El tipo de categoría es requerido', 'error');
            return;
        }
        
        // Verificar si ya existe una categoría con el mismo nombre
        const categoriaExistente = appState.categorias.find(c => 
            c.nombre.toLowerCase() === nombre.toLowerCase() && c.tipo === tipo
        );
        
        if (categoriaExistente) {
            showToast('Ya existe una categoría con ese nombre y tipo', 'error');
            return;
        }
        
        const nuevaCategoria = await invoke('create_categoria', {
            data: { nombre, tipo, color }
        });
        
        appState.categorias.push(nuevaCategoria);
        await loadCategorias();
        hideModal();
        showToast('Categoría creada exitosamente', 'success');
    } catch (error) {
        console.error('Error creando categoría:', error);
        showToast('Error creando categoría: ' + getErrorMessage(error), 'error');
    }
}

function editCategoria(id) {
    const categoria = appState.categorias.find(c => c.id === id);
    if (!categoria) return;
    
    const modalContent = `
        <div class="modal-header">
            <h3>Editar Categoría</h3>
            <button class="close-modal" onclick="hideModal()">×</button>
        </div>
        <form id="edit-categoria-form">
            <div class="form-group">
                <label for="edit-categoria-nombre">Nombre *</label>
                <input type="text" id="edit-categoria-nombre" value="${categoria.nombre}" required maxlength="100">
            </div>
            <div class="form-group">
                <label for="edit-categoria-tipo">Tipo *</label>
                <select id="edit-categoria-tipo" required>
                    <option value="Ingreso" ${categoria.tipo === 'Ingreso' ? 'selected' : ''}>Ingreso</option>
                    <option value="Gasto" ${categoria.tipo === 'Gasto' ? 'selected' : ''}>Gasto</option>
                </select>
            </div>
            <div class="form-group">
                <label for="edit-categoria-color">Color *</label>
                <input type="color" id="edit-categoria-color" value="${categoria.color}" required>
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="hideModal()">
                    Cancelar
                </button>
                <button type="submit" class="btn btn-primary">
                    Actualizar Categoría
                </button>
            </div>
        </form>
    `;
    
    showModal(modalContent);
    
    document.getElementById('edit-categoria-form').onsubmit = async (e) => {
        e.preventDefault();
        await updateCategoria(id);
    };
}

async function updateCategoria(id) {
    try {
        const nombre = document.getElementById('edit-categoria-nombre').value;
        const tipo = document.getElementById('edit-categoria-tipo').value;
        const color = document.getElementById('edit-categoria-color').value;
        
        const categoriaActualizada = await invoke('update_categoria', {
            id: id,
            data: { nombre, tipo, color }
        });
        
        if (categoriaActualizada) {
            const index = appState.categorias.findIndex(c => c.id === id);
            if (index !== -1) {
                appState.categorias[index] = categoriaActualizada;
            }
            await loadCategorias();
            hideModal();
            showToast('Categoría actualizada exitosamente', 'success');
        } else {
            showToast('Categoría no encontrada', 'error');
        }
    } catch (error) {
        console.error('Error actualizando categoría:', error);
        showToast('Error actualizando categoría: ' + error, 'error');
    }
}

async function deleteCategoria(id) {
    const categoria = appState.categorias.find(c => c.id === id);
    if (!categoria) return;
    
    if (confirm(`¿Estás seguro de que quieres eliminar la categoría "${categoria.nombre}"?`)) {
        try {
            await invoke('delete_categoria', { id: id });
            
            appState.categorias = appState.categorias.filter(c => c.id !== id);
            await loadCategorias();
            showToast('Categoría eliminada exitosamente', 'success');
        } catch (error) {
            console.error('Error eliminando categoría:', error);
            showToast('Error eliminando categoría: ' + error, 'error');
        }
    }
}

async function createTransaccion() {
    try {
        const tipo = document.getElementById('transaccion-tipo').value;
        const categoriaId = document.getElementById('transaccion-categoria').value;
        const montoStr = document.getElementById('transaccion-monto').value;
        const descripcion = document.getElementById('transaccion-descripcion').value.trim();
        
        // Validaciones del lado cliente
        if (!tipo) {
            showToast('El tipo de transacción es requerido', 'error');
            return;
        }
        
        if (!categoriaId) {
            showToast('La categoría es requerida', 'error');
            return;
        }
        
        if (!montoStr || isNaN(parseFloat(montoStr))) {
            showToast('El monto debe ser un número válido', 'error');
            return;
        }
        
        const monto = parseFloat(montoStr);
        if (monto <= 0) {
            showToast('El monto debe ser mayor a 0', 'error');
            return;
        }
        
        if (monto > 999999.99) {
            showToast('El monto no puede exceder 999.999,99€', 'error');
            return;
        }
        
        if (!descripcion) {
            showToast('La descripción es requerida', 'error');
            return;
        }
        
        if (descripcion.length > 200) {
            showToast('La descripción no puede exceder 200 caracteres', 'error');
            return;
        }
        
        // Verificar que hay un retiro seleccionado
        if (!appState.selectedRetiro) {
            showToast('No hay retiro seleccionado. Selecciona un retiro primero.', 'error');
            return;
        }
        
        // Verificar que la categoría seleccionada sea del tipo correcto
        const categoria = appState.categorias.find(c => c.id === categoriaId);
        if (!categoria) {
            showToast('Categoría no encontrada', 'error');
            return;
        }
        
        if (categoria.tipo !== tipo) {
            showToast('La categoría seleccionada no coincide con el tipo de transacción', 'error');
            return;
        }
        
        const nuevaTransaccion = await invoke('create_transaccion', {
            data: {
                tipo,
                categoria_id: categoriaId,
                retiro_id: appState.selectedRetiro,
                monto,
                descripcion
            }
        });
        
        await loadTransaccionesTable();
        await loadDashboard(); // Actualizar dashboard con nuevo balance
        hideModal();
        showToast('Transacción creada exitosamente', 'success');
    } catch (error) {
        console.error('Error creando transacción:', error);
        showToast('Error creando transacción: ' + getErrorMessage(error), 'error');
    }
}

async function deleteTransaccion(id) {
    if (confirm('¿Estás seguro de que quieres eliminar esta transacción?')) {
        try {
            await invoke('delete_transaccion', { id: id });
            
            await loadTransaccionesTable();
            await loadDashboard(); // Actualizar dashboard con nuevo balance
            showToast('Transacción eliminada exitosamente', 'success');
        } catch (error) {
            console.error('Error eliminando transacción:', error);
            showToast('Error eliminando transacción: ' + error, 'error');
        }
    }
}

// Configurar filtro de categorías
document.addEventListener('DOMContentLoaded', () => {
    const categoriaFilter = document.getElementById('categoria-filter');
    if (categoriaFilter) {
        categoriaFilter.onchange = () => loadCategorias();
    }
});
