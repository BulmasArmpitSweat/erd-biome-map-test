#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let colour = vec3<f32>(
        uv.x,
        uv.y,
        0.5
    );

    return vec4<f32>(colour, 1.0);
}