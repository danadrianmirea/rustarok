use crate::components::char::SpriteBoundingRect;
use crate::effect::StrEffectId;
use crate::systems::render_sys::ONE_SPRITE_PIXEL_SIZE_IN_3D;
use crate::video::{GlNativeTextureId, GlTexture, VIDEO_HEIGHT, VIDEO_WIDTH};
use nalgebra::{Matrix3, Matrix4, Rotation3, Vector2, Vector3, Vector4};
use specs::prelude::*;
use std::collections::HashMap;

pub fn create_2d_pos_rot_matrix(pos: &[i16; 2], rotation_rad: f32) -> Matrix4<f32> {
    let mut matrix = Matrix4::<f32>::identity();
    matrix.prepend_translation_mut(&v3!(pos[0], pos[1], 0));

    let rotation =
        Rotation3::from_axis_angle(&nalgebra::Unit::new_normalize(Vector3::z()), rotation_rad)
            .to_homogeneous();
    matrix * rotation
}

pub fn create_3d_pos_rot_matrix(
    pos: &Vector3<f32>,
    rotation_rad: &(Vector3<f32>, f32),
) -> Matrix4<f32> {
    let mut matrix = Matrix4::<f32>::identity();
    matrix.prepend_translation_mut(&pos);
    let rotation = Rotation3::from_axis_angle(
        &nalgebra::Unit::new_normalize(rotation_rad.0),
        rotation_rad.1,
    )
    .to_homogeneous();
    return matrix * rotation;
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct EffectFrameCacheKey {
    pub effect_id: StrEffectId,
    pub layer_index: usize,
    pub key_index: i32,
}

#[derive(Component)]
pub struct RenderCommandCollectorComponent {
    pub(super) partial_circle_2d_commands: Vec<PartialCircle2dRenderCommand>,
    pub(super) texture_2d_commands: Vec<Texture2dRenderCommand>,
    pub(super) rectangle_3d_commands: Vec<Rectangle3dRenderCommand>,
    pub(super) rectangle_2d_commands: Vec<Rectangle2dRenderCommand>,
    pub(super) point_2d_commands: Vec<Point2dRenderCommand>,
    pub(super) text_2d_commands: Vec<Text2dRenderCommand>,
    pub(super) circle_3d_commands: Vec<Circle3dRenderCommand>,
    pub(super) sprite_3d_commands: Vec<Sprite3dRenderCommand>,
    pub(super) number_3d_commands: Vec<Number3dRenderCommand>,
    pub(super) model_commands: Vec<ModelRenderCommand>,
    pub(super) effect_commands: HashMap<EffectFrameCacheKey, Vec<Vector2<f32>>>,
    pub(super) effect_commands2: Vec<(StrEffectId, i32, Vector2<f32>)>,
    pub(super) view_matrix: Matrix4<f32>,
    pub(super) normal_matrix: Matrix3<f32>,
}

impl<'a> RenderCommandCollectorComponent {
    pub fn new() -> RenderCommandCollectorComponent {
        RenderCommandCollectorComponent {
            partial_circle_2d_commands: Vec::with_capacity(128),
            texture_2d_commands: Vec::with_capacity(128),
            text_2d_commands: Vec::with_capacity(128),
            rectangle_3d_commands: Vec::with_capacity(128),
            rectangle_2d_commands: Vec::with_capacity(128),
            point_2d_commands: Vec::with_capacity(128),
            circle_3d_commands: Vec::with_capacity(128),
            sprite_3d_commands: Vec::with_capacity(128),
            number_3d_commands: Vec::with_capacity(128),
            effect_commands: HashMap::with_capacity(128),
            effect_commands2: Vec::with_capacity(128),
            model_commands: Vec::with_capacity(128),
            view_matrix: Matrix4::identity(),
            normal_matrix: Matrix3::identity(),
        }
    }

    pub fn set_view_matrix(&mut self, view_matrix: &Matrix4<f32>, normal_matrix: &Matrix3<f32>) {
        self.view_matrix = *view_matrix;
        self.normal_matrix = *normal_matrix;
    }

    pub fn clear(&mut self) {
        self.partial_circle_2d_commands.clear();
        self.texture_2d_commands.clear();
        self.text_2d_commands.clear();
        self.rectangle_3d_commands.clear();
        self.rectangle_2d_commands.clear();
        self.point_2d_commands.clear();
        self.circle_3d_commands.clear();
        self.sprite_3d_commands.clear();
        self.number_3d_commands.clear();
        self.effect_commands2.clear();
        self.effect_commands
            .iter_mut()
            .for_each(|(_key, vec)| vec.clear());
        self.model_commands.clear();
    }

    pub fn add_model_command_3d(&'a mut self, model_instance_index: usize, is_transparent: bool) {
        self.model_commands.push(ModelRenderCommand {
            model_instance_index,
            is_transparent,
        });
    }

    pub fn partial_circle_2d(&'a mut self) -> PartialCircl2dBuilder {
        PartialCircl2dBuilder::new(self)
    }

    pub fn circle_3d(&'a mut self) -> Circle3dRenderCommandBuilder {
        Circle3dRenderCommandBuilder::new(self)
    }

    pub fn rectangle_3d(&'a mut self) -> Rectangle3dRenderCommandBuilder {
        Rectangle3dRenderCommandBuilder::new(self)
    }

    pub fn text_2d(&'a mut self) -> Text2dRenderCommandBuilder {
        Text2dRenderCommandBuilder::new(self)
    }

    pub fn number_3d(&'a mut self) -> Number3dRenderCommandBuilder {
        Number3dRenderCommandBuilder::new(self)
    }

    pub fn sprite_3d(&'a mut self) -> Sprite3dRenderCommandBuilder {
        Sprite3dRenderCommandBuilder::new(self)
    }

    pub fn rectangle_2d(&'a mut self) -> Rectangle2dCommandBuilder {
        Rectangle2dCommandBuilder::new(self)
    }

    pub fn point_2d(&'a mut self) -> Point2dCommandBuilder {
        Point2dCommandBuilder::new(self)
    }

    pub fn sprite_2d(&'a mut self) -> Texture2dRenderCommandCommandBuilder {
        Texture2dRenderCommandCommandBuilder::new(self)
    }

    pub fn add_effect_command2(
        &'a mut self,
        pos: &Vector2<f32>,
        effect_id: StrEffectId,
        key_index: i32,
    ) {
        self.effect_commands2.push((effect_id, key_index, *pos));
    }

    pub fn add_effect_command(
        &'a mut self,
        pos: &Vector2<f32>,
        effect_id: StrEffectId,
        key_index: i32,
        layer_index: usize,
    ) {
        let frame_cache_key = EffectFrameCacheKey {
            effect_id,
            layer_index,
            key_index,
        };
        self.effect_commands
            .entry(frame_cache_key.clone())
            .or_insert(Vec::with_capacity(128))
            .push(*pos);
    }

    pub fn get_last_billboard_command(&'a self) -> &'a Sprite3dRenderCommand {
        return &self.sprite_3d_commands[self.sprite_3d_commands.len() - 1];
    }
}

pub struct Common2DProperties {
    pub(super) color: [u8; 4],
    pub(super) scale: [f32; 2],
    pub(super) matrix: Matrix4<f32>,
    pub(super) layer: UiLayer2d,
}

pub struct Rectangle2dRenderCommand {
    pub(super) color: [u8; 4],
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) screen_pos: [i16; 2],
    pub(super) rotation_rad: f32,
    pub(super) layer: UiLayer2d,
}

