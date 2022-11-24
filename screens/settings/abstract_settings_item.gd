class_name AbstractSettingsItem
extends VBoxContainer

@onready
var status: Label = $Status

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
	assert(false, "Not yet implemented")

func set_status(text: String) -> void:
	status.text = text
	status.show()

func clear_status() -> void:
	status.text = ""
	status.hide()
