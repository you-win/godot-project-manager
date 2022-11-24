extends VBoxContainer

const FILTER_ALL := "ALL"
const LOGS_START := "---Start logs---"

var _current_filter: int = 0

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	var filter: OptionButton = $HFlowContainer/HBoxContainer/Filter
	# These are not expected to be translated
	filter.add_item(FILTER_ALL)
	for i in Logger.LogTypes.keys():
		filter.add_item(i)
	
	var te: TextEdit = $TextEdit
	
	var handler := func(text: String) -> void:
		if _current_filter != 0:
			var option: String = Logger.LogTypes[filter.get_item_text(_current_filter)]
			
			var split := text.split(" ", false, 1)
			if split.size() < 2:
				return
			if split[0] != option:
				return
			
		te.text = "%s\n%s" % [text, te.text]
	
	var populate_te := func() -> void:
		te.text = LOGS_START
		for i in Logger.get_logs():
			handler.call(i)
	populate_te.call()
	
	AM.log_received.connect(handler)
	
	filter.item_selected.connect(func(idx: int) -> void:
		_current_filter = idx
		var selected_option := filter.get_item_text(idx)
		
		if selected_option == FILTER_ALL:
			populate_te.call()
			return
		
		var tag: String = Logger.LogTypes[selected_option]
		
		te.text = LOGS_START
		for i in Logger.get_logs():
			var i_split: PackedStringArray = i.split(" ", false, 1)
			if i_split.size() < 2:
				continue
			if i_split[0] != tag:
				continue
			
			handler.call(i)
	)
	filter.select(_current_filter)
	
	$HFlowContainer/OpenLogFile.pressed.connect(func() -> void:
		OS.shell_open(ProjectSettings.globalize_path("user://logs/godot.log"))
	)
	$HFlowContainer/Clear.pressed.connect(func() -> void:
		Logger.clear_logs()
		populate_te.call()
	)

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
