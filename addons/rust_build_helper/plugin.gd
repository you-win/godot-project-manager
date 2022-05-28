tool
extends EditorPlugin

const TOOLBAR_NAME := "Rust Builder"
const Runner := preload("res://addons/rust_build_helper/runner.tscn")
var runner: Control

func _enter_tree():
	runner = Runner.instance()
	inject_tool(runner)
	runner.plugin = self
	
	add_control_to_bottom_panel(runner, TOOLBAR_NAME)

func _exit_tree():
	if runner != null:
		remove_control_from_bottom_panel(runner)
		runner.free()

## Inject `tool` at the top of the Node's script
##
## @param: node: Node - The node to inject
##
## @return: int - The error code
func inject_tool(node: Node) -> int:
	var script: Script = node.get_script().duplicate()
	script.source_code = "tool\n%s" % script.source_code
	
	var err: int = script.reload()
	if err != OK:
		printerr("Unable to inject \"tool\" into node %s" % node.name)
		return err
	
	node.set_script(script)
	
	return OK
