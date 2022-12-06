extends HSplitContainer

const Project: PackedScene = preload("res://screens/projects/project.tscn")

const SORT_OPTIONS := [
	"Name",
	"Path",
	"Last Modified"
]

const CONFIG_VERSION_KEY := "config_version"

# NOTE: for now, these are the same between Godot 3/4
const CONFIG_APPLICATION_SECTION := "application"
const CONFIG_PROJECT_NAME_KEY := "config/name"
const CONFIG_PROJECT_ICON := "config/icon"

const DEFAULT_PROJECT_NAME := "PROJECTS_MISSING_PROJECT_NAME"
const MISSING_CONFIG_VERSION := "PROJECTS_MISSING_CONFIG_VERSION"

const GODOT_3_CONFIG_VERSION: int = 4
const GODOT_4_CONFIG_VERSION: int = 5

@onready
var filter_line_edit := $Left/HFlowContainer/Filter
@onready
var sort_button := $Left/HFlowContainer/Sort/Sort

@onready
var projects := $Left/ScrollContainer/Projects

var logger := Logger.new("Projects")

var _last_size_x: float = 1.0
var _last_selected_project: Control = null

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	for i in SORT_OPTIONS:
		sort_button.add_item(i)
	
	%Edit.pressed.connect(func() -> void:
		pass
	)
	%Run.pressed.connect(func() -> void:
		pass
	)
	%Scan.pressed.connect(func() -> void:
		_scan()
		_populate_projects()
	)
	%NewProject.pressed.connect(func() -> void:
		pass
	)
	%Import.pressed.connect(func() -> void:
		pass
	)
	%Rename.pressed.connect(func() -> void:
		pass
	)
	%Remove.pressed.connect(func() -> void:
		pass
	)
	%RemoveMissing.pressed.connect(func() -> void:
		logger.info("Removing removing missing projects")
		
		var missing_projects := PackedStringArray()
		for path in AM.config.known_projects:
			if FileAccess.file_exists(path):
				continue
			
			missing_projects.append(path)
		
		for path in missing_projects:
			AM.config.known_projects.erase(path)
		
		logger.info("Finished removing missing projects")
		
		_scan()
		_populate_projects()
	)
	
	_last_size_x = size.x
	split_offset = _last_size_x * 0.8
	
	get_viewport().size_changed.connect(func() -> void:
		split_offset = size.x * (split_offset / _last_size_x)
		_last_size_x = size.x
	)
	
	_populate_projects()

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

## Scans all configured scan paths for projects and stores them in the [code]Config[/code].
## Does not populate projects by itself.
func _scan() -> void:
	logger.info("Scanning for projects")
	
	for i in AM.config.scan_paths:
		logger.debug("Scanning path %s" % i)
		
		var dir := DirAccess.open(i)
		if dir == null:
			logger.error("Cannot access scan path: %s" % i)
			continue
		
		dir.include_hidden = true
		dir.include_navigational = false
		dir.list_dir_begin()
		
		var file_name := "next"
		while not file_name.is_empty():
			file_name = dir.get_next()
			
			if not dir.current_is_dir():
				continue
			
			var project_path := "%s/%s" % [i, file_name]
			var project_file_path := "%s/project.godot" % [project_path]
			var file := FileAccess.open(project_file_path, FileAccess.READ)
			if file == null:
				logger.debug("Cannot access project file at: %s" % project_file_path)
				continue
			
			var config := ConfigFile.new()
			if config.parse(file.get_as_text()) != OK:
				logger.error("Unable to parse project file at: %s" % project_file_path)
				continue
			
			var project_name: String = config.get_value(
				CONFIG_APPLICATION_SECTION, CONFIG_PROJECT_NAME_KEY, DEFAULT_PROJECT_NAME)
			
			# TODO not really using this for anything i guess
			var config_version: String = str(config.get_value("", CONFIG_VERSION_KEY, 0))
			match config_version:
				GODOT_3_CONFIG_VERSION:
					logger.debug("Found Godot 3 project: %s" % project_name)
				GODOT_4_CONFIG_VERSION:
					logger.debug("Found Godot 4 project: %s" % project_name)
				_:
					logger.debug("Found unknown Godot project: %s" % project_name)
			
			if project_file_path in AM.config.known_projects:
				continue
			
			AM.config.known_projects.append(project_file_path)
	
	logger.info("Finished scanning for projects")

## Iterates through all known projects and creates a UI element for each project.
func _populate_projects() -> void:
	logger.info("Populating projects")
	
	for child in projects.get_children():
		child.free()
	
	for path in AM.config.known_projects:
		var file := FileAccess.open(path, FileAccess.READ)
		if file == null:
			logger.debug("Cannot access project file at: %s" % path)
			return
		
		var containing_directory: String = path.trim_suffix(path.get_file())
		
		var config := ConfigFile.new()
		if config.parse(file.get_as_text()) != OK:
			logger.error("Unable to parse project file at: %s" % path)
			continue
		
		var project := Project.instantiate()
		projects.add_child(project)
		
		project.selected.connect(func() -> void:
			if _last_selected_project != null:
				_last_selected_project.selected_highlight.hide()
			
			_last_selected_project = project
			_last_selected_project.selected_highlight.show()
		)
		
		project.title.text = config.get_value(
			CONFIG_APPLICATION_SECTION, CONFIG_PROJECT_NAME_KEY, DEFAULT_PROJECT_NAME)
		match config.get_value("", CONFIG_VERSION_KEY, -1):
			3, 4:
				project.godot_version.text = Globals.GODOT_3
			5:
				project.godot_version.text = Globals.GODOT_4
			_:
				project.godot_version.text = MISSING_CONFIG_VERSION
		project.path.text = containing_directory
	
	logger.info("Finished populating projects")

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
