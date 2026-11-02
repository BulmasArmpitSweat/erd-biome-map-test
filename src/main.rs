pub(crate) use bevy::prelude::*;

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Shader::default()
    ));
}

#[derive(Component)]
struct AppMain;

#[derive(Component, ShaderType, Default)]
#[extract_app(RenderApp)]
struct Shader {
    _unused: LinearRgba
}

impl FullscreenMaterial for Shader {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "src/shader.wesl".into()
    }

    fn schedule() -> impl bevy::ecs::schedule::ScheduleLabel + Clone {
        bevy::core_pipeline::Core2d
    }

    fn schedule_configs(system: bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::BoxedSystem>) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::BoxedSystem> {
        system.in_set(bevy::core_pipeline::Core2dSystems::PostProcess).before(bevy::core_pipeline::tonemapping::tonemapping)
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
        )
        .add_systems(Startup, startup)
        .run();
}
