extends Node
class_name ControlsManager

signal changed

const CONTROLS_SAVE_PATH := "user://controls.tres"
@export var control_menu: ControlMenu

func _enter_tree() -> void:
	load_controls()
	control_menu.remapped.connect(save_controls)

func save_controls() -> void:
	var actions := InputMap.get_actions()
	var data := ControlsData.new()
	for action in actions:
		if action.begins_with("editor_") or action.begins_with("ui_"):
			continue
		data.controls[action] = InputMap.action_get_events(action)
	var error := ResourceSaver.save(data, CONTROLS_SAVE_PATH)
	if error != OK:
		printerr("Failed to save controls! Error: ", error_string(error))

func load_controls() -> void:
	if not ResourceLoader.exists(CONTROLS_SAVE_PATH, &"ControlsData"):
		printerr("No saved controls data in ", CONTROLS_SAVE_PATH)
		return

	var data: ControlsData = ResourceLoader.load(CONTROLS_SAVE_PATH, &"ControlsData")
	if not is_instance_valid(data):
		printerr("Failed to load controls!")
		return

	for action in data.controls.keys():
		InputMap.action_erase_events(action)
		for event in data.controls[action]:
			InputMap.action_add_event(action, event)

	changed.emit()
