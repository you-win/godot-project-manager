extends AbstractSettingsItem

const MIN_DEBOUNCE_TIME: float = 0.01

@onready
var line_edit := $HBoxContainer/LineEdit

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	line_edit.text_changed.connect(func(text: String) -> void:
		if text.is_empty() or not text.is_valid_float():
			set_status("GENERIC_UNEXPECTED_VALUE")
			return
		
		var new_time: float = text.to_float()
		if new_time < MIN_DEBOUNCE_TIME:
			set_status("SETTINGS_DEBOUNCE_TIME_VALUE_TOO_LOW")
			return
		
		AM.config.debounce_time = text.to_float()
		clear_status()
	)

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func update() -> void:
	clear_status()
	line_edit.text = "%1.2f" % AM.config.debounce_time
