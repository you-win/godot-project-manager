class_name Config
extends Resource

## The configuration data for the project manager.

const SAVE_PATH := "user://config.tres"

const DEFAULT_DEBOUNCE_TIME: float = 1.0
const DEFAULT_FILE_SEARCH_PATH := "user://"

var logger := Logger.new("Config")

## The paths to do any scanning in. If there are no scan paths, will open in the app's directory.
@export
var scan_paths := []
## The default path to use when selecting files.
@export
var default_file_search_path := DEFAULT_FILE_SEARCH_PATH
## Time it takes to debounce in general.
@export
var debounce_time: float = DEFAULT_DEBOUNCE_TIME
## All registered Godot binaries. Binary path -> [code]EditorBinary[/code].
@export
var editor_binaries := {}
## The editor binary to run Godot 3 projects with.
@export
var default_godot_3_binary := ""
## The editor binary to run Godot 4 projects with.
@export
var default_godot_4_binary := ""
## Absolute paths to [code]project.godot[/code] files.
@export
var known_projects: Array[String] = []

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

## Clears all config data.
func clear() -> void:
	scan_paths.clear()
	default_file_search_path = DEFAULT_FILE_SEARCH_PATH
	
	debounce_time = DEFAULT_DEBOUNCE_TIME
	
	editor_binaries.clear()
	default_godot_3_binary = ""
	default_godot_4_binary = ""
	
	known_projects.clear()

## Execute a project using a configured editor binary.
func execute(project_path: String, editor_type: String, editor_binary: String = "") -> int:
	if editor_binary.is_empty():
		match editor_type:
			Globals.GODOT_3:
				editor_binary = default_godot_3_binary
			Globals.GODOT_4:
				editor_binary = default_godot_4_binary
			_:
				logger.error("Tried to execute project %s with an unknown Godot version" %
					project_path)
				return ERR_UNCONFIGURED
	
	var binary: EditorBinary = editor_binaries.get(editor_binary, null)
	if binary == null:
		logger.error("Tried to execute project %s with an unknown editor binary %s" % [
			project_path, editor_binary])
		return ERR_UNCONFIGURED
	
	var pid := OS.create_process(binary.path, [project_path] + binary.default_args)
	if pid == -1:
		logger.error("Unable to start project %s using binary %s with args %s" % [
			project_path,
			binary.path,
			str(binary.default_args)
		])
		return ERR_CANT_FORK
	
	logger.info("Started project %s with PID %d" % [project_path, pid])
	
	return OK
