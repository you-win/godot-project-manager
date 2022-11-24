extends CanvasLayer

const VERSION_MESSAGES := [
	"Hello, world",
	"sometimes youwin",
	"Virtual Puppet Project"
]
const BB_TAGS: Array[String] = [
	"wave",
	"tornado",
	"shake",
	"rainbow"
]

const AppButtonOptions := [
	"MENU_BUTTON_APP_QUIT"
]

var logger := Logger.new("Main")

@onready
var version_message: RichTextLabel = $VBoxContainer/TopBar/VersionMessage
@onready
var app_button: MenuButton = $VBoxContainer/TopBar/App

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	var app_button_popup := app_button.get_popup()
	for i in AppButtonOptions:
		app_button_popup.add_item(i)
	
	app_button_popup.index_pressed.connect(func(idx: int) -> void:
		match AppButtonOptions[idx]:
			"MENU_BUTTON_APP_QUIT":
				get_tree().quit()
	)
	
	var set_version_message := func(text: String) -> void:
		var tag: String = BB_TAGS[randi_range(0, BB_TAGS.size() - 1)]
		version_message.text = "[right][%s]%s[/%s][/right]" % [tag, text, tag]
	
	if AM.latest_godot_release == AM.WAITING_FOR_RESPONSE:
		AM.received_latest_godot_release.connect(func(text: String) -> void:
			set_version_message.call(text)
		, CONNECT_ONE_SHOT)
	else:
		set_version_message.call(AM.latest_godot_release)
	
	version_message.gui_input.connect(func(event: InputEvent) -> void:
		if not event is InputEventMouseButton:
			return
		if not event.pressed:
			return
		
		var i: int = randi_range(0, VERSION_MESSAGES.size())
		if i in range(VERSION_MESSAGES.size()):
			set_version_message.call(VERSION_MESSAGES[i])
		else:
			set_version_message.call(AM.latest_godot_release)
	)
	
	logger.info("Main ready!")

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
