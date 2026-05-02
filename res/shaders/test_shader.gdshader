shader_type canvas_item;

void fragment() {
    // Получаем ID материала (0.0 - 1.0) и переводим в 0-255
    int mat_id = int(COLOR.r * 255.0 + 0.5); 
    
    vec4 final_color = vec4(1.0);
	
    if (mat_id == 0) final_color = vec4(0.0); // Vacuum
    else if (mat_id == 1) final_color = vec4(0.4, 0.4, 0.4, 1.0); // Stone
    else if (mat_id == 2) final_color = vec4(0.5, 0.3, 0.2, 1.0); // Dirt
    else if (mat_id == 3) final_color = vec4(0.2, 0.7, 0.2, 1.0); // Grass
    else if (mat_id == 4) final_color = vec4(0.9, 0.8, 0.4, 1.0); // Sand
    else if (mat_id == 5) final_color = vec4(0.9, 0.9, 1.0, 1.0); // Snow
    else if (mat_id == 6) final_color = vec4(1.0, 0.1, 0.1, 1.0); // Core
    // ... добавь остальные ID по списку
    
    COLOR = final_color;
}

//shader_type canvas_item;
//
//void fragment() {
    //// Получаем ID материала (0.0 - 1.0) и переводим в 0-255
    //// Добавляем 0.5 перед приведением к int, чтобы избежать ошибок округления
    //int mat_id = int(COLOR.r * 255.0 + 0.5); 
    //
    //vec4 final_color = vec4(1.0, 0.0, 1.0, 1.0); // Ядовито-розовый для неизвестных ID (ошибка)
//
    //if (mat_id == 0) {
        //final_color = vec4(0.0, 0.0, 0.0, 0.0); // Vacuum (прозрачный)
    //} else if (mat_id >= 1 && mat_id <= 5) {
        //// Слои атмосферы: от темно-синего к светлому
        //float t = float(mat_id) / 5.0;
        //final_color = mix(vec4(0.1, 0.2, 0.5, 0.2), vec4(0.4, 0.6, 1.0, 0.1), t);
    //} else if (mat_id == 6) {
        //final_color = vec4(0.2, 0.8, 0.2, 1.0); // Grass (ярко-зеленый)
    //} else if (mat_id == 7) {
        //final_color = vec4(0.5, 0.3, 0.2, 1.0); // Dirt (коричневый)
    //} else if (mat_id == 8) {
        //final_color = vec4(0.4, 0.4, 0.4, 1.0); // Stone (серый)
    //} else if (mat_id == 9) {
        //final_color = vec4(0.2, 0.2, 0.2, 1.0); // Hard Stone (темно-серый)
    //} else if (mat_id == 10) {
        //final_color = vec4(0.4, 0.1, 0.1, 1.0); // Mantle (темно-бордовый)
    //} else if (mat_id == 11) {
        //final_color = vec4(0.9, 0.2, 0.0, 1.0); // Lava (оранжево-красный)
    //} else if (mat_id == 12) {
        //final_color = vec4(1.0, 0.8, 0.2, 1.0); // Core (ярко-желтый)
    //}
    //
    //// Специальные биомы (из pick_biome)
    //else if (mat_id == 1) final_color = vec4(0.1, 0.5, 0.1, 1.0); // Jungle (темно-зеленый)
    //else if (mat_id == 2) final_color = vec4(0.9, 0.8, 0.4, 1.0); // Desert (песочный)
    //else if (mat_id == 3) final_color = vec4(0.6, 0.6, 0.7, 1.0); // Mountains (скалистый)
    //else if (mat_id == 4) final_color = vec4(0.9, 0.9, 1.0, 1.0); // Snow (белый)
    //
    //COLOR = final_color;
//}
