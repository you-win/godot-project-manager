extends CanvasLayer

var logger := Logger.new("Splash")

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	while not get_tree().root.get_node_or_null("AM"):
		await get_tree().process_frame
	
	var translation_helper := TranslationHelper.new()
	
	var res := translation_helper.scan("res://assets/translations")
	if res.is_err():
		logger.error(res.to_string())
	
	get_tree().change_scene_to_file("res://screens/main.tscn")

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
