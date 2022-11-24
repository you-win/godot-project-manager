extends VBoxContainer

const TREE_COL: int = 0

@onready
var content: HSplitContainer = $Content
@onready
var tree: Tree = $Content/Tree

var pages := {}

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _ready() -> void:
	content.split_offset = content.size.x * 0.2
	
	var root: TreeItem = tree.create_item()
	
	var first := true
	for child in content.get_children():
		if child == tree:
			continue
		
		child.config_updated.connect(func() -> void:
			for page in pages.values():
				page.update()
		)
		child.update()
		
		var item := root.create_child()
		var item_name: String = tr(child.page_name)
		item.set_text(TREE_COL, item_name)
		pages[item_name] = child
		
		if first:
			first = false
			item.select(TREE_COL)
	
	tree.item_selected.connect(func() -> void:
		for i in pages.values():
			i.hide()
		pages[tree.get_selected().get_text(TREE_COL)].show()
	)

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
