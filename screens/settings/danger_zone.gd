extends AbstractSettingsPage

const CLEAR_SETTINGS_START := "SETTINGS_CLEAR_SETTINGS_0"
const CLEAR_SETTINGS_CONFIRM := "SETTINGS_CLEAR_SETTINGS_1"
const CLEAR_SETTINGS_CLEARING := "SETTINGS_CLEAR_SETTINGS_2"
const CLEAR_SETTINGS_CLEARED := "SETTINGS_CLEAR_SETTINGS_3"

var logger := Logger.new("DangerZone")

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	var clear_settings: Button = $VBoxContainer/ClearSettings
	
	var clear_callable := func(button: Button) -> void:
		match button.text:
			CLEAR_SETTINGS_START:
				button.text = CLEAR_SETTINGS_CONFIRM
			CLEAR_SETTINGS_CONFIRM:
				AM.config.clear()
				button.text = CLEAR_SETTINGS_CLEARING
				var res = await AM.write_config()
				while not res is Result:
					await get_tree().process_frame
				if res.is_err():
					var error: Error = res.unwrap_err()
					if error.code != Error.Code.TRYING_TO_SAVE:
						logger.error(error.to_string())
						return
				button.text = CLEAR_SETTINGS_CLEARED
				self.config_updated.emit()
	
	clear_settings.pressed.connect(clear_callable.bind(clear_settings))
	clear_settings.mouse_exited.connect(func() -> void:
		clear_settings.text = CLEAR_SETTINGS_START
	)

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func update() -> void:
	pass
