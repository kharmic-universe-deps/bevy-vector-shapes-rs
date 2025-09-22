use bevy::{
    prelude::*,
    reflect::Reflect,
    render::render_resource::{ShaderRef, ShaderType},
};
use wgpu::vertex_attr_array;

use crate::render::NGON_METER_HANDLE;
use crate::{
    prelude::*,
    render::{Flags, ShapeComponent, ShapeData, NGON_HANDLE},
};

/// Component containing the data for drawing a regular polygon.
#[derive(Component, Reflect)]
pub struct PolygonMeterComponent {
    pub color: Color,
    pub thickness: f32,
    pub thickness_type: ThicknessType,
    pub alignment: Alignment,
    pub hollow: bool,

    /// Number of sides, non-integer values may have unexpected results.
    pub sides: f32,
    /// Radius to the tip of a corner.
    pub radius: f32,
    /// Corner rounding radius for all corner in world units.
    pub roundness: f32,

    pub start_angle: f32,
    pub end_angle: f32,
    pub rotation: f32,

    pub percent: f32,
}

impl PolygonMeterComponent {
    pub fn new(
        config: &ShapeConfig,
        sides: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        rotation: f32,
        percent: f32,
    ) -> Self {
        Self {
            color: config.color,
            thickness: config.thickness,
            thickness_type: config.thickness_type,
            alignment: config.alignment,
            hollow: config.hollow,

            sides,
            radius,
            roundness: config.roundness,
            start_angle,
            end_angle,
            rotation,
            percent,
        }
    }
}

impl ShapeComponent for PolygonMeterComponent {
    type Data = NgonMeterData;

    fn get_data(&self, tf: &GlobalTransform, fill: &ShapeFill) -> NgonMeterData {
        let mut flags = Flags(0);
        let thickness = match fill.ty {
            FillType::Stroke(thickness, thickness_type) => {
                flags.set_thickness_type(thickness_type);
                flags.set_hollow(1);
                thickness
            }
            FillType::Fill => 1.0,
        };
        flags.set_alignment(self.alignment);

        NgonMeterData {
            transform: tf.compute_matrix().to_cols_array_2d(),

            color: fill.color.to_linear().to_f32_array(),
            thickness,
            flags: flags.0,

            sides: self.sides,
            radius: self.radius,
            roundness: self.roundness,

            start_angle: self.start_angle,
            end_angle: self.end_angle,
            rotation: self.rotation,
            percent: self.percent,
            padding: default(),
        }
    }
}

impl Default for PolygonMeterComponent {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            thickness: 1.0,
            thickness_type: default(),
            alignment: default(),
            hollow: false,

            sides: 3.0,
            radius: 1.0,
            roundness: 0.0,

            start_angle: 0.0,
            end_angle: 0.0,
            rotation: 0.0,

            percent: 0.0,
        }
    }
}

/// Raw data sent to the regular polygon shader to draw a regular polygon
#[derive(Clone, Copy, Reflect, Default, Debug, ShaderType)]
#[repr(C)]
pub struct NgonMeterData {
    transform: [[f32; 4]; 4],

    color: [f32; 4],
    thickness: f32,
    flags: u32,

    sides: f32,
    radius: f32,
    roundness: f32,

    start_angle: f32,
    end_angle: f32,
    rotation: f32,

    percent: f32,

    padding: [f32; 3],
}

impl NgonMeterData {
    pub fn new(
        config: &ShapeConfig,
        sides: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        rotation: f32,
        percent: f32,
    ) -> NgonMeterData {
        let mut flags = Flags(0);
        flags.set_thickness_type(config.thickness_type);
        flags.set_alignment(config.alignment);
        flags.set_hollow(config.hollow as u32);

        NgonMeterData {
            transform: config.transform.compute_matrix().to_cols_array_2d(),

            color: config.color.to_linear().to_f32_array(),
            thickness: config.thickness,
            flags: flags.0,

            sides,
            radius,
            roundness: config.roundness,

            start_angle,
            end_angle,
            rotation,
            percent,

            padding: default(),
        }
    }
}

impl ShapeData for NgonMeterData {
    type Component = PolygonMeterComponent;

    fn vertex_layout() -> Vec<wgpu::VertexAttribute> {
        vertex_attr_array![
            0 => Float32x4,
            1 => Float32x4,
            2 => Float32x4,
            3 => Float32x4,

            4 => Float32x4,
            5 => Float32,
            6 => Uint32,

            7 => Float32,
            8 => Float32,
            9 => Float32,

            10 => Float32,
            11 => Float32,
            12 => Float32,

            13 => Float32,
        ]
        .to_vec()
    }

    fn shader() -> ShaderRef {
        NGON_METER_HANDLE.into()
    }

    fn transform(&self) -> Mat4 {
        Mat4::from_cols_array_2d(&self.transform)
    }
}

/// Extension trait for [`ShapePainter`] to enable it to draw regular polygons.
pub trait PolygonMeterPainter {
    fn ngon_meter(
        &mut self,
        sides: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        rotation: f32,
        percent: f32,
    ) -> &mut Self;
}

impl<'w, 's> PolygonMeterPainter for ShapePainter<'w, 's> {
    fn ngon_meter(
        &mut self,
        sides: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        rotation: f32,
        percent: f32,
    ) -> &mut Self {
        self.send(NgonMeterData::new(
            self.config(),
            sides,
            radius,
            start_angle,
            end_angle,
            rotation,
            percent,
        ))
    }
}

/// Extension trait for [`ShapeBundle`] to enable creation of regular polygon bundles.
pub trait PolygonMeterBundle {
    fn ngon_meter(
        config: &ShapeConfig,
        sides: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        rotation: f32,
        percent: f32,
    ) -> Self;
}

impl PolygonMeterBundle for ShapeBundle<PolygonMeterComponent> {
    fn ngon_meter(
        config: &ShapeConfig,
        sides: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        rotation: f32,
        percent: f32,
    ) -> Self {
        Self::new(
            config,
            PolygonMeterComponent::new(
                config,
                sides,
                radius,
                start_angle,
                end_angle,
                rotation,
                percent,
            ),
        )
    }
}

/// Extension trait for [`ShapeSpawner`] to enable spawning of regular polygon entities.
pub trait PolygonMeterSpawner<'w> {
    fn ngon_meter(
        &mut self,
        sides: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        rotation: f32,
        percent: f32,
    ) -> ShapeEntityCommands<'_, '_>;
}

impl<'w, T: ShapeSpawner<'w>> PolygonMeterSpawner<'w> for T {
    fn ngon_meter(
        &mut self,
        sides: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        rotation: f32,
        percent: f32,
    ) -> ShapeEntityCommands<'_, '_> {
        self.spawn_shape(ShapeBundle::ngon_meter(
            self.config(),
            sides,
            radius,
            start_angle,
            end_angle,
            rotation,
            percent,
        ))
    }
}