pub struct Rectangle2dCommandBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    screen_pos: [i16; 2],
    rotation_rad: f32,
    width: u16,
    height: u16,
    layer: UiLayer2d,
}

impl<'a> Rectangle2dCommandBuilder<'a> {
    pub fn new(collector: &mut RenderCommandCollectorComponent) -> Rectangle2dCommandBuilder {
        Rectangle2dCommandBuilder {
            collector,
            color: [255, 255, 255, 255],
            screen_pos: [0, 0],
            width: 0,
            height: 0,
            rotation_rad: 0.0,
            layer: UiLayer2d::HealthBars,
        }
    }

    pub fn add(&mut self) {
        self.collector
            .rectangle_2d_commands
            .push(Rectangle2dRenderCommand {
                color: self.color,
                width: self.width,
                height: self.height,
                screen_pos: self.screen_pos,
                layer: self.layer,
                rotation_rad: self.rotation_rad,
            });
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut Rectangle2dCommandBuilder {
        self.color = *color;
        self
    }

    pub fn rotation_rad(&mut self, rotation_rad: f32) -> &'a mut Rectangle2dCommandBuilder {
        self.rotation_rad = rotation_rad;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut Rectangle2dCommandBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn screen_pos(&mut self, x: i32, y: i32) -> &'a mut Rectangle2dCommandBuilder {
        self.screen_pos = [x as i16, y as i16];
        self
    }

