(def COLOR_BUFFER_BIT 16384)
(def VERTEX_SHADER 35633)
(def FRAGMENT_SHADER 35632)
(def vertex_shader_src "
precision mediump float;
attribute vec2 vertPosition;
attribute vec3 vertColor;
varying vec3 fragColor;
void main()
{
  fragColor = vertColor;
  gl_Position = vec4(vertPosition, 0.0, 1.0);
}
")
(def fragment_shader_src "
precision mediump float;

varying vec3 fragColor;
void main()
{
  gl_FragColor = vec4(fragColor, 1.0);
}
")

(extern global_get_window [])
(extern window_get_document [window])
(extern document_query_selector [document query])
(extern htmlcanvas_get_context [element context])
(extern webgl_create_shader [ctx shader_type] )
(extern webgl_shader_source [ctx shader source] )
(extern webgl_compile_shader [ctx shader] )
(extern webgl_create_shader [ctx shader_type vertex_shader_src] )
(extern webgl_create_program [ctx] )
(extern webgl_attach_shader [ctx program shader] )
(extern webgl_link_program [ctx program] )
(extern webgl_use_program [ctx program] )
(extern webgl_clear_color [ctx r g b a] )
(extern webgl_clear [ctx buffer_bit] )
(extern webgl_get_attrib_location [ctx program attrib_name] )

(defn create_shader [ctx shader_type source]
  (let [shader (webgl_create_shader ctx shader_type)]
       (webgl_shader_source ctx shader source)
       (webgl_compile_shader ctx shader)
       shader))

(defn start_program [ctx]
  (let [vertex_shader (create_shader ctx VERTEX_SHADER vertex_shader_src)
        fragment_shader (create_shader ctx FRAGMENT_SHADER fragment_shader_src)
        program (webgl_create_program ctx)]
        (webgl_attach_shader ctx program vertex_shader)
        (webgl_attach_shader ctx program fragment_shader)
        (webgl_link_program ctx program)
        (webgl_use_program ctx program)
        program))

(pub defn main []
  (let [win (global_get_window)
        doc (window_get_document win)
        canvas (document_query_selector doc "#screen")
        ctx (htmlcanvas_get_context canvas "webgl")]
        (webgl_clear_color ctx 0.75 0.85 0.8 1.0)
        (webgl_clear ctx COLOR_BUFFER_BIT)
        (let [program (start_program ctx)
              position_location (webgl_get_attrib_location ctx program "vertPosition")
              color_location (webgl_get_attrib_location ctx program "vertColor")]
              123)))


;     // create a program and get its attribute and uniforms
;     let program = start_program(ctx);
;     let position_location = webgl::get_attrib_location(ctx, program, "vertPosition");
;     let color_location = webgl::get_attrib_location(ctx, program, "vertColor");
;     webgl::use_program(ctx, NULL);
;
;     // setup data buffer
;     let vertices: Vec<f32> = vec![
;         // X, Y,       R, G, B
;         0.0, 0.5, 1.0, 1.0, 0.0, -0.5, -0.5, 0.7, 0.0, 1.0, 0.5, -0.5, 0.1, 1.0, 0.6,
;     ];
;     let vertices = create_f32array(&vertices.into_bytes());
;     let vertex_buffer = webgl::create_buffer(ctx);
;     webgl::bind_buffer(ctx, webgl::ARRAY_BUFFER, vertex_buffer);
;     webgl::buffer_data(ctx, webgl::ARRAY_BUFFER, vertices, webgl::STATIC_DRAW);
;     webgl::bind_buffer(ctx, webgl::ARRAY_BUFFER, NULL);
;
;     // setup for drawing
;     webgl::use_program(ctx, program);
;
;     // draw
;     webgl::bind_buffer(ctx, webgl::ARRAY_BUFFER, vertex_buffer);
;     webgl::enable_vertex_attrib_array(ctx, position_location);
;     webgl::enable_vertex_attrib_array(ctx, color_location);
;     webgl::vertex_attrib_pointer(
;         ctx,
;         position_location,
;         2.0,
;         webgl::FLOAT,
;         false,
;         5.0 * 4.0,
;         0.0,
;     );
;     webgl::vertex_attrib_pointer(
;         ctx,
;         color_location,
;         3.0,
;         webgl::FLOAT,
;         false,
;         5.0 * 4.0,
;         2.0 * 4.0,
;     );
;     webgl::bind_buffer(ctx, webgl::ARRAY_BUFFER, NULL);
;
;     webgl::draw_arrays(ctx, webgl::TRIANGLES, 0.0, 3.0);
; }
