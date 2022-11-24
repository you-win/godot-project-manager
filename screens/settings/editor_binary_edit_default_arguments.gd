extends AbstractSettingsItem

@onready
var line_edit: LineEdit = $HBoxContainer/LineEdit

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
		
		for i in eb.default_args:
			line_edit.text += "%s " % i
		line_edit.text = line_edit.text.strip_edges()
	else:
		line_edit.text = ""