    pub fn width(&mut self, w: u16) -> &'a mut Rectangle2dCommandBuilder {
        self.width = w;
        self
    }

    pub fn layer(&mut self, layer: UiLayer2d) -> &'a mut Rectangle2dCommandBuilder {
        self.layer = layer;
        self
    }

    pub fn size(&mut self, w: u16, h: u16) -> &'a mut Rectangle2dCommandBuilder {
        self.width = w;
        self.height = h;
        self
    }

    pub fn height(&mut self, h: u16) -> &'a mut Rectangle2dCommandBuilder {
        self.height = h;
        self
    }
}

pub struct Point2dRenderCommand {
    pub(super) color: [u8; 4],
    pub(super) screen_pos: [i16; 2],
    pub(super) layer: UiLayer2d,
}

pub struct Point2dCommandBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    screen_pos: [i16; 2],
    layer: UiLayer2d,
}

impl<'a> Point2dCommandBuilder<'a> {
    pub fn new(collector: &mut RenderCommandCollectorComponent) -> Point2dCommandBuilder {
        Point2dCommandBuilder {
            collector,
            color: [255, 255, 255, 255],
            screen_pos: [0, 0],
            layer: UiLayer2d::HealthBars,
        }
    }

    pub fn add(&mut self) {
        self.collector.point_2d_commands.push(Point2dRenderCommand {
            color: self.color,
            screen_pos: self.screen_pos,
            layer: self.layer,
        });
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut Point2dCommandBuilder {
        self.color = *color;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut Point2dCommandBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn screen_pos(&mut self, x: i32, y: i32) -> &'a mut Point2dCommandBuilder {
        self.screen_pos = [x as i16, y as i16];
        self
    }

    pub fn layer(&mut self, layer: UiLayer2d) -> &'a mut Point2dCommandBuilder {
        self.layer = layer;
        self
    }
}

#[derive(Debug)]
pub struct PartialCircle2dRenderCommand {
    pub(super) circumference_index: usize,
    pub(super) color: [u8; 4],
    pub(super) screen_pos: [i16; 2],
    pub(super) layer: UiLayer2d,
}

pub struct PartialCircl2dBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    screen_pos: [i16; 2],
    circumference_percentage: usize,
    layer: UiLayer2d,
}

impl<'a> PartialCircl2dBuilder<'a> {
    pub fn new(collector: &mut RenderCommandCollectorComponent) -> PartialCircl2dBuilder {
        PartialCircl2dBuilder {
            collector,
            color: [255, 255, 255, 255],
            screen_pos: [0, 0],
            layer: UiLayer2d::HealthBars,
            circumference_percentage: 100,
        }
    }

    pub fn add(&mut self) {
        self.collector
            .partial_circle_2d_commands
            .push(PartialCircle2dRenderCommand {
                circumference_index: self.circumference_percentage - 1,
                color: self.color,
                screen_pos: self.screen_pos,
                layer: self.layer,
            });
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut PartialCircl2dBuilder {
        self.color = *color;
        self
    }

    pub fn circumference_percentage(
        &mut self,
        circumference_percentage: usize,
    ) -> &'a mut PartialCircl2dBuilder {
        self.circumference_percentage = circumference_percentage;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut PartialCircl2dBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn screen_pos(&mut self, x: i32, y: i32) -> &'a mut PartialCircl2dBuilder {
        self.screen_pos = [x as i16, y as i16];
        self
    }

