extends RigidBody2D

@export var timeToFade = 1.5
var timePassed = 0
var rps = 0.0

func _enter_tree() -> void:
	rps = randf_range(-3.14, 3.14)

func _process(delta: float) -> void:
	timePassed += delta
	apply_torque_impulse(rps)
	self.modulate.a = 1 - (timePassed / timeToFade) * 1
	if self.modulate.a <= 0:
		self.queue_free()
