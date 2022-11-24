class_name Error
extends RefCounted

enum Code {
	NONE = 0,
	
	NULL_VALUE,
	TRYING_TO_SAVE,
	
	CANNOT_OPEN_DIRECTORY,
	CANNOT_OPEN_FILE,
	
	CONFIG_READ_ERROR,
	CONFIG_WRITE_ERROR,
}

var code := Code.NONE :
	set(value):
		code = value
		name = Code.keys()[value]
var description := ""
var name := ""

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _init(code: Code, description: String = "") -> void:
	self.code = code
	self.description = description

func _to_string() -> String:
	return "Code: %s, Name: %s, Description: %s" % [code, name, description]

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
