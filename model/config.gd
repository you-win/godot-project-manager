class_name Config
extends Resource

## The configuration data for the project manager.

const SAVE_PATH := "user://config.tres"

const DEFAULT_DEBOUNCE_TIME: float = 1.0
const DEFAULT_FILE_SEARCH_PATH := "user://"

## The paths to do any scanning in. If there are no scan paths, will open in the app's directory.
@export
var scan_paths := []
## The default path to use when selecting files.
@export
var default_file_search_path := DEFAULT_FILE_SEARCH_PATH
## Time it takes to debounce in general.
@export
var debounce_time: float = DEFAULT_DEBOUNCE_TIME
## All registered Godot binaries.
@export
var editor_binaries := {}
## The default editor binary to use when running a project. Separate binaries can be
## registered according to [code]EditorBinary::Type[/code], as projects contain
## the necessary metadata to differentiate between Godot 3/4.
@export
var default_editor_binaries := {}
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
	default_editor_binaries.clear()
	
	known_projects.clear()
