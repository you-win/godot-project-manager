class_name Result
extends RefCounted

var value: Variant = null

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _init(value: Variant) -> void:
	self.value = value

func _to_string() -> String:
	return "%s: %s" % ["Error" if is_err() else "Result", str(value)]

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func is_ok() -> bool:
	return not is_err()

func is_err() -> bool:
	return value is Error

func unwrap() -> Variant:
	assert(not value is Error)
	return value

func unwrap_err() -> Error:
	assert(value is Error)
	return value
