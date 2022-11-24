extends AbstractSettingsItem

@onready
var option_button: OptionButton = $HBoxContainer/OptionButton

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
		
		option_button.select(eb.type)
	else:
		option_button.select(0)
