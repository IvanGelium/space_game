#extends TextureRect
#class_name PlanetRenderer
#
#var image: Image
#var texture_rect: ImageTexture
#
#func setup():
	#image = Image.create(Config.MAP_SIZE, Config.MAP_SIZE, false, Image.FORMAT_RGBA8)
	#texture_rect = ImageTexture.create_from_image(image)
	#self.texture = texture_rect
	#self.texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST
#
## Теперь на вход идет только готовый массив цветов
#func update_frame(pixel_data: PackedColorArray):
	## Image.set_data_from_array — это встроенный метод, 
	## который работает на порядки быстрее ручного цикла set_pixel
	## Но для него массив должен быть правильно сконвертирован в байты.
	## Для простоты пока оставим цикл, но уже без логики выбора цвета:
	#
	#for i in range(pixel_data.size()):
		#var x = i % image.get_width()
		#var y = i / image.get_width()
		#image.set_pixel(x, y, pixel_data[i])
	#
	#texture_rect.update(image)

extends Node2D
class_name PlanetRenderer

# Настройки отображения
@export var fill_color: Color = Color(0.3, 0.3, 0.3, 1.0)

var active_chunks: Dictionary = {}

func has_chunk(cid: Vector2i) -> bool:
	return active_chunks.has(cid)

func setup():
	# Очищаем старые чанки при инициализации
	for child in get_children():
		child.queue_free()

#func update_chunk_visual(chunk_id: Vector2i, vertices: PackedVector2Array):
	#var chunk_node: StaticBody2D
	#
	## 1. Проверяем наличие чанка в словаре (это быстрее, чем поиск по имени ноды)
	#if active_chunks.has(chunk_id):
		#chunk_node = active_chunks[chunk_id]
	#else:
		## Если нет — создаем, называем и регистрируем в словаре
		#var chunk_name = "Chunk_%d_%d" % [chunk_id.x, chunk_id.y]
		#chunk_node = _create_chunk_node(chunk_name)
		#active_chunks[chunk_id] = chunk_node
	#
	## Ссылки на компоненты внутри чанка (используем кэшированные имена из _create_chunk_node)
	#var polygon_node = chunk_node.get_node("Polygon")
	#var collision_node = chunk_node.get_node("Collision")
	#var line_node = chunk_node.get_node("Line")
	#
	## 2. Обновляем геометрию
	#if vertices.size() > 2:
		## Визуал (заливка)
		#polygon_node.polygon = vertices
		#polygon_node.color = fill_color
		#
		## Линия контура (для четкости)
		#line_node.points = vertices
		#line_node.width = line_width
		#line_node.default_color = line_color
		#
		## Физика (обновляем асинхронно, чтобы не "вешать" основной поток)
		##collision_node.set_deferred("polygon", vertices)
	#else:
		## Очистка, если в чанке больше нет материи (например, всё выкопали)
		#polygon_node.polygon = PackedVector2Array()
		#line_node.points = PackedVector2Array()
		#collision_node.set_deferred("polygon", PackedVector2Array())

func update_chunk_visual(chunk_id: Vector2i, mesh_data: Dictionary):
	var chunk_node: StaticBody2D
	
	if active_chunks.has(chunk_id):
		chunk_node = active_chunks[chunk_id]
	else:
		var chunk_name = "Chunk_%d_%d" % [chunk_id.x, chunk_id.y]
		chunk_node = _create_chunk_node(chunk_name)
		active_chunks[chunk_id] = chunk_node
	
	var mesh_instance = chunk_node.get_node("MeshInstance")
	var collision_node = chunk_node.get_node("Collision")

	var vertices = mesh_data.get("vertices", PackedVector2Array())
	var indices = mesh_data.get("indices", PackedInt32Array())

	if vertices.size() > 0:
		# Создаем ArrayMesh
		var array_mesh = ArrayMesh.new()
		var arrays = []
		arrays.resize(Mesh.ARRAY_MAX)
		arrays[Mesh.ARRAY_VERTEX] = vertices
		arrays[Mesh.ARRAY_INDEX] = indices

		array_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)
		
		# Обновляем визуал
		mesh_instance.mesh = array_mesh
		
		# Обновляем физику (ConcavePolygonShape2D идеально ест сырые данные меша)
		var shape = ConcavePolygonShape2D.new()
		# Важно: Concave требует сегменты, поэтому мы скармливаем ему 
		# развернутый в линии меш. Для 2D земли это самый стабильный вариант.
		shape.set_segments(vertices) 
		collision_node.shape = shape
	else:
		mesh_instance.mesh = null
		collision_node.shape = null


# Вспомогательная функция для сборки узла чанка
#func _create_chunk_node(node_name: String) -> StaticBody2D:
	#var node = StaticBody2D.new()
	#node.name = node_name
	#add_child(node)
	#
	## Узел для заливки цветом
	#var poly = Polygon2D.new()
	#poly.name = "Polygon"
	#node.add_child(poly)
	#
	## Узел для рисования красивой линии контура
	#var line = Line2D.new()
	#line.name = "Line"
	#line.closed = true
	#node.add_child(line)
	#
	## Узел для физических столкновений
	#var coll = CollisionPolygon2D.new()
	#coll.name = "Collision"
	#node.add_child(coll)
	#
	#return node


func _create_chunk_node(node_name: String) -> StaticBody2D:
	var node = StaticBody2D.new()
	node.name = node_name
	add_child(node)
	
	# Теперь вместо Polygon2D используем MeshInstance2D
	var mesh_i = MeshInstance2D.new()
	mesh_i.name = "MeshInstance"
	# Можно задать дефолтный цвет через Self Modulate или материал
	mesh_i.self_modulate = fill_color 
	node.add_child(mesh_i)
	
	# Для физики используем CollisionShape2D вместо CollisionPolygon2D
	# так как мы будем генерировать форму программно
	var coll = CollisionShape2D.new()
	coll.name = "Collision"
	node.add_child(coll)
	
	return node
