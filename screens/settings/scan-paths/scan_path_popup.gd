extends FileDialog

var last_valid_path := ""

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _init(starting_path: String = "") -> void:
	file_mode = FileDialog.FILE_MODE_OPEN_DIR
	access = FileDialog.ACCESS_FILESYSTEM
	
	title = "Select a scan path"
	
	show_hidden_files = true
	exclusive = false
	popup_window = true
	dialog_hide_on_ok = true
	
	last_valid_path = starting_path
	current_dir = starting_path if not starting_path.is_empty() else AM.config.default_file_search_path

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func hide() -> void:
	super()
	close_requested.emit()
