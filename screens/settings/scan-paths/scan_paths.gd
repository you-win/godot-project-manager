extends AbstractSettingsItem

const ScanPathItem: PackedScene = preload("res://screens/settings/scan-paths/scan_path_item.tscn")
const ScanPathPopup: GDScript = preload("res://screens/settings/scan-paths/scan_path_popup.gd")

const FILE_SELECT_SCALE: float = 0.8

@onready
var list: VBoxContainer = $List

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	$AddScanPath.pressed.connect(func() -> void:
		clear_status()
		
		var popup := ScanPathPopup.new()
		
		add_child(popup)
		popup.popup_centered(get_viewport().size * FILE_SELECT_SCALE)
		
		await popup.close_requested
		
		var selected_path := popup.current_file if not popup.current_file.is_empty() else popup.current_dir
		if selected_path == popup.last_valid_path:
			return
		if selected_path.is_empty():
			return
		if selected_path in AM.config.scan_paths:
			set_status("Path %s is already configured." % selected_path)
			return
		
		popup.queue_free()
		
		AM.config.scan_paths.append(selected_path)
		
		_create_item(selected_path)
	)

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

func _create_item(initial_path: String) -> void:
	var item := ScanPathItem.instantiate()
	list.add_child(item)
	item.label.text = initial_path
	item.last_valid_path = initial_path
	
	item.edit_button.pressed.connect(func() -> void:
		item.clear_status()
		
		var popup := ScanPathPopup.new()
		
		add_child(popup)
		popup.popup_centered(get_viewport().size * FILE_SELECT_SCALE)
		
		await popup.close_requested
		
		var selected_path := popup.current_file if not popup.current_file.is_empty() else popup.current_dir
		if selected_path == popup.last_valid_path:
			return
		if selected_path.is_empty():
			return
		if selected_path == item.last_valid_path:
			return
		
		popup.queue_free()
		
		AM.config.scan_paths.erase(item.last_valid_path)
		AM.config.scan_paths.append(selected_path)
		
		item.last_valid_path = item.label.text
		item.label.text = selected_path
	)
	item.delete_button.pressed.connect(func() -> void:
		AM.config.scan_paths.erase(item.last_valid_path)
		item.queue_free()
	)

func _populate(list: VBoxContainer) -> void:
	clear_status()
	
	for path in AM.config.scan_paths:
		_create_item(path)

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func update() -> void:
	var list: VBoxContainer = $List
	for child in list.get_children():
		child.free()
	
	_populate(list)
