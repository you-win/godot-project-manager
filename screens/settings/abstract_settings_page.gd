class_name AbstractSettingsPage
extends ScrollContainer

## Emitted when a config change has occurred where all settings pages must update.
signal config_updated()

@export
var page_name := ""

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

## Force the page to update itself. [code]config_updated[/code] should never be emitted
## from this function as that would cause an infinite loop.
func update() -> void:
	assert(false, "Not yet implemented")
