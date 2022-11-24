extends PanelContainer

@onready
var icon: TextureRect = %Icon
@onready
var title: Label = %Title
@onready
var path: Label = %Path

# TODO testing
var is_favorite := false

var in_option_button := false

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	var favorite: Button = %Favorite
	favorite.modulate = Color.BLACK
	favorite.pressed.connect(func() -> void:
		is_favorite = not is_favorite
		if is_favorite:
			favorite.modulate = Color.WHITE
		else:
			favorite.modulate = Color.BLACK
	)
	
	var highlight: Panel = %Highlight
	
	mouse_entered.connect(func() -> void:
		highlight.show()
	)
	mouse_exited.connect(func() -> void:
		highlight.hide()
	)
	
	var options: MenuButton = %Options
	options.mouse_entered.connect(func() -> void:
		in_option_button = true
	)
	options.mouse_exited.connect(func() -> void:
		in_option_button = false
	)

func _gui_input(event: InputEvent) -> void:
	if not event is InputEventMouseButton:
		return
	if not event.double_click:
		return
	if in_option_button:
		return
	
	print(title.text)

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
