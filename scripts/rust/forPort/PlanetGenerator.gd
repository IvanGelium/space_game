
class_name PlanetGeneratorDepr
extends Node2D

var noise = FastNoiseLite.new()

# Хранилище плотности (для всей планеты)
var density_map = PackedFloat32Array()

func setup():
	noise.seed = randi()
	noise.frequency = 0.04
	density_map.resize(Config.MAP_SIZE * Config.MAP_SIZE)

# 1. Сначала заполняем карту плотности (SDF)
func generate_density(config: PlanetConfig):
	var radius = config.radius
	var center = Vector2(Config.MAP_SIZE / 2.0, Config.MAP_SIZE / 2.0)
	
	for y in range(Config.MAP_SIZE):
		for x in range(Config.MAP_SIZE):
			var pos = Vector2(x, y)
			var offset = pos - center
			var d = radius - offset.length()
			d += noise.get_noise_2d(x, y) * 5.0 # Сила шума
			density_map[x + y * Config.MAP_SIZE] = d

# 2. Метод для получения геометрии конкретного чанка
func get_chunk_geometry(chunk_pos: Vector2i) -> PackedVector2Array:
	var vertices = PackedVector2Array()
	
	# Проходим по ячейкам внутри чанка
	for y in range(chunk_pos.y, chunk_pos.y + Config.CHUNK_SIZE):
		for x in range(chunk_pos.x, chunk_pos.x + Config.CHUNK_SIZE):
			if x >= Config.MAP_SIZE - 1 or y >= Config.MAP_SIZE - 1: continue
			
			# Получаем значения плотности в 4 углах ячейки
			var d0 = _get_d(x, y)     # Лево-верх
			var d1 = _get_d(x+1, y)   # Право-верх
			var d2 = _get_d(x+1, y+1) # Право-низ
			var d3 = _get_d(x, y+1)   # Лево-низ
			
			# Вычисляем индекс случая (Case Index) от 0 до 15
			var case_index = 0
			if d0 > 0: case_index |= 1
			if d1 > 0: case_index |= 2
			if d2 > 0: case_index |= 4
			if d3 > 0: case_index |= 8
			
			# Генерируем линии для этого случая
			if case_index > 0 and case_index < 15:
				_add_ms_edges(vertices, x, y, d0, d1, d2, d3, case_index)
				
	return vertices

func _add_ms_edges(out_pts: PackedVector2Array, x: int, y: int, d0: float, d1: float, d2: float, d3: float, case: int):
	#x *= Config.CHUNK_SIZE
	#y *= Config.CHUNK_SIZE
	var p0 = Vector2(x, y)
	var p1 = Vector2(x + 1, y)
	var p2 = Vector2(x + 1, y + 1)
	var p3 = Vector2(x, y + 1)
	
	# Точки на ребрах с интерполяцией
	var t = _get_interp(p0, p1, d0, d1) # Top
	var r = _get_interp(p1, p2, d1, d2) # Right
	var b = _get_interp(p2, p3, d2, d3) # Bottom
	var l = _get_interp(p3, p0, d3, d0) # Left
	
	match case:
		0, 15: # Пусто или Полностью заполнено
			pass
		1: # Лево-Верх (d0)
			out_pts.append_array([l, t])
		2: # Право-Верх (d1)
			out_pts.append_array([t, r])
		3: # Верхняя грань заполнена (d0, d1)
			out_pts.append_array([l, r])
		4: # Право-Низ (d2)
			out_pts.append_array([r, b])
		5: # Диагональ (d0, d2) - особый случай
			out_pts.append_array([l, t, r, b])
		6: # Правая грань заполнена (d1, d2)
			out_pts.append_array([t, b])
		7: # Кроме Лево-Низ
			out_pts.append_array([l, b])
		8: # Лево-Низ (d3)
			out_pts.append_array([b, l])
		9: # Левая грань заполнена (d0, d3)
			out_pts.append_array([b, t])
		10: # Диагональ (d1, d3) - особый случай
			out_pts.append_array([t, l, b, r])
		11: # Кроме Право-Низ
			out_pts.append_array([t, r]) # Исправлено для контура
			out_pts.append_array([r, b]) # (в зависимости от инверсии)
			# Для упрощения:
			out_pts.append_array([r, b])
		12: # Нижняя грань заполнена (d2, d3)
			out_pts.append_array([r, l])
		13: # Кроме Право-Верх
			out_pts.append_array([t, r])
		14: # Кроме Лево-Верх
			out_pts.append_array([l, t])


func _get_interp(p1: Vector2, p2: Vector2, val1: float, val2: float) -> Vector2:
	var mu = (0.0 - val1) / (val2 - val1)
	return p1 + mu * (p2 - p1)

func _get_d(x: int, y: int) -> float:
	return density_map[x + y * Config.MAP_SIZE]

func set_density(x: int, y: int, amount: float):
	if x < 0 or x >= Config.MAP_SIZE or y < 0 or y >= Config.MAP_SIZE:
		return
		
	var idx = x + y * Config.MAP_SIZE

	var new_density = density_map[idx] + amount
	
	# Ограничиваем значения (например, от -50 до 50)
	# Это нужно, чтобы поверхность реагировала на изменения сразу
	density_map[idx] = clamp(new_density, -50.0, 50.0)
