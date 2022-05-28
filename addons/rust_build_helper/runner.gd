extends PanelContainer

var plugin: EditorPlugin

const StatusMessages := {
	"READY": "Ready to build",
	
	"BUILDING_1": "Building code .",
	"BUILDING_2": "Building code ..",
	"BUILDING_3": "Building code ...",
	
	"BUILD_SUCCESS": "Complete",
	
	"THREAD_ERR": "Unable to start thread",
	"BUILD_ERR": "Build error",
	"CONFIG_ERR": "Configuration error"
}
onready var status := $VBoxContainer/HBoxContainer/Status as Label

const RUST_DIR_HINT := "The location of the Rust code"
onready var rust_dir := $VBoxContainer/RustDir/LineEdit as LineEdit
const BUILD_COMMAND_HINT := "The command to run when in the Rust code's root directory"
onready var build_command := $VBoxContainer/BuildCommand/LineEdit as LineEdit

onready var execute := $VBoxContainer/Execute as Button

var build_thread: Thread
var thread_return_code: int = -1
var thread_output := []

const BUILD_DISPLAY_DELAY: float = 1.0
var build_display_counter: float = BUILD_DISPLAY_DELAY

var dir := Directory.new()

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _init() -> void:
	pass

func _ready() -> void:
	var rust_dir_container := $VBoxContainer/RustDir as Control
	rust_dir_container.hint_tooltip = RUST_DIR_HINT
	rust_dir.hint_tooltip = RUST_DIR_HINT
	
	rust_dir.connect("text_changed", self, "_on_rust_dir_changed")
	
	var build_command_container := $VBoxContainer/BuildCommand as Control
	build_command_container.hint_tooltip = BUILD_COMMAND_HINT
	build_command.hint_tooltip = BUILD_COMMAND_HINT
	
	execute.connect("pressed", self, "_on_execute")
	
	_update_status(StatusMessages.READY)

func _process(delta: float) -> void:
	if build_thread == null:
		return
	
	if build_thread.is_alive():
		if build_display_counter > 0.0:
			build_display_counter -= delta
		else:
			build_display_counter = BUILD_DISPLAY_DELAY
			match status.text:
				StatusMessages.BUILDING_1:
					_update_status(StatusMessages.BUILDING_2)
				StatusMessages.BUILDING_2:
					_update_status(StatusMessages.BUILDING_3)
				StatusMessages.BUILDING_3:
					_update_status(StatusMessages.BUILDING_1)
	else:
		thread_return_code = build_thread.wait_to_finish()
		build_thread = null
		
		if thread_return_code == 0:
			_update_status("%s - %s" % [StatusMessages.BUILD_SUCCESS, str(thread_output)])
		else:
			_update_status("%s - %s" % [StatusMessages.BUILD_ERR, str(thread_output)])
		
		execute.disabled = false
		OS.window_minimized = false

#-----------------------------------------------------------------------------#
# Connections                                                                 #
#-----------------------------------------------------------------------------#

func _on_rust_dir_changed(text: String) -> void:
	if dir.dir_exists(ProjectSettings.globalize_path(text)):
		if build_thread != null and build_thread.is_alive():
			return
		execute.disabled = false
	else:
		execute.disabled = true

func _on_execute() -> void:
	# TODO test this on linux
	# For some reason, cmd.exe requires this format
	var args := ["%s %s %s" % ["cd", ProjectSettings.globalize_path(rust_dir.text), "&&"]]
	args.append_array(build_command.text.split(" ", false))
	
	var command := ""
	
	match OS.get_name().to_lower():
		"windows":
			args.push_front("/c")
			command = "CMD.exe"
		"osx", "x11":
			command = "bash"
	
	thread_output.clear()
	
	OS.window_minimized = true
	
	build_thread = Thread.new()
	var err: int = build_thread.start(self, "_thread_func", {
		"command": command,
		"args": args,
		"output": thread_output
	})
	
	if err != OK:
		_update_status(StatusMessages.THREAD_ERR)
	else:
		_update_status(StatusMessages.BUILDING_1)
		execute.disabled = true

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

func _update_status(text: String) -> void:
	status.text = text

func _thread_func(args: Dictionary) -> int:
	return OS.execute(args.command, args.args, true, args.output)

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
