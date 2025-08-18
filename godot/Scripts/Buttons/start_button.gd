extends Button

@export var scene_root: SceneRoot

func _on_pressed() -> void:
	scene_root.start_game.emit()
