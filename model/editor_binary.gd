class_name EditorBinary
extends Resource

enum Type {
	NONE = 0,
	
	GODOT_3,
	GODOT_4,
	
	OTHER
}

@export
var name := ""
@export
var path := ""
@export
var type := Type.NONE
@export
var default_args: Array[String] = []

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

## [code]path[/code] should never use its default value.
func _init(
		name: String = "",
		path: String = "",
		type: Type = Type.NONE,
		default_args: PackedStringArray = PackedStringArray()
) -> void:
	self.name = name
	self.path = path
	self.type = type
	self.default_args.append_array(default_args)

func _to_string() -> String:
	return "EditorBinary - Name: %s, Path: %s, Type: %d, Args: %s" % [
		name, path, type, default_args]

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
