
use crossbeam::channel::*;
use crate::kernel::*;

use makepad_render::*;
use makepad_widget::*;


#[derive(Clone)]
pub struct Display {
}
impl Display {
	pub fn new() -> Box<dyn Serviceable> {
		Box::new(Self {})
	}
}
impl Serviceable for Display {
    fn name(&self) -> &str { "Display" }
	fn stop(&self) {}
	fn start(&self, _name: String, sid: SID, send: Sender<Message>, recv: Receiver<Message> ) {

        // listen to display messages
		send.send(
		    Message::Subscribe(sid,"/display".to_string())
		).expect("Display: failed to subscribe");

        // open a display
        let mut cx = Cx::default();
        cx.style();
        ThreeDExampleApp::style(&mut cx);
        cx.init_live_styles();
        let mut app = ThreeDExampleApp::new(&mut cx,send,recv);
        let mut cxafterdraw = CxAfterDraw::new(&mut cx);
        cx.event_loop( | cx, mut event | {
            if let Event::Draw = event {
                app.draw_app(cx);
                cxafterdraw.after_draw(cx);
                return
            }
            app.handle_app(cx, &mut event);
        });


	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// statically declared 2d layout
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ThreeDExampleApp {
    desktop_window: DesktopWindow, 
    menu: Menu,
    world_view: WorldView,
    textinput:TextInput,
    button:NormalButton,
    send:Sender<Message>,
    recv:Receiver<Message>,
}

impl ThreeDExampleApp {
    pub fn new(cx: &mut Cx, send: Sender<Message>, recv: Receiver<Message>) -> Self {
        Self {
            desktop_window: DesktopWindow::new(cx).with_inner_layout(Layout{
                line_wrap: LineWrap::NewLine,
                ..Layout::default()
            }),

            menu:Menu::main(vec![
                Menu::sub("Example", vec![
                    Menu::line(),
                    Menu::item("Quit Example",  Cx::command_quit()),
                ]),
            ]),

            world_view: WorldView::new(cx),

            textinput: TextInput::new(cx,TextInputOptions { multiline:false, read_only: false, empty_message: "Enter URL here".to_string() }),

            button: NormalButton::new(cx),

            send:send,
            recv:recv,
        }
    }
    
    pub fn style(cx: &mut Cx){
        set_widget_style(cx);
        WorldView::style(cx);
        SkyBox::style(cx);
    }
       
    pub fn handle_app(&mut self, cx: &mut Cx, event: &mut Event) {

        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // draw primitives
        while let Ok(message) = self.recv.try_recv() {
            match message {
                Message::Event(topic,data) => {
                    println!("Display: Received: {} {}",topic, data);
                },
                _ => { },
            }
        }
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////

        self.desktop_window.handle_desktop_window(cx, event);

        if let TextEditorEvent::KeyFocusLost = self.textinput.handle_text_input(cx,event) {
            // TODO - detect carriage return
        }

        if let ButtonEvent::Clicked = self.button.handle_normal_button(cx,event) {
            let str = self.textinput.get_value();
            println!("User has asked to load this url: {}",str);
            let _ = self.send.send(Message::BrokerGoto(str));

            // hack;
            // the idea is that i'd be painting some kind of display of the loaded apps
            // TODO this has to interogate the broker or somebody to get an enumeration of all apps...
            // TODO so this presumes services at that level
            let thing = SceneThing { x:0.0, y:0.0, s:0.0, kind:"cube".to_string()};
            self.world_view.add( thing );

        }

        self.world_view.handle_world_view(cx, event);        
    }
    
    pub fn draw_app(&mut self, cx: &mut Cx) {
        if self.desktop_window.begin_desktop_window(cx, Some(&self.menu)).is_err() {
            return
        };
        self.world_view.draw_world_view_2d(cx);

        cx.reset_turtle_pos();

        self.button.draw_normal_button(cx, "GO");

        self.textinput.draw_text_input(cx);

        self.desktop_window.end_desktop_window(cx);
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 3d immediate mode
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct SceneThing {
    pub x: f64,
    pub y: f64,
    pub s: f64,
    pub kind: String,
}

#[derive(Clone)]
pub struct WorldView {
    pub view: View,
    pub time: f64,
    pub sky_box: SkyBox,
    pub cube: DrawCube,
//    pub image: DrawImage,
    pub viewport_3d: Viewport3D,
    pub next_frame: NextFrame,
    pub scene: Vec<SceneThing>,
}

impl WorldView {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            view: View::new(),
            time: 0.0,
            viewport_3d: Viewport3D::new(cx),
            next_frame: NextFrame::default(),
            sky_box: SkyBox::new(cx),
            cube: DrawCube::new(cx, default_shader!()),
            scene: Vec::new(),
        //    image: DrawImage::new(cx, default_shader!()),
        }
    }
    
    pub fn add(&mut self, x: SceneThing) {
        self.scene.push(x);
    }

    pub fn style(cx: &mut Cx) {
        live_body!(cx, r#"
            self::color_bg: #222222;
        "#);
    }
    
    pub fn handle_world_view(&mut self, cx: &mut Cx, event: &mut Event) {
        // do 2D camera interaction.
        self.viewport_3d.handle_viewport_2d(cx, event);
        
        if let Some(ae) = event.is_next_frame(cx, self.next_frame) {
            self.time = ae.time;
            self.view.redraw_view(cx);
        } 
    }
    
    pub fn draw_world_view_2d(&mut self, cx: &mut Cx) {
        
        if self.viewport_3d.begin_viewport_3d(cx).is_ok() {
            self.draw_world_view_3d(cx);
            self.viewport_3d.end_viewport_3d(cx)
        };
        
        // lets draw it
        self.viewport_3d.draw_viewport_2d(cx);
    }
    
    pub fn draw_world_view_3d(&mut self, cx: &mut Cx) {

        if self.view.begin_view(cx, Layout::abs_origin_zero()).is_err() {
            return
        };
        
        self.view.lock_view_transform(cx, &Mat4::identity());
        
        self.sky_box.draw_sky_box(cx);
        
        for i in 0..2000{
            let ti = (i as f32)/4.0 + (self.time*0.1) as f32;
            let mat = Mat4::txyz_s_ry_rx_txyz(
                Vec3{x:0.0,y:0.0,z:0.0},
                1.0,
                (self.time*15.0) as f32 + ti*10.,(self.time*15.0) as f32,
                Vec3{x:0.0, y:0.5, z:-1.5} 
            );
            self.cube.transform = mat; 
            self.cube.cube_size = Vec3{x:0.05,y:0.05,z:0.05};
            self.cube.cube_pos = Vec3{x:ti.sin()*0.8,y:ti.cos()*0.8,z:(ti*3.0).cos()*0.8};
            self.cube.draw_cube(cx);
        }

        let mut cube2 = DrawCube::new(cx, default_shader!());
        for x in &self.scene {
            let mat = Mat4::txyz_s_ry_rx_txyz(
                Vec3{x:0.0,y:0.0,z:0.0},
                1.0,0.0,0.0,
                Vec3{x:0.0, y:0.5, z:-1.5}
            );
            cube2.transform = mat;
            cube2.cube_size=Vec3{x:0.05, y:0.05, z:0.05 };
            cube2.cube_pos=Vec3{x:0.05,y:0.05,z:0.05};
            cube2.set_color(cx, Vec4{x:1.0, y:1.0,z:0.0, w:1.0});
            cube2.draw_cube(cx);
        }
       
        self.view.end_view(cx,);
        self.next_frame = cx.new_next_frame();
    }
    
} 


#[derive(Clone)]
pub struct SkyBox {
    cube: DrawCube,
}

impl SkyBox {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            cube: DrawCube::new(cx, live_shader!(cx, self::shader_sky_box))
        }
    }
    
    pub fn style(cx: &mut Cx) {
        live_body!(cx, r#"
            self::sky_color: #000000;
            self::edge_color: #111111;
            self::floor_color: #8;
            self::size: vec3(200.0, 100.0, 200.0);
            self::pos: vec3(0.0, 50.0, 0.);
            
            self::shader_sky_box: Shader {
                use makepad_render::drawcube::shader::*;
                fn color_form_id() -> vec4 {
                    if geom_id>4.5 {
                        return #f00;
                    }
                    if geom_id>3.5 {
                        return #0f0;
                    }
                    if geom_id>2.5 {
                        return #00f;
                    }
                    if geom_id>1.5 {
                        return #0ff;
                    }
                    return #f0f;
                }
                varying t:float;
                fn vertex() -> vec4 {
                
                    let model_view = camera_view * view_transform * transform ;
                    return camera_projection * (model_view * vec4(
                        geom_pos.x * cube_size.x + cube_pos.x,
                        geom_pos.y * cube_size.y + cube_pos.y,
                        geom_pos.z * cube_size.z + cube_pos.z + draw_zbias,
                        1.
                    ));
                }
                
                fn pixel() -> vec4 { 
                    let x = geom_uv.x;
                    let y = geom_uv.y;
                    // walls
                    let sky = self::sky_color;
                    let edge = self::edge_color;
                    if geom_id>4.5 || geom_id > 3.5 || geom_id < 1.5 {
                        return mix(edge, sky, y);
                    }
                    // floor
                    if geom_id>2.5 {
                        let coord = geom_uv * 150.0;
                        let grid = abs(
                            fract(coord - 0.5) - 0.5
                        ) / (abs(dFdx(coord)) + abs(dFdy(coord)));
                        let line = min(grid.x, grid.y);
                        let grid2 = self::floor_color + 0.4 * vec4(vec3(1.0 - min(line, 1.0)), 1.0);
                        let uv2 = abs(2.0 * geom_uv - 1.0);
                        return mix(grid2, edge, min(max(uv2.x, uv2.y) + 0.7, 1.0));
                    }
                    return sky;
                }
            }
        "#)
    }
    
    pub fn draw_sky_box(&mut self, cx: &mut Cx) {
        self.cube.cube_size = live_vec3!(cx, self::size);
        self.cube.cube_pos = live_vec3!(cx, self::pos);
        self.cube.draw_cube(cx);
    }
}



