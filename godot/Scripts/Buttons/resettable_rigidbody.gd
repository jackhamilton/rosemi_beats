extends RigidBody2D
class_name ResettableRigidbody2D

@export var scene_root: SceneRoot

var reset_position = false

func _ready() -> void:
	scene_root.start_game.connect(initiate_reset)

func _integrate_forces(state: PhysicsDirectBodyState2D) -> void:
	var xform = state.get_transform()
	var parent = get_parent().get_position()
	if reset_position:
		xform.origin.x = parent.x
		xform.origin.y = parent.y
	state.set_transform(xform)
	reset_position = false

func initiate_reset():
	reset_position = true
