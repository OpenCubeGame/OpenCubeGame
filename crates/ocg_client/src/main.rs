use bevy::a11y::AccessibilityPlugin;
use bevy::audio::AudioPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::gltf::GltfPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::pbr::PbrPlugin;
use bevy::prelude::*;
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::text::TextPlugin;
use bevy::time::TimePlugin;
use bevy::ui::UiPlugin;
use bevy::window::{ExitCondition, PresentMode};
use bevy::winit::WinitPlugin;

pub mod cameramove;

fn main() {
    // Unset the manifest dir to make bevy load assets from the workspace root
    std::env::set_var("CARGO_MANIFEST_DIR", "");

    let mut app = App::new();
    // Bevy Base
    app.add_plugins(LogPlugin::default())
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(TypeRegistrationPlugin)
        .add_plugins(FrameCountPlugin)
        .add_plugins(TimePlugin)
        .add_plugins(TransformPlugin)
        .add_plugins(HierarchyPlugin)
        .add_plugins(DiagnosticsPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
                title: "OpenCubeGame".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        })
        .add_plugins(AccessibilityPlugin)
        .add_plugins(AssetPlugin::default())
        .add_plugins(ScenePlugin)
        .add_plugins(WinitPlugin)
        .add_plugins(RenderPlugin::default())
        .add_plugins(ImagePlugin::default())
        .add_plugins(PipelinedRenderingPlugin)
        .add_plugins(CorePipelinePlugin)
        .add_plugins(SpritePlugin)
        .add_plugins(TextPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PbrPlugin::default())
        .add_plugins(AudioPlugin::default())
        .add_plugins(GilrsPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(GltfPlugin::default())
        .add_plugins(cameramove::PlayerPlugin);

    app.add_plugins(debug_window::DebugWindow);

    app.run();

}

mod debug_window {
    use std::collections::HashMap;

    use bevy::log;
    use bevy::prelude::*;
    use bevy::reflect::erased_serde::__private::serde::__private::de;
    use ocg_schemas::coordinates;
    use ocg_schemas::voxel::chunk_storage::PaletteStorage;
    
    pub struct DebugWindow;

    impl Plugin for DebugWindow {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, debug_window_setup);
        }
    }

    fn debug_window_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        log::warn!("Setting up debug window");
        let font: Handle<Font> = asset_server.load("fonts/cascadiacode.ttf");

        let debug_material = materials.add(StandardMaterial {
            base_color: Color::FUCHSIA,
            ..default()
        });

        let stone_material = &materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        });
        let grass_material = &materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        });
        let dirt_material = &materials.add(StandardMaterial {
            base_color: Color::BISQUE,
            ..default()
        });
        let snow_material = &materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        });

        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Torus::default().into()),
            material: debug_material,
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        });

        let chunks: HashMap<coordinates::AbsBlockPos, PaletteStorage<u64>> = ocg_common::world::generator::generate(4, 4, 4);

        for chunk in chunks.iter() {
            for block in chunk.1.iter_with_coords() {
                if block.1 > &0 { // not air
                    let pos = chunk.0;
                    let mat = match block.1 {
                        2 => grass_material,
                        3 => dirt_material,
                        4 => snow_material,
                        _ => stone_material,
                    };
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
                        material: mat.clone(),
                        transform: Transform::from_xyz((block.0.x + (pos.x * coordinates::CHUNK_DIM)) as f32, (block.0.y + (pos.y * coordinates::CHUNK_DIM)) as f32, (block.0.z + (pos.z * coordinates::CHUNK_DIM)) as f32),
                        ..default()
                    });
                }
            }
        }

        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 100000.0,
                range: 10000.,
                shadows_enabled: false, // enable after optimizing idk
                ..default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 128.0),
            ..default()
        });

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(25.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::CRIMSON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Hello Bevy",
                    TextStyle {
                        font: font.clone(),
                        font_size: 75.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
                log::warn!("Child made");
            });
        log::warn!("Setting up debug window done");
    }
}