    pub fn layer(&mut self, layer: UiLayer2d) -> &'a mut PartialCircl2dBuilder {
        self.layer = layer;
        self
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UiLayer2d {
    HealthBars,
    StatusIndicators,
    SelfCastingBar,
    SkillBar,
    SkillBarIcon,
    SkillBarKey,
    Minimap,
    MinimapSimpleEntities,
    MinimapImportantEntities,
    SelectingTargetSkillName,
    Console,
    ConsoleTexts,
    ConsoleAutocompletion,
    Cursor,
}

#[derive(Debug)]
pub struct Texture2dRenderCommand {
    pub(super) color: [u8; 4],
    pub(super) offset: [i16; 2],
    pub(super) screen_pos: [i16; 2],
    pub(super) rotation_rad: f32,
    pub(super) scale: f32,
    pub(super) texture: GlNativeTextureId,
    pub(super) texture_width: i32,
    pub(super) texture_height: i32,
    pub(super) layer: UiLayer2d,
}

pub struct Texture2dRenderCommandCommandBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    screen_pos: [i16; 2],
    offset: [i16; 2],
    rotation_rad: f32,
    layer: UiLayer2d,
    scale: f32,
    flip_vertically: bool,
}

impl<'a> Texture2dRenderCommandCommandBuilder<'a> {
    pub fn new(
        collector: &mut RenderCommandCollectorComponent,
    ) -> Texture2dRenderCommandCommandBuilder {
        Texture2dRenderCommandCommandBuilder {
            collector,
            color: [255, 255, 255, 255],
            screen_pos: [0, 0],
            offset: [0, 0],
            rotation_rad: 0.0,
            scale: 1.0,
            layer: UiLayer2d::HealthBars,
            flip_vertically: false,
        }
    }

    pub fn add(&mut self, texture: &GlTexture) {
        self.collector
            .texture_2d_commands
            .push(Texture2dRenderCommand {
                color: self.color,
                scale: self.scale,
                texture: texture.id(),
                offset: self.offset,
                texture_width: (1 - self.flip_vertically as i32 * 2) * texture.width,
                texture_height: texture.height,
                screen_pos: self.screen_pos,
                rotation_rad: self.rotation_rad,
                layer: self.layer,
            });
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut Texture2dRenderCommandCommandBuilder {
        self.color = *color;
        self
    }

    pub fn flip_vertically(&mut self, flip: bool) -> &'a mut Texture2dRenderCommandCommandBuilder {
        self.flip_vertically = flip;
        self
    }

    pub fn rotation_rad(
        &mut self,
        rotation_rad: f32,
    ) -> &'a mut Texture2dRenderCommandCommandBuilder {
        self.rotation_rad = rotation_rad;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut Texture2dRenderCommandCommandBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn screen_pos(&mut self, x: i32, y: i32) -> &'a mut Texture2dRenderCommandCommandBuilder {
        self.screen_pos = [x as i16, y as i16];
        self
    }

    pub fn offset(&mut self, x: i16, y: i16) -> &'a mut Texture2dRenderCommandCommandBuilder {
        self.offset = [x, y];
        self
    }

    pub fn layer(&mut self, layer: UiLayer2d) -> &'a mut Texture2dRenderCommandCommandBuilder {
        self.layer = layer;
        self
    }

    pub fn scale(&mut self, scale: f32) -> &'a mut Texture2dRenderCommandCommandBuilder {
        self.scale = scale;
        self
    }
}

#[derive(Debug)]
pub struct Text2dRenderCommand {
    pub(super) text: String,
    pub(super) color: [u8; 4],
    pub(super) screen_pos: [i16; 2],
    pub(super) font: Font,
    pub(super) outline: bool,
    pub(super) layer: UiLayer2d,
}

pub struct Text2dRenderCommandBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    screen_pos: [i16; 2],
    layer: UiLayer2d,
    font: Font,
    outline: bool,
}

impl<'a> Text2dRenderCommandBuilder<'a> {
    pub fn new(collector: &mut RenderCommandCollectorComponent) -> Text2dRenderCommandBuilder {
        Text2dRenderCommandBuilder {
            collector,
            color: [255, 255, 255, 255],
            font: Font::Normal,
            screen_pos: [0, 0],
            outline: false,
            layer: UiLayer2d::HealthBars,
        }
    }

