extends Control
class_name DebugNode

func _process(_delta: float) -> void:
	print(get_viewport().gui_get_hovered_control())
