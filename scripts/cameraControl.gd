extends Camera2D

var zoom_speed = 0.1
var min_zoom = 0.1
var max_zoom = 10.0
var drag_sensitivity = 1.0

func _input(event):
	# Зум колесиком мыши
	if event is InputEventMouseButton:
		if event.is_pressed():
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				zoom_camera(zoom_speed)
			if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				zoom_camera(-zoom_speed)

	# Перемещение камерой (Drag)
	if event is InputEventMouseMotion:
		if  Input.is_mouse_button_pressed(MOUSE_BUTTON_MIDDLE):
			# Двигаем камеру против движения мыши, учитывая текущий зум
			position -= event.relative * (1.0 / zoom.x) * drag_sensitivity

func zoom_camera(delta):
	var new_zoom = clamp(zoom.x + delta, min_zoom, max_zoom)
	zoom = Vector2(new_zoom, new_zoom)