    pub fn add(&mut self, text: &str) {
        self.collector.text_2d_commands.push(Text2dRenderCommand {
            text: text.to_owned(),
            color: self.color,
            screen_pos: self.screen_pos,
            font: self.font,
            outline: self.outline,
            layer: self.layer,
        })
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut Text2dRenderCommandBuilder {
        self.color = *color;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut Text2dRenderCommandBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn screen_pos(&mut self, x: i32, y: i32) -> &'a mut Text2dRenderCommandBuilder {
        self.screen_pos = [x as i16, y as i16];
        self
    }

    pub fn layer(&mut self, layer: UiLayer2d) -> &'a mut Text2dRenderCommandBuilder {
        self.layer = layer;
        self
    }

    pub fn font(&mut self, font: Font) -> &'a mut Text2dRenderCommandBuilder {
        self.font = font;
        self
    }

    pub fn outline(&mut self, outline: bool) -> &'a mut Text2dRenderCommandBuilder {
        self.outline = outline;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Font {
    Small,
    SmallBold,
    Normal,
    NormalBold,
    Big,
    BigBold,
}

pub struct Rectangle3dRenderCommand {
    pub(super) color: [u8; 4],
    pub(super) pos: Vector3<f32>,
    pub(super) rotation_rad: f32,
    pub(super) width: u16,
    pub(super) height: u16,
}

pub struct Rectangle3dRenderCommandBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    pos: Vector3<f32>,
    rotation_rad: f32,
    width: u16,
    height: u16,
}

impl<'a> Rectangle3dRenderCommandBuilder<'a> {
    fn new(collector: &mut RenderCommandCollectorComponent) -> Rectangle3dRenderCommandBuilder {
        Rectangle3dRenderCommandBuilder {
            collector,
            color: [255, 255, 255, 255],
            pos: Vector3::zeros(),
            rotation_rad: 0.0,
            width: 0,
            height: 0,
        }
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut Rectangle3dRenderCommandBuilder {
        self.color = *color;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut Rectangle3dRenderCommandBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn alpha(&mut self, a: u8) -> &'a mut Rectangle3dRenderCommandBuilder {
        self.color[3] = a;
        self
    }

    pub fn pos_2d(&mut self, pos: &Vector2<f32>) -> &'a mut Rectangle3dRenderCommandBuilder {
        self.pos.x = pos.x;
        self.pos.z = pos.y;
        self
    }

    pub fn pos(&mut self, pos: &Vector3<f32>) -> &'a mut Rectangle3dRenderCommandBuilder {
        self.pos = *pos;
        self
    }

    pub fn y(&mut self, y: f32) -> &'a mut Rectangle3dRenderCommandBuilder {
        self.pos.y = y;
        self
    }

    pub fn size(&mut self, w: u16, h: u16) -> &'a mut Rectangle3dRenderCommandBuilder {
        self.width = w;
        self.height = h;
        self
    }

    pub fn rotation_rad(
        &'a mut self,
        rotation_rad: f32,
    ) -> &'a mut Rectangle3dRenderCommandBuilder {
        self.rotation_rad = rotation_rad;
        self
    }

    pub fn add(&'a mut self) {
        self.collector
            .rectangle_3d_commands
            .push(Rectangle3dRenderCommand {
                color: self.color,
                pos: self.pos,
                width: self.width,
                height: self.height,
                rotation_rad: self.rotation_rad,
            });
    }
}

pub struct Circle3dRenderCommand {
    pub(super) color: [u8; 4],
    pub(super) pos: Vector3<f32>,
    pub(super) radius: f32,
}

pub struct Circle3dRenderCommandBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    pos: Vector3<f32>,
    radius: f32,
}

impl<'a> Circle3dRenderCommandBuilder<'a> {
    fn new(collector: &mut RenderCommandCollectorComponent) -> Circle3dRenderCommandBuilder {
        Circle3dRenderCommandBuilder {
            collector,
            color: [255, 255, 255, 255],
            pos: Vector3::zeros(),
            radius: 1.0,
        }
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut Circle3dRenderCommandBuilder {
        self.color = *color;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut Circle3dRenderCommandBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn alpha(&mut self, a: u8) -> &'a mut Circle3dRenderCommandBuilder {
        self.color[3] = a;
        self
    }

    pub fn pos_2d(&mut self, pos: &Vector2<f32>) -> &'a mut Circle3dRenderCommandBuilder {
        self.pos.x = pos.x;
        self.pos.z = pos.y;
        self
    }

