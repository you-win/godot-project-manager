extends AbstractSettingsPage

const EditorBinaryEditPopup: PackedScene = preload("res://screens/settings/editor-binaries/editor_binary_edit_popup.tscn")
const EditorBinaryItem: PackedScene = preload("res://screens/settings/editor-binaries/editor_binary_item.tscn")

const POPUP_SIZE_RATIO: float = 0.75

@onready
var default_binaries: VBoxContainer = $VBoxContainer/DefaultBinaries
@onready
var none: VBoxContainer = $VBoxContainer/None
@onready
var godot_3: VBoxContainer = $VBoxContainer/Godot3
@onready
var godot_4: VBoxContainer = $VBoxContainer/Godot4
@onready
var other: VBoxContainer = $VBoxContainer/Other
@onready
var _categories := {
	EditorBinary.Type.NONE: none,
	EditorBinary.Type.GODOT_3: godot_3,
	EditorBinary.Type.GODOT_4: godot_4,
	EditorBinary.Type.OTHER: other
}

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	_populate(_categories)
	$VBoxContainer/Button.pressed.connect(_create_edit_popup)

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

func _create_edit_popup(path: String = "") -> Window:
	var popup: Window = EditorBinaryEditPopup.instantiate()
	popup.close_requested.connect(func() -> void:
		popup.queue_free()
	)
	popup.confirmed.connect(func(eb: EditorBinary) -> void:
		if path.is_empty():
			_add_item(eb, _categories)
		popup.queue_free()
	)
	popup.initial_path = path
	add_child(popup)
	popup.popup_centered(get_tree().root.size * POPUP_SIZE_RATIO)
	
	return popup

func _add_item(eb: EditorBinary, mapping: Dictionary) -> void:
	var list: VBoxContainer = mapping[eb.type]
	
	var apply_editor_binary := func(editor_binary: EditorBinary, control: Control) -> void:
		control.item_name.text = editor_binary.name
		control.path.text = editor_binary.path
		control.default_arguments.text = ""
		for i in editor_binary.default_args:
			control.default_arguments.text += "%s " % i
		control.default_arguments.text = control.default_arguments.text.strip_edges()
	
	var item := EditorBinaryItem.instantiate()
	list.add_child(item)
	item.edit.pressed.connect(func() -> void:
		var popup: Window = self._create_edit_popup.call(eb.path)
		popup.confirmed.connect(func(new_eb: EditorBinary) -> void:
			if new_eb.path != eb.path:
				AM.config.editor_binaries.erase(eb.path)
			if new_eb.type != eb.type:
				list.remove_child(item)
				mapping[new_eb.type].add_child(item)
			
			apply_editor_binary.call(new_eb, item)
		)
	)
	item.delete.pressed.connect(func() -> void:
		AM.config.editor_binaries.erase(eb.path)
		item.queue_free()
	)
	
	apply_editor_binary.call(eb, item)

func _populate(data: Dictionary) -> void:
	for i in AM.config.editor_binaries.values():
		_add_item(i as EditorBinary, _categories)
	
	default_binaries.godot3.text = AM.config.default_godot_3_binary
	default_binaries.godot4.text = AM.config.default_godot_4_binary

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func update() -> void:
	# Update popup windows first
	for child in get_children():
		if child is Window:
			child.update()
	
	for control in _categories.values():
		for child in control.get_children():
			child.free()
	
	_populate(_categories)
