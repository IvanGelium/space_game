extends Node2D

@onready var renderer: PlanetRenderer = $Planet_renderer 
@onready var generator: PlanetGeneratorDepr = $Planet_generator_depr
@onready var camera: Camera2D = $Camera2D
#@onready var generator_rust = $PlanetGenerator # Имя должно совпадать с именем в дереве
@onready var world_api: = $WorldAPI

func _ready():
	generator.setup()
	renderer.setup()
	world_api.setup_generator(512);
	world_api.generate_density()
	generate_logic()

func _process(_delta):
	update_visible_chunks()

func generate_logic():
	var config = PlanetConfig.new().set_radius(350)
	generator.generate_density(config)

	for y in range(0, Config.MAP_SIZE - Config.CHUNK_SIZE, Config.CHUNK_SIZE):
		for x in range(0, Config.MAP_SIZE - Config.CHUNK_SIZE, Config.CHUNK_SIZE):
			var chunk_pos = Vector2i(x, y)
			var raw_vertices = generator.get_chunk_geometry(chunk_pos)
			
			if raw_vertices.size() > 0:
				# Сшиваем отрезки в замкнутый контур
				var closed_polygon = _stitch_segments(raw_vertices)
				
				# Отправляем в рендерер
				var chunk_id = Vector2i(x / Config.CHUNK_SIZE, y / Config.CHUNK_SIZE)
				renderer.update_chunk_visual(chunk_id, closed_polygon)

# Алгоритм сшивания отрезков в один (или несколько) полигонов
func _stitch_segments(segments: PackedVector2Array) -> PackedVector2Array:
	if segments.size() < 2: return PackedVector2Array()
	
	# Т.к. Marching Squares выдает пары [A, B, C, D...] где AB - отрезок, CD - отрезок
	# Нам нужно выстроить их друг за другом.
	# Для MVP мы просто вернем их как есть, но для Polygon2D в идеале 
	# нужно найти соседние точки. 
	# ВАЖНО: Если мы используем Line2D с закрытым контуром, 
	# массив должен идти по порядку часовой стрелки.
	
	# Временное решение для отображения (просто возвращаем массив):
	# Чтобы Polygon2D работал без артефактов, Marching Squares должен быть полным.
	return segments

func update_visible_chunks():
	# 1. Получаем область видимости камеры в мировых координатах
	var screen_size = get_viewport_rect().size / camera.zoom
	var top_left = camera.get_screen_center_position() - screen_size / 2.0
	
	# 2. Определяем диапазон индексов чанков
	var start_x = floor(top_left.x / (Config.CHUNK_SIZE * Config.GRID_STEP))
	var start_y = floor(top_left.y / (Config.CHUNK_SIZE * Config.GRID_STEP))
	var end_x = start_x + ceil(screen_size.x / (Config.CHUNK_SIZE * Config.GRID_STEP))
	var end_y = start_y + ceil(screen_size.y / (Config.CHUNK_SIZE * Config.GRID_STEP))

	# 3. Проходим по чанкам в зоне видимости
	for y in range(start_y, end_y + 1):
		for x in range(start_x, end_x + 1):
			var chunk_id = Vector2i(x, y)
			
			# Если чанк в пределах карты и еще не отрисован
			if is_inside_map(chunk_id) and not renderer.has_chunk(chunk_id):
				render_chunk(chunk_id)

func is_inside_map(cid):
	var max_chunks = Config.MAP_SIZE / Config.CHUNK_SIZE
	return cid.x >= 0 and cid.y >= 0 and cid.x < max_chunks and cid.y < max_chunks
	
	
	
func _input(event):
	if event is InputEventMouseButton and event.pressed:
		# 1. Позиция мыши в мире
		var mouse_world_pos = get_local_mouse_position()
		var grid_pos = mouse_world_pos
		
		if event.button_index == MOUSE_BUTTON_LEFT:
			modify_terrain(grid_pos, -20.0) # Сильно копаем
		elif event.button_index == MOUSE_BUTTON_RIGHT:
			modify_terrain(grid_pos, 20.0)  # Сильно строим

#func modify_terrain(grid_pos: Vector2, amount: float):
	## Увеличим радиус и силу, чтобы эффект был мгновенным
	#print("Grid Click:", grid_pos) # Должно быть в пределах от 0 до 800
	#var brush_radius = 8.0 
	#var brush_strength = amount * 100.0 # Усиливаем воздействие
	#
	#for y in range(int(grid_pos.y - brush_radius), int(grid_pos.y + brush_radius)):
		#for x in range(int(grid_pos.x - brush_radius), int(grid_pos.x + brush_radius)):
			#var p = Vector2(x, y)
			#if grid_pos.distance_to(p) < brush_radius:
				#generator.set_density(x, y, brush_strength)
	#
	#var affected_chunks = get_affected_chunks(grid_pos, brush_radius)
	#for cid in affected_chunks:
		## Вычисляем мировые координаты начала чанка для генератора
		#var chunk_origin = cid * Config.CHUNK_SIZE
		#var vertices = generator.get_chunk_geometry(chunk_origin)
		#renderer.update_chunk_visual(cid, vertices)
 	#
	#print("Изменен террейн в ", grid_pos, ". Обновлено чанков: ", affected_chunks.size())

func modify_terrain(grid_pos: Vector2, amount: float):
	# 1. Вызываем тяжелую логику в Rust
	generator.modify_terrain(grid_pos, 15.0, amount)
	
	# 2. Обновляем только затронутые чанки
	var affected = get_affected_chunks(grid_pos, 15.0)
	for cid in affected:
		var chunk_origin = cid * Config.CHUNK_SIZE
		# Получаем УЖЕ готовую геометрию из Rust
		var vertices = generator.get_chunk_geometry(chunk_origin)
		renderer.update_chunk_visual(cid, vertices)


func render_chunk(cid):
	var chunk_origin = cid * Config.CHUNK_SIZE
	var vertices = generator.get_chunk_geometry(chunk_origin)
	# Мы добавили метод force_update в рендерер, чтобы он заменял старый полигон
	renderer.update_chunk_visual(cid, vertices)
	
	
func get_affected_chunks(grid_pos: Vector2, brush_radius: float) -> Array[Vector2i]:
	var affected: Array[Vector2i] = []
	var chunk_size = Config.CHUNK_SIZE
	
	# 1. Находим границы области (Bounding Box) кисти в координатах сетки
	var min_grid_x = grid_pos.x - brush_radius
	var max_grid_x = grid_pos.x + brush_radius
	var min_grid_y = grid_pos.y - brush_radius
	var max_grid_y = grid_pos.y + brush_radius
	
	# 2. Переводим границы сетки в индексы чанков (используем floor, чтобы округлить вниз)
	var min_chunk_x = int(floor(min_grid_x / chunk_size))
	var max_chunk_x = int(floor(max_grid_x / chunk_size))
	var min_chunk_y = int(floor(min_grid_y / chunk_size))
	var max_chunk_y = int(floor(max_grid_y / chunk_size))
	
	# 3. Проходим циклом по всем чанкам, попавшим в этот квадрат
	for y in range(min_chunk_y, max_chunk_y + 1):
		for x in range(min_chunk_x, max_chunk_x + 1):
			var cid = Vector2i(x, y)
			# Проверяем, что чанк вообще существует в пределах карты
			if is_inside_map(cid):
				affected.append(cid)
				
	return affected