    pub fn pos(&mut self, pos: &Vector3<f32>) -> &'a mut Circle3dRenderCommandBuilder {
        self.pos = *pos;
        self
    }

    pub fn y(&mut self, y: f32) -> &'a mut Circle3dRenderCommandBuilder {
        self.pos.y = y;
        self
    }

    pub fn radius(&'a mut self, radius: f32) -> &'a mut Circle3dRenderCommandBuilder {
        self.radius = radius;
        self
    }

    pub fn add(&'a mut self) {
        self.collector
            .circle_3d_commands
            .push(Circle3dRenderCommand {
                color: self.color,
                pos: self.pos,
                radius: self.radius,
            });
    }
}

#[derive(Debug)]
pub struct Sprite3dRenderCommand {
    pub color: [u8; 4],
    pub scale: f32,
    pub pos: Vector3<f32>,
    pub texture: GlNativeTextureId,
    pub offset: [i16; 2],
    pub texture_width: u16,
    pub texture_height: u16,
    pub is_vertically_flipped: bool,
}

impl Sprite3dRenderCommand {
    pub fn project_to_screen(
        &self,
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
    ) -> SpriteBoundingRect {
        let width = self.texture_width as f32 * ONE_SPRITE_PIXEL_SIZE_IN_3D * self.scale;
        let height = self.texture_height as f32 * ONE_SPRITE_PIXEL_SIZE_IN_3D * self.scale;
        let offset_in_3d_space = [
            self.offset[0] as f32 * ONE_SPRITE_PIXEL_SIZE_IN_3D * self.scale,
            self.offset[1] as f32 * ONE_SPRITE_PIXEL_SIZE_IN_3D * self.scale,
        ];
        let mut top_right = Vector4::new(0.5 * width, 0.5 * height, 0.0, 1.0);
        top_right.x += offset_in_3d_space[0] as f32;
        top_right.y -= offset_in_3d_space[1] as f32;

        let mut bottom_left = Vector4::new(-0.5 * width, -0.5 * height, 0.0, 1.0);
        bottom_left.x += offset_in_3d_space[0] as f32;
        bottom_left.y -= offset_in_3d_space[1] as f32;

        let mut model_view = view * Matrix4::new_translation(&self.pos);
        Sprite3dRenderCommand::set_spherical_billboard(&mut model_view);
        fn sh(v: Vector4<f32>) -> [i32; 2] {
            let s = if v[3] == 0.0 { 1.0 } else { 1.0 / v[3] };
            [
                ((v[0] * s / 2.0 + 0.5) * VIDEO_WIDTH as f32) as i32,
                VIDEO_HEIGHT as i32 - ((v[1] * s / 2.0 + 0.5) * VIDEO_HEIGHT as f32) as i32,
            ]
        }
        let bottom_left = sh(projection * model_view * bottom_left);
        let top_right = sh(projection * model_view * top_right);
        return SpriteBoundingRect {
            bottom_left,
            top_right,
        };
    }

    fn set_spherical_billboard(model_view: &mut Matrix4<f32>) {
        model_view[0] = 1.0;
        model_view[1] = 0.0;
        model_view[2] = 0.0;
        model_view[4] = 0.0;
        model_view[5] = 1.0;
        model_view[6] = 0.0;
        model_view[8] = 0.0;
        model_view[9] = 0.0;
        model_view[10] = 1.0;
    }
}

pub struct Sprite3dRenderCommandBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    pos: Vector3<f32>,
    offset: [i16; 2],
    scale: f32,
    flip_vertically: bool,
}

impl<'a> Sprite3dRenderCommandBuilder<'a> {
    fn new(collector: &mut RenderCommandCollectorComponent) -> Sprite3dRenderCommandBuilder {
        Sprite3dRenderCommandBuilder {
            collector,
            color: [255, 255, 255, 255],
            pos: Vector3::zeros(),
            offset: [0, 0],
            scale: 1.0,
            flip_vertically: false,
        }
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut Sprite3dRenderCommandBuilder {
        self.color = *color;
        self
    }

    pub fn flip_vertically(&mut self, flip: bool) -> &'a mut Sprite3dRenderCommandBuilder {
        self.flip_vertically = flip;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut Sprite3dRenderCommandBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn alpha(&mut self, a: u8) -> &'a mut Sprite3dRenderCommandBuilder {
        self.color[3] = a;
        self
    }

