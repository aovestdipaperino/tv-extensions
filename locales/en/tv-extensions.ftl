# The strings tv-extensions shows: the CSV editor, record forms, dual lists,
# autocomplete fields, dialogs loaded from JSON and the log window. This
# English file is the reference: every other locale defines the same ids
# (the tests in src/i18n.rs check it). The OK and Cancel buttons are
# turbo-vision's own `button-ok` and `button-cancel`.
#
# A `~` pair marks the hot key: `~O~pen` is reached with Alt+O. Keep one per
# string, and keep the letters of one menu or dialog different from each
# other (the groups are listed in those tests). Ids start with `tvx-`.

## Autocomplete

tvx-autocomplete-required = A value is required
tvx-autocomplete-not-in-list = Choose a value from the list
tvx-autocomplete-no-matches = No matches

## Dual list. The headers keep a leading space that is added in code.

tvx-dual-filter = Filter:
tvx-dual-available = Available
tvx-dual-chosen = Chosen
tvx-dual-header = { $title } ({ $total })
tvx-dual-header-filtered = { $title } ({ $shown } of { $total })
tvx-dual-min-one = Choose at least one item
tvx-dual-min-many = Choose at least { $count } items
tvx-dual-max-one = Choose at most one item
tvx-dual-max-many = Choose at most { $count } items

## Form fields. A message that says what is wrong with a value ("must be a
## number") is put after the field's name by tvx-form-problem.

tvx-form-problem = { $name } { $problem }
tvx-form-required = { $name } is required
tvx-form-must-be-checked = { $name } must be checked
tvx-form-this-field = This field
tvx-form-must-be-whole-number = must be a whole number
tvx-form-must-be-number = must be a number
tvx-form-must-be-date = must be a date like 2026-12-31
tvx-form-must-be-time = must be a time like 14:30
tvx-form-not-valid = is not valid
tvx-form-must-be-chosen = must be chosen

## Dialogs loaded from JSON

tvx-dialog-more-errors = { $message } (+{ $count } more)

## Log window

tvx-log-title = Log

## CSV editor: menus

tvx-csv-menu-file = ~F~ile
tvx-csv-file-new = ~N~ew
tvx-csv-file-open = ~O~pen...
tvx-csv-file-save = ~S~ave
tvx-csv-file-save-as = Save ~a~s...
tvx-csv-file-exit = E~x~it
tvx-csv-menu-edit = ~E~dit
tvx-csv-edit-cell = ~E~dit cell
tvx-csv-edit-rename-column = ~R~ename column
tvx-csv-menu-row = ~R~ow
tvx-csv-menu-column = ~C~olumn
tvx-csv-insert = ~I~nsert
tvx-csv-delete = ~D~elete

## CSV editor: status line. The key (Ctrl-R, F10) is not part of the message;
## the code puts it in front, since a key is not translated. At 60 columns a
## long translation loses its first items, never F10, the way into the menus.

tvx-csv-status-row = Row
tvx-csv-status-save = Save
tvx-csv-status-exit = Exit
tvx-csv-status-menu = Menu

## CSV editor: dialogs

tvx-csv-edit-cell-title = Edit cell
tvx-csv-edit-cell-label = Row { $row }, column { $column }:
tvx-csv-rename-title = Rename column
tvx-csv-rename-label = Column name:
tvx-csv-save-as-title = Save as
tvx-csv-save-as-label = File name:
tvx-csv-open-title = Open
tvx-csv-open-button = ~O~pen
tvx-csv-unsaved-title = Unsaved changes
tvx-csv-unsaved-text = Save changes to { $name }?
tvx-csv-unsaved-save = ~S~ave
tvx-csv-unsaved-discard = ~D~iscard
tvx-csv-delete-row-title = Delete row
tvx-csv-delete-row-text = Delete row { $row }? It is removed from the store when the grid closes.

## CSV editor: messages on the status bar

tvx-csv-parse-error = parse error at line { $line }; saving will drop the rest
tvx-csv-saved = saved { $name }
tvx-csv-no-files = no saved files yet
tvx-csv-name-needed = a file name is needed
tvx-csv-name-slash = names cannot contain '/'
tvx-csv-column-fixed = the # column is fixed
tvx-csv-columns-fixed = this grid's columns are fixed
tvx-csv-grid-stays = this grid stays on { $name }
