# Textos de tv-extensions en español. Las claves son las de
# locales/en/tv-extensions.ftl; `~` marca la tecla rápida (Alt+letra).

## Autocompletado

tvx-autocomplete-required = Se necesita un valor
tvx-autocomplete-not-in-list = Elija un valor de la lista
tvx-autocomplete-no-matches = Sin coincidencias

## Lista doble

tvx-dual-filter = Filtro:
tvx-dual-available = Disponibles
tvx-dual-chosen = Elegidos
tvx-dual-header = { $title } ({ $total })
tvx-dual-header-filtered = { $title } ({ $shown } de { $total })
tvx-dual-min-one = Elija al menos un elemento
tvx-dual-min-many = Elija al menos { $count } elementos
tvx-dual-max-one = Elija como máximo un elemento
tvx-dual-max-many = Elija como máximo { $count } elementos

## Campos de formulario

tvx-form-problem = { $name } { $problem }
tvx-form-required = { $name } es obligatorio
tvx-form-must-be-checked = { $name } debe estar marcado
tvx-form-this-field = Este campo
tvx-form-must-be-whole-number = debe ser un número entero
tvx-form-must-be-number = debe ser un número
tvx-form-must-be-date = debe ser una fecha como 2026-12-31
tvx-form-must-be-time = debe ser una hora como 14:30
tvx-form-not-valid = no es válido
tvx-form-must-be-chosen = debe elegirse

## Diálogos cargados desde JSON

tvx-dialog-more-errors = { $message } (y { $count } más)

## Ventana de registro

tvx-log-title = Registro

## Editor CSV: menús

tvx-csv-menu-file = ~A~rchivo
tvx-csv-file-new = ~N~uevo
tvx-csv-file-open = A~b~rir...
tvx-csv-file-save = ~G~uardar
tvx-csv-file-save-as = Guardar ~c~omo...
tvx-csv-file-exit = ~S~alir
tvx-csv-menu-edit = ~E~ditar
tvx-csv-edit-cell = ~E~ditar celda
tvx-csv-edit-rename-column = ~R~enombrar columna
tvx-csv-menu-row = ~F~ila
tvx-csv-menu-column = ~C~olumna
tvx-csv-insert = ~I~nsertar
tvx-csv-delete = ~E~liminar

## Editor CSV: línea de estado. En 60 columnas no caben los cuatro avisos y
## se quita el primero (Fila): F10, la entrada a los menús, se conserva.

tvx-csv-status-row = Fila
tvx-csv-status-save = Guardar
tvx-csv-status-exit = Salir
tvx-csv-status-menu = Menú

## Editor CSV: diálogos

tvx-csv-edit-cell-title = Editar celda
tvx-csv-edit-cell-label = Fila { $row }, columna { $column }:
tvx-csv-rename-title = Renombrar columna
tvx-csv-rename-label = Nombre de la columna:
tvx-csv-save-as-title = Guardar como
tvx-csv-save-as-label = Nombre del archivo:
tvx-csv-open-title = Abrir
tvx-csv-open-button = ~A~brir
tvx-csv-unsaved-title = Cambios sin guardar
tvx-csv-unsaved-text = ¿Guardar los cambios de { $name }?
tvx-csv-unsaved-save = ~G~uardar
tvx-csv-unsaved-discard = ~D~escartar
tvx-csv-delete-row-title = Eliminar fila
tvx-csv-delete-row-text = ¿Eliminar la fila { $row }? Se quita del almacén al cerrar la tabla.

## Editor CSV: mensajes de la barra de estado

tvx-csv-parse-error = error de lectura en la línea { $line }; al guardar se perderá el resto
tvx-csv-saved = { $name } guardado
tvx-csv-no-files = aún no hay archivos guardados
tvx-csv-name-needed = hace falta un nombre de archivo
tvx-csv-name-slash = los nombres no pueden contener '/'
tvx-csv-column-fixed = la columna # es fija
tvx-csv-columns-fixed = las columnas de esta tabla son fijas
tvx-csv-grid-stays = esta tabla sigue en { $name }
