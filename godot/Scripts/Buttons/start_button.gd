extends Button

var main_scene = preload("res://main.tscn")

func _on_pressed() -> void:
	var main = main_scene.instantiate()
	get_tree().root.add_child(main)
	get_node("/root/StartMenu").queue_free()
