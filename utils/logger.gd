class_name Logger
extends RefCounted

## A logger.

## The various log types.
const LogTypes := {
	"INFO": "[INFO]",
	"DEBUG": "[DEBUG]",
	"TRACE": "[TRACE]",
	"ERROR": "[ERROR]",
	"GLOBAL": "[GLOBAL]"
}

## All logs that are generated are also kept in memory.
const LOGS: Array[String] = []
## The max logs to be held in memory. Logs over this amount will be popped from [code]LOGS[/code]
## but will still be stored in the appropriate log file.
const MAX_LOGS: int = 10000

## Called whenever a log is generated.
const HOOKS: Array[Callable] = []

## Used to identify where a log came from. Not useful when identifying global logs.
const DEFAULT_NAME := "DefaultLogger"

var name := DEFAULT_NAME

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _init(v: Variant = null) -> void:
	if v == null:
		return
	
	if v is String:
		name = v
	elif v is Node:
		name = v.name
	else:
		name = str(v)

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

static func _log(prefix: String, id: String, text: String) -> void:
	var datetime := Time.get_datetime_dict_from_system()
	
	text = "%s %s-%s-%s_%s:%s:%s %s %s" % [
		prefix,
		datetime.year,
		datetime.month,
		datetime.day,
		datetime.hour,
		datetime.minute,
		datetime.second,
		id,
		text
	]
	
	print(text)
	LOGS.append(text)
	while LOGS.size() > MAX_LOGS:
		LOGS.pop_front()
	
	for i in get_hooks():
		i.call(text)

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

## Generate an info log.
func info(text: String) -> void:
	_log("[INFO]", name, text)

## Generate a debug log. Only fires in debug builds or if [code]AppManager.all_logs[/code] is true.
func debug(text: String) -> void:
	if not (OS.is_debug_build() or AM.all_logs):
		return
	
	_log("[DEBUG]", name, text)

## Generate a trace log. Only fires in debug builds or if [code]AppManager.all_logs[/code] is true.
## The trace will not include the calls to the [code]Logger[/code].
func trace(text: String) -> void:
	if not (OS.is_debug_build() or AM.all_logs):
		return
	
	var stack := get_stack()
	for i in stack.size() - 2:
		var data: Dictionary = stack[i + 2]
		text = "%s\n%t%d - %s:%d - %s" % [
			text, i, data.source, data.line, data.function
		]
	
	_log("[TRACE]", name, text)

## Generate an error log.
func error(text: String) -> void:
	_log("[ERROR]", name, text)

## Log something without needing a [code]Logger[/code] instance.
static func global(text: String) -> void:
	_log("[GLOBAL]", DEFAULT_NAME, text)

## Add a hook to be called when anything is logged.
static func add_hook(hook: Callable) -> void:
	HOOKS.append(hook)

static func clear_hooks() -> void:
	HOOKS.clear()

## Get all logs that have not been flushed.
static func get_logs() -> PackedStringArray:
	return PackedStringArray(LOGS)

static func clear_logs() -> void:
	LOGS.clear()

## Get all hooks.
static func get_hooks() -> Array[Callable]:
	return HOOKS