    pub fn offset(&mut self, offset: [i16; 2]) -> &'a mut Sprite3dRenderCommandBuilder {
        self.offset = offset;
        self
    }

    pub fn pos_2d(&mut self, pos: &Vector2<f32>) -> &'a mut Sprite3dRenderCommandBuilder {
        self.pos.x = pos.x;
        self.pos.z = pos.y;
        self
    }

    pub fn pos(&mut self, pos: &Vector3<f32>) -> &'a mut Sprite3dRenderCommandBuilder {
        self.pos = *pos;
        self
    }

    pub fn y(&mut self, y: f32) -> &'a mut Sprite3dRenderCommandBuilder {
        self.pos.y = y;
        self
    }

    pub fn scale(&'a mut self, scale: f32) -> &'a mut Sprite3dRenderCommandBuilder {
        self.scale = scale;
        self
    }

    pub fn add(&'a mut self, texture: &GlTexture) {
        let command = Sprite3dRenderCommand {
            color: self.color,
            scale: self.scale,
            offset: self.offset,
            texture: texture.id(),
            is_vertically_flipped: self.flip_vertically,
            texture_width: texture.width as u16,
            texture_height: texture.height as u16,
            pos: self.pos,
        };
        self.collector.sprite_3d_commands.push(command);
    }
}

#[derive(Debug)]
pub struct Number3dRenderCommand {
    pub(super) color: [u8; 4],
    pub(super) scale: f32,
    pub(super) pos: Vector3<f32>,
    pub(super) value: u32,
    pub(super) digit_count: u8,
}

pub struct Number3dRenderCommandBuilder<'a> {
    collector: &'a mut RenderCommandCollectorComponent,
    color: [u8; 4],
    pos: Vector3<f32>,
    offset: [i16; 2],
    scale: f32,
}

impl<'a> Number3dRenderCommandBuilder<'a> {
    fn new(collector: &mut RenderCommandCollectorComponent) -> Number3dRenderCommandBuilder {
        Number3dRenderCommandBuilder {
            collector,
            color: [255, 255, 255, 255],
            pos: Vector3::zeros(),
            offset: [0, 0],
            scale: 1.0,
        }
    }

    pub fn color(&mut self, color: &[u8; 4]) -> &'a mut Number3dRenderCommandBuilder {
        self.color = *color;
        self
    }

    pub fn color_rgb(&mut self, color: &[u8; 3]) -> &'a mut Number3dRenderCommandBuilder {
        self.color[0] = color[0];
        self.color[1] = color[1];
        self.color[2] = color[2];
        self
    }

    pub fn alpha(&mut self, a: u8) -> &'a mut Number3dRenderCommandBuilder {
        self.color[3] = a;
        self
    }

    pub fn offset(&mut self, offset: [i16; 2]) -> &'a mut Number3dRenderCommandBuilder {
        self.offset = offset;
        self
    }

    pub fn pos_2d(&mut self, pos: &Vector2<f32>) -> &'a mut Number3dRenderCommandBuilder {
        self.pos.x = pos.x;
        self.pos.z = pos.y;
        self
    }

    pub fn pos(&mut self, pos: &Vector3<f32>) -> &'a mut Number3dRenderCommandBuilder {
        self.pos = *pos;
        self
    }

    pub fn y(&mut self, y: f32) -> &'a mut Number3dRenderCommandBuilder {
        self.pos.y = y;
        self
    }

    pub fn scale(&'a mut self, scale: f32) -> &'a mut Number3dRenderCommandBuilder {
        self.scale = scale;
        self
    }

    pub fn add(&'a mut self, value: u32, digit_count: u8) {
        self.collector
            .number_3d_commands
            .push(Number3dRenderCommand {
                color: self.color,
                scale: self.scale,
                pos: self.pos,
                value,
                digit_count,
            });
    }
}

#[derive(Debug)]
pub struct ModelRenderCommand {
    pub(super) is_transparent: bool,
    pub(super) model_instance_index: usize,
}
