class_name AppManager
extends Node

signal log_received(text: String)
signal received_latest_godot_release(text: String)

const WAITING_FOR_RESPONSE := "LATEST_GODOT_RELEASE_WAITING_FOR_RESPONSE"
const RESPONSE_ERROR := "LATEST_GODOT_RELEASE_RESPONSE_ERROR"

var logger := Logger.new("AppManager")

var all_logs := true
var latest_godot_release := WAITING_FOR_RESPONSE :
	set(text):
		latest_godot_release = ("Latest Godot release: %s" % text) if text != RESPONSE_ERROR else \
			RESPONSE_ERROR
		received_latest_godot_release.emit(latest_godot_release)

var config: Config = null
var trying_to_save := false
var debounce_map := {}

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

## Note: the logger cannot be used here since the AppManager won't be accessible yet
func _init() -> void:
	Logger.add_hook(func(text: String) -> void:
		self.log_received.emit(text)
	)
	randomize()
	
	logger.info(ProjectSettings.globalize_path(Config.SAVE_PATH))
	
	var res := read_config()
	if res.is_err():
		logger.error(res.to_string())
		config = Config.new()
	else:
		config = res.unwrap()

func _ready() -> void:
	_get_latest_godot_release()

func _exit_tree() -> void:
	var res := _write_config()
	if res.is_err():
		pass
		logger.error(res.to_string())
	
	logger.info("Goodbye!")

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

func _get_latest_godot_release() -> void:
	var http_request := HTTPRequest.new()
	add_child(http_request)
	
	http_request.request_completed.connect(
		func(_result: int, code: int, _headers: PackedStringArray, body: PackedByteArray) -> void:
			if code != 200:
				logger.error("Bad response code while getting the latest Godot release")
				latest_godot_release = RESPONSE_ERROR
				return
			
			var data: Variant = JSON.parse_string(body.get_string_from_utf8())
			if data == null:
				logger.error("Bad response body while getting the latest Godot release")
				latest_godot_release = RESPONSE_ERROR
				return
			
			if typeof(data) != TYPE_DICTIONARY:
				logger.error("Unexpected response body while getting the latest Godot release")
				latest_godot_release = RESPONSE_ERROR
				return
			
			latest_godot_release = data.get("tag_name", RESPONSE_ERROR)
			
			logger.info("Found the latest Godot release: %s" % latest_godot_release)
	)
	
	const LATEST_RELEASE = "https://api.github.com/repos/godotengine/godot/releases/latest"
	
	logger.info("Getting the latest Godot release from GitHub.")
	
	http_request.request(LATEST_RELEASE, [
		"Accept: application/vnd.github+json",
		"User-Agent: GodotProjectManager/1.0"
	])
	
	await http_request.request_completed
	
	http_request.queue_free()

func _write_config() -> Result:
	if ResourceSaver.save(config, Config.SAVE_PATH) != OK:
		return Safely.err(Error.Code.CONFIG_WRITE_ERROR, Config.SAVE_PATH)
	
	return Safely.ok()

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func read_config() -> Result:
	logger.debug("Reading config")
	
	var config: Resource = ResourceLoader.load(Config.SAVE_PATH, "Config")
	if not config is Config:
		return Safely.err(Error.Code.CONFIG_READ_ERROR, Config.SAVE_PATH)
	
	logger.debug("Finished reading config")
	
	return Safely.ok(config)

func write_config() -> Result:
	if trying_to_save:
		return Safely.err(Error.Code.TRYING_TO_SAVE)
	
	logger.debug("Writing config")
	
	trying_to_save = true
	await debounce(write_config, config.debounce_time)
	
	if ResourceSaver.save(config, Config.SAVE_PATH) != OK:
		return Safely.err(Error.Code.CONFIG_WRITE_ERROR, Config.SAVE_PATH)
	
	trying_to_save = false
	
	logger.debug("Finished writing config")
	
	return Safely.ok()

func debounce(key: Variant, time: float) -> void:
	if debounce_map.get(key, false):
		return
	
	debounce_map[key] = true
	await get_tree().create_timer(time).timeout
	debounce_map[key] = false
