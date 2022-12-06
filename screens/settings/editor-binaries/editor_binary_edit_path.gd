extends AbstractSettingsItem

@onready
var line_edit: LineEdit = $HBoxContainer/HBoxContainer/LineEdit
@onready
var button: Button = $HBoxContainer/HBoxContainer/Button

var path := ""

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func update() -> void:
	if not path.is_empty():
		var eb: EditorBinary = AM.config.editor_binaries.get(path, null)
		if eb == null:
			path = ""
			set_status("SETTINGS_EDITOR_BINARY_EDIT_OBJECT_DELETED")
			return
		
		line_edit.text = eb.path
	else:
		line_edit.text = ""
