extends VBoxContainer

const ScanPathItem: PackedScene = preload("res://screens/settings/scan_path_item.tscn")

const FILE_SELECT_SCALE: float = 0.8

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

func _populate(list: VBoxContainer) -> void:
	var create_scan_path_item := func(path: String = "") -> void:
		var item := ScanPathItem.instantiate()
		list.add_child(item)
		
		if not path.is_empty():
			item.line_edit.text = path
		
		var validate_scan_path := func(text: String) -> void:
			if not item.last_valid_path.is_empty():
				AM.config.scan_paths.erase(item.last_valid_path)
				item.last_valid_path = ""
			
			if text.is_empty():
				item.clear_status()
				return
			if not DirAccess.dir_exists_absolute(text):
				item.set_status("SETTINGS_SCAN_PATH_DOES_NOT_EXIST")
				return
			if text in AM.config.scan_paths:
				item.set_status("SETTINGS_SCAN_PATH_ALREADY_EXISTS")
				return
			
			AM.config.scan_paths.append(text)
			item.last_valid_path = text
			item.clear_status()
		
		item.line_edit.text_changed.connect(validate_scan_path)
		item.delete.pressed.connect(func() -> void:
			item.queue_free()
		)
		
		item.select.pressed.connect(func() -> void:
			# TODO This almost certainly can be refactored out
			var popup := FileDialog.new()
			popup.file_mode = FileDialog.FILE_MODE_OPEN_DIR
			popup.access = FileDialog.ACCESS_FILESYSTEM
			popup.current_path = AM.config.default_file_search_path
			popup.show_hidden_files = true
			popup.dir_selected.connect(func(text: String) -> void:
				validate_scan_path.call(text)
				item.line_edit.text = text
			)
			
			add_child(popup)
			popup.popup_centered(get_viewport().size * FILE_SELECT_SCALE)
			
			await popup.close_requested
			
			popup.queue_free()
		)
	
	var button := $HBoxContainer/Button
	if not button.pressed.is_connected(create_scan_path_item):
		button.pressed.connect(create_scan_path_item)
	
	for path in AM.config.scan_paths:
		create_scan_path_item.call(path)

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func update() -> void:
	var list: VBoxContainer = $List
	for child in list.get_children():
		child.free()
	
	_populate(list)
