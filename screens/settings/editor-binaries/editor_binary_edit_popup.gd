extends Window

signal confirmed(binary: EditorBinary)

const FILE_SELECT_SCALE: float = 0.8

@onready
var item_name: Control = $VBoxContainer/ScrollContainer/VBoxContainer/EditorBinaryEditName
@onready
var binary_path: Control = $VBoxContainer/ScrollContainer/VBoxContainer/EditorBinaryEditPath
@onready
var type: Control = $VBoxContainer/ScrollContainer/VBoxContainer/EditorBinaryEditType
@onready
var default_args: Control = $VBoxContainer/ScrollContainer/VBoxContainer/EditorBinaryEditDefaultArguments

var initial_path := ""

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	# Intentionally not translated
	for key in EditorBinary.Type.keys():
		type.option_button.add_item(key)
	
	var confirm: Button = $VBoxContainer/HBoxContainer/Confirm
	confirm.pressed.connect(func() -> void:
		var bin_path: String = binary_path.line_edit.text
		var bin_name: String = item_name.line_edit.text
		if bin_name.is_empty():
			bin_name = bin_path.get_file()
		var bin_type: int = type.option_button.get_item_index(type.option_button.get_selected_id())
		var bin_args: Array[String] = []
		for i in default_args.line_edit.text.split(","):
			bin_args.append(i.strip_edges())
			
		var eb := EditorBinary.new(bin_name, bin_path, bin_type, bin_args)
		
		var config := AM.config
		
		config.editor_binaries[bin_path] = eb
		
		match eb.type:
			EditorBinary.Type.GODOT_3:
				if config.default_godot_3_binary.is_empty():
					config.default_godot_3_binary = bin_path
			EditorBinary.Type.GODOT_4:
				if config.default_godot_4_binary.is_empty():
					config.default_godot_4_binary = bin_path
		
		confirmed.emit(eb)
	)
	
	$VBoxContainer/HBoxContainer/Cancel.pressed.connect(func() -> void:
		close_requested.emit()
	)
	
	var is_valid_path := func(text: String) -> bool:
		if text.is_empty():
			confirm.disabled = true
			binary_path.clear_status()
			return false
		if not (FileAccess.file_exists(text) or DirAccess.dir_exists_absolute(text)):
			confirm.disabled = true
			binary_path.set_status("SETTINGS_EDITOR_BINARY_EDIT_PATH_DOES_NOT_EXIST")
			return false
		if AM.config.editor_binaries.has(text) and not text == initial_path:
			confirm.disabled = true
			binary_path.set_status("SETTINGS_EDITOR_BINARY_EDIT_PATH_ALREADY_EXISTS")
			return false
		
		binary_path.clear_status()
		confirm.disabled = false
		
		return true
	
	binary_path.button.pressed.connect(func() -> void:
		var popup := FileDialog.new()
		popup.file_mode = FileDialog.FILE_MODE_OPEN_ANY
		popup.access = FileDialog.ACCESS_FILESYSTEM
		popup.current_path = AM.config.default_file_search_path
		popup.show_hidden_files = true
		
		var any_selected := func(text: String) -> void:
			binary_path.line_edit.text = text
			if is_valid_path.call(text) and item_name.line_edit.text.is_empty():
				item_name.line_edit.text = text.get_file()
		
		popup.file_selected.connect(any_selected)
		popup.dir_selected.connect(any_selected)
		
		add_child(popup)
		popup.popup_centered(size * FILE_SELECT_SCALE)
		
		await popup.close_requested
		
		popup.queue_free()
		
	)
	binary_path.line_edit.text_changed.connect(is_valid_path)
	
	for i in [item_name, binary_path, type, default_args]:
		i.path = initial_path
		i.update()

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func update() -> void:
	item_name.update()
	binary_path.update()
	type.update()
	default_args.update()
