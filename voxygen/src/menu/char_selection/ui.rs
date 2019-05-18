use crate::{
    render::Renderer,
    ui::{
        self,
        img_ids::{ImageGraphic, VoxelGraphic},
        ImageSlider, ScaleMode, Ui,
    },
    window::Window,
};
use common::comp::{
    actor::{Belt, BodyType, Chest, Foot, Hand, Head, Pants, Race, Weapon, ALL_CHESTS},
    HumanoidBody,
};
use conrod_core::{
    color,graph,
    color::TRANSPARENT,
    widget::{text_box::Event as TextBoxEvent, Button, Image, Rectangle, Scrollbar, Text, TextBox},
    widget_ids, Borderable, Color, Colorable, Labelable, Positionable, Sizeable, Widget, WidgetCommon,
};
use std::sync::Arc;

widget_ids! {
    struct Ids {
        // Background and logo
        charlist_bg,
        charlist_frame,
        charlist_alignment,
        selection_scrollbar,
        creation_bg,
        creation_frame,
        creation_alignment,
        server_name_text,
        change_server,
        server_frame_bg,
        server_frame,
        v_logo,
        version,
        divider,
        bodyrace_text,
        facialfeatures_text,


        // Characters
        character_box_1,
        character_name_1,
        character_location_1,
        character_level_1,

        character_box_2,
        character_name_2,
        character_location_2,
        character_level_2,


        // Windows
        selection_window,
        char_name,
        char_level,
        creation_window,
        select_window_title,
        creation_buttons_alignment_1, 
        creation_buttons_alignment_2,       
        weapon_heading,
        weapon_description,        
        human_skin_bg,
        orc_skin_bg,
        dwarf_skin_bg,
        undead_skin_bg,
        elf_skin_bg,
        danari_skin_bg,
        name_input_bg,        


        // Buttons
        enter_world_button,
        back_button,
        logout_button,
        create_character_button,
        delete_button,
        create_button,
        name_input,
        name_field,
        race_1,
        race_2,
        race_3,
        race_4,
        race_5,
        race_6,
        body_type_1,
        body_type_2,
        weapon_buttons[],

        // Weapons
        sword,
        sword_button,
        daggers,
        daggers_button,
        axe,
        axe_button,
        hammer,
        hammer_button, 
        bow,
        bow_button,      
        staff,
        staff_button,
        // Char Creation
        // Race Icons
        male,
        female,
        human,
        orc,
        dwarf,
        undead,
        elf,
        danari,
        // Weapon Icons
        weapon_bg,
        weapon_icons[],
        // Arrows
        arrow_left,
        arrow_mid_right_of,
        // Body Features
        window_skin_eyes,
        window_skin_eyes_mid,
        window_skin_eyes_bot,
        window_hair,
        window_hair_mid,
        window_hair_bot,
        window_acessories,
        window_acessories_mid,
        window_acessories_bot,
        skin_color_picker,
        skin_color_slider,
        skin_color_text,
        skin_color_slider_text,
        eye_color_picker,
        eye_color_slider,
        eye_color_text,
        eye_color_slider_text,
        skin_color_slider_range,
        skin_color_slider_indicator,
        hair_color_slider_text,
        chest_slider,
        // Creation Hair Contents
        hair_style_text,
        hair_style_arrow_l,
        hair_style_arrow_r,
        hair_color_picker_bg,
        hair_color_text,
        hair_color_slider_range,
        hair_color_slider_indicator,
        eyebrow_style_text,
        eyebrow_arrow_l,
        eyebrow_arrow_r,
        beard_style_text,
        beard_arrow_l,
        beard_arrow_r,
        // Creation Accessories Contents
        warpaint_text,
        warpaint_arrow_l,
        warpaint_arrow_r,
        warpaint_color_picker_bg,
        warpaint_color_text,
        warpaint_slider_indicator,
        warpaint_slider_range,
        warpaint_slider_text,
    }
}

image_ids! {
    struct Imgs {
        <VoxelGraphic>
        v_logo: "/voxygen/element/v_logo.vox",
        button: "/voxygen/element/buttons/button.vox",
        button_hover: "/voxygen/element/buttons/button_hover.vox",
        button_press: "/voxygen/element/buttons/button_press.vox",
        button_red: "/voxygen/element/buttons/button_red.vox",
        button_red_hover: "/voxygen/element/buttons/button_red_hover.vox",
        button_red_press: "/voxygen/element/buttons/button_red_press.vox",
        name_input: "/voxygen/element/misc_bg/textbox.vox",
        charlist_frame: "/voxygen/element/frames/window_4.vox",
        selection_frame: "/voxygen/element/frames/selection_frame.vox",
        server_frame: "/voxygen/element/frames/server_frame.vox",
        selection: "/voxygen/element/frames/selection.vox",

        arrow_left:"/voxygen/element/buttons/button_red_press.vox",
        arrow_left_mo:"/voxygen/element/buttons/button_red_press.vox",
        arrow_left_press:"/voxygen/element/buttons/button_red_press.vox",

        divider: "/voxygen/element/frames/divider.vox",

        <ImageGraphic>    
        frame_closed: "/voxygen/element/buttons/frame/closed.png",
        frame_closed_mo: "/voxygen/element/buttons/frame/closed_mo.png",
        frame_closed_press: "/voxygen/element/buttons/frame/closed_press.png",
        frame_open: "/voxygen/element/buttons/frame/open.png",
        frame_open_mo: "/voxygen/element/buttons/frame/open_mo.png",
        frame_open_press: "/voxygen/element/buttons/frame/open_press.png",
        skin_eyes_window: "/voxygen/element/frames/skin_eyes.png",
        hair_window: "/voxygen/element/frames/skin_eyes.png",
        accessories_window: "/voxygen/element/frames/skin_eyes.png",
        color_picker_bg: "/voxygen/element/misc_bg/color_picker_blank.png",
        slider_range: "/voxygen/element/slider/track.png",
        slider_indicator: "/voxygen/element/slider/indicator.png",
        window_frame_2: "/voxygen/element/frames/window_2.png",


        // Weapon Icons
        daggers: "/voxygen/element/icons/daggers.png",
        sword_shield: "/voxygen/element/icons/swordshield.png",
        sword: "/voxygen/element/icons/sword.png",
        axe: "/voxygen/element/icons/axe.png",
        hammer: "/voxygen/element/icons/hammer.png",
        bow: "/voxygen/element/icons/bow.png",
        staff: "/voxygen/element/icons/staff.png",
        // Race Icons
        male: "/voxygen/element/icons/male.png",
        female: "/voxygen/element/icons/female.png",
        human_m: "/voxygen/element/icons/human_m.png",
        human_f: "/voxygen/element/icons/human_f.png",
        orc_m: "/voxygen/element/icons/orc_m.png",
        orc_f: "/voxygen/element/icons/orc_f.png",
        dwarf_m: "/voxygen/element/icons/dwarf_m.png",
        dwarf_f: "/voxygen/element/icons/dwarf_f.png",
        undead_m: "/voxygen/element/icons/ud_m.png",
        undead_f: "/voxygen/element/icons/ud_f.png",
        elf_m: "/voxygen/element/icons/elf_m.png",
        elf_f: "/voxygen/element/icons/elf_f.png",
        danari_m: "/voxygen/element/icons/danari_m.png",
        danari_f: "/voxygen/element/icons/danari_f.png",    
        // Icon Borders
        icon_border: "/voxygen/element/buttons/border.png",
        icon_border_mo: "/voxygen/element/buttons/border_mo.png",
        icon_border_press: "/voxygen/element/buttons/border_press.png",
        icon_border_pressed: "/voxygen/element/buttons/border_pressed.png",
        nothing: "/voxygen/element/nothing.png",
    }
}

font_ids! {
    pub struct Fonts {
        opensans: "/voxygen/font/OpenSans-Regular.ttf",
        metamorph: "/voxygen/font/Metamorphous-Regular.ttf",
    }
}

pub enum Event {
    Logout,
    Play,
}

const TEXT_COLOR: Color = Color::Rgba(1.0, 1.0, 1.0, 1.0);

pub struct CharSelectionUi {
    ui: Ui,
    ids: Ids,
    imgs: Imgs,
    fonts: Fonts,
    character_creation: bool,    
    pub character_name: String,
    pub character_body: HumanoidBody, 
    pub character_weapon: Weapon, 
    pub body_type: BodyType,       
}

impl CharSelectionUi {
    pub fn new(window: &mut Window) -> Self {
        let mut ui = Ui::new(window).unwrap();
<<<<<<< HEAD
        // TODO: Adjust/remove this, right now it is used to demonstrate window scaling functionality.
=======
        // TODO: adjust/remove this, mid_right_of now it is used to demonstrate window scaling functionality
>>>>>>> Update
        ui.scaling_mode(ScaleMode::RelativeToWindow([1920.0, 1080.0].into()));
        // Generate ids
        let ids = Ids::new(ui.id_generator());
        // Load images
        let imgs = Imgs::load(&mut ui).expect("Failed to load images!");
        // Load fonts
        let fonts = Fonts::load(&mut ui).expect("Failed to load fonts!");

        // TODO: Randomize initial values.
        Self {
            ui,
            ids,
            imgs,
            fonts,
            character_creation: false,            
            character_name: "Character Name".to_string(),
            character_body: HumanoidBody::random(), 
            character_weapon: Weapon::Sword,   
            body_type: BodyType::Male, 
        }
    }

    // TODO: Split this into multiple modules or functions.
    fn update_layout(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        let ref mut ui_widgets = self.ui.set_widgets();
        let version = env!("CARGO_PKG_VERSION");

        // Character Selection /////////////////
        if !self.character_creation {
            // Background for Server Frame
            Rectangle::fill_with([386.0, 95.0], color::rgba(0.0, 0.0, 0.0, 0.8))
                .top_left_with_margins_on(ui_widgets.window, 30.0, 30.0)
                .set(self.ids.server_frame_bg, ui_widgets);
            Image::new(self.imgs.server_frame)
                .w_h(400.0, 100.0)
                .middle_of(self.ids.server_frame_bg)
                .set(self.ids.server_frame, ui_widgets);

            // Background for Char List
            Rectangle::fill_with([386.0, 788.0], color::rgba(0.0, 0.0, 0.0, 0.8))
                .down_from(self.ids.server_frame_bg, 20.0)
                .set(self.ids.charlist_bg, ui_widgets);
            Image::new(self.imgs.charlist_frame)
                .w_h(400.0, 800.0)
                .middle_of(self.ids.charlist_bg)
                .set(self.ids.charlist_frame, ui_widgets);
            Rectangle::fill_with([386.0, 783.0], color::TRANSPARENT)
                .middle_of(self.ids.charlist_bg)
                .scroll_kids()
                .scroll_kids_vertically()
                .set(self.ids.charlist_alignment, ui_widgets);
            Scrollbar::y_axis(self.ids.charlist_alignment)
                .thickness(5.0)
                .auto_hide(true)
                .rgba(0.33, 0.33, 0.33, 1.0)
                .set(self.ids.selection_scrollbar, ui_widgets);
            // Server Name
            Text::new("Server Name") //TODO: Add in Server Name
                .mid_top_with_margin_on(self.ids.server_frame_bg, 5.0)
                .font_size(24)
                .font_id(self.fonts.metamorph)
                .color(TEXT_COLOR)
                .set(self.ids.server_name_text, ui_widgets);
            //Change Server
            if Button::image(self.imgs.button)
                .mid_top_with_margin_on(self.ids.server_frame_bg, 45.0)
                .w_h(200.0, 40.0)
                .parent(self.ids.charlist_bg)
                .hover_image(self.imgs.button_hover)
                .press_image(self.imgs.button_press)
                .label("Change Server")
                .label_color(TEXT_COLOR)
                .label_font_size(18)
                .label_y(conrod_core::position::Relative::Scalar(3.0))
                .set(self.ids.change_server, ui_widgets)
                .was_clicked()
            {
                events.push(Event::Logout);
            }

            // Enter World Button
            if Button::image(self.imgs.button)
                .mid_bottom_with_margin_on(ui_widgets.window, 10.0)
                .w_h(250.0, 60.0)
                .hover_image(self.imgs.button_hover)
                .press_image(self.imgs.button_press)
                .label("Enter World")
                .label_color(TEXT_COLOR)
                .label_font_size(22)
                .label_y(conrod_core::position::Relative::Scalar(3.0))
                .set(self.ids.enter_world_button, ui_widgets)
                .was_clicked()
            {
                events.push(Event::Play);
            }

            // Logout_Button
            if Button::image(self.imgs.button)
                .bottom_left_with_margins_on(ui_widgets.window, 10.0, 10.0)
                .w_h(150.0, 40.0)
                .hover_image(self.imgs.button_hover)
                .press_image(self.imgs.button_press)
                .label("Logout")
                .label_color(TEXT_COLOR)
                .label_font_size(18)
                .label_y(conrod_core::position::Relative::Scalar(3.0))
                .set(self.ids.logout_button, ui_widgets)
                .was_clicked()
            {
                events.push(Event::Logout);
            }

            // Create Character Button.
            if Button::image(self.imgs.button)
                .mid_bottom_with_margin_on(self.ids.charlist_bg, -60.0)
                .w_h(270.0, 50.0)
                .hover_image(self.imgs.button_hover)
                .press_image(self.imgs.button_press)
                .label("Create Character")
                .label_color(TEXT_COLOR)
                .label_font_size(20)
                .label_y(conrod_core::position::Relative::Scalar(3.0))
                .set(self.ids.create_character_button, ui_widgets)
                .was_clicked()
            {
                self.character_creation = true;                
            }

            // Alpha Version
            Text::new(version)
                .top_right_with_margins_on(ui_widgets.window, 5.0, 5.0)
                .font_size(14)
                .font_id(self.fonts.opensans)
                .color(TEXT_COLOR)
                .set(self.ids.version, ui_widgets);

            // 1st Character in Selection List           
            if Button::image(self.imgs.selection)
                .top_left_with_margins_on(self.ids.charlist_alignment, 0.0, 2.0)
                .w_h(386.0, 80.0)
                .image_color(Color::Rgba(1.0, 1.0, 1.0, 0.8))
                .hover_image(self.imgs.selection)
                .press_image(self.imgs.selection)
                .label_y(conrod_core::position::Relative::Scalar(20.0))
                .set(self.ids.character_box_1, ui_widgets)
                .was_clicked()
            {
                
            }
            Text::new("Human Default")
                .top_left_with_margins_on(self.ids.character_box_1, 6.0, 9.0)
                .font_size(19)
                .font_id(self.fonts.metamorph)
                .color(TEXT_COLOR)
                .set(self.ids.character_name_1, ui_widgets);

            Text::new("Level 1")
                .down_from(self.ids.character_name_1, 4.0)
                .font_size(17)
                .font_id(self.fonts.opensans)
                .color(TEXT_COLOR)
                .set(self.ids.character_level_1, ui_widgets);

            Text::new("Uncanny Valley")
                .down_from(self.ids.character_level_1, 4.0)
                .font_size(17)
                .font_id(self.fonts.opensans)
                .color(TEXT_COLOR)
                .set(self.ids.character_location_1, ui_widgets);

            // 2nd Character in List
            if Button::image(self.imgs.nothing)
                .down_from(self.ids.character_box_1, 5.0)
                .w_h(386.0, 80.0)
                .hover_image(self.imgs.selection)
                .press_image(self.imgs.selection)
                .image_color(Color::Rgba(1.0, 1.0, 1.0, 0.8))
                .label_y(conrod_core::position::Relative::Scalar(20.0))
                .set(self.ids.character_box_2, ui_widgets)
                .was_clicked()
            {
                
            }
            Text::new("Example 2nd Char")
                .top_left_with_margins_on(self.ids.character_box_2, 6.0, 9.0)
                .font_size(19)
                .font_id(self.fonts.metamorph)
                .color(TEXT_COLOR)
                .set(self.ids.character_name_2, ui_widgets);

            Text::new("Level ??")
                .down_from(self.ids.character_name_2, 4.0)
                .font_size(17)
                .font_id(self.fonts.opensans)
                .color(TEXT_COLOR)
                .set(self.ids.character_level_2, ui_widgets);

            Text::new("Plains of Uncertainty")
                .down_from(self.ids.character_level_2, 4.0)
                .font_size(17)
                .font_id(self.fonts.opensans)
                .color(TEXT_COLOR)
                .set(self.ids.character_location_2, ui_widgets);

            
            
        }
       
       
        // Character_Creation //////////////////////////////////////////////////////////////////////

        else {
            // Back Button
            if Button::image(self.imgs.button)
                .bottom_left_with_margins_on(ui_widgets.window, 10.0, 10.0)
                .w_h(150.0, 40.0)
                .hover_image(self.imgs.button_hover)
                .press_image(self.imgs.button_press)
                .label("Back")
                .label_color(TEXT_COLOR)
                .label_font_size(18)
                .label_y(conrod_core::position::Relative::Scalar(3.0))
                .set(self.ids.back_button, ui_widgets)
                .was_clicked()
            {
                self.character_creation = false;
            }
            // Create Button
            if Button::image(self.imgs.button)
                .bottom_right_with_margins_on(ui_widgets.window, 10.0, 10.0)
                .w_h(150.0, 40.0)
                .hover_image(self.imgs.button_hover)
                .press_image(self.imgs.button_press)
                .label("Create")
                .label_color(TEXT_COLOR)
                .label_font_size(18)
                .label_y(conrod_core::position::Relative::Scalar(3.0))
                .set(self.ids.create_button, ui_widgets)
                .was_clicked()
            {
                // TODO: Save character.
                self.character_creation = false;
            }
            // Character Name Input
            Rectangle::fill_with([320.0, 50.0], color::rgba(0.0, 0.0, 0.0, 0.97))
                .mid_bottom_with_margin_on(ui_widgets.window, 20.0)
                .set(self.ids.name_input_bg, ui_widgets);
            Button::image(self.imgs.name_input)
                .image_color(Color::Rgba(1.0, 1.0, 1.0, 0.9))
                .w_h(337.0, 67.0)
                .middle_of(self.ids.name_input_bg)
                .set(self.ids.name_input, ui_widgets);
            for event in TextBox::new(&self.character_name)
                .w_h(300.0, 60.0)
                .mid_top_with_margin_on(self.ids.name_input, 2.0)
                .font_size(26)
                .font_id(self.fonts.metamorph)
                .center_justify()
                .text_color(TEXT_COLOR)
                .color(TRANSPARENT)
                .border_color(TRANSPARENT)
                .set(self.ids.name_field, ui_widgets)
            {
                match event {
                    TextBoxEvent::Update(name) => {
                        self.character_name = name;
                    }
                    TextBoxEvent::Enter => {}
                }
            }

            // Window       

            Rectangle::fill_with([386.0, 988.0], color::rgba(0.0, 0.0, 0.0, 0.8))
                .top_left_with_margins_on(ui_widgets.window, 30.0, 30.0)
                .set(self.ids.creation_bg, ui_widgets);
            Image::new(self.imgs.charlist_frame)
                .w_h(400.0, 1000.0)
                .middle_of(self.ids.creation_bg)
                .set(self.ids.charlist_frame, ui_widgets);
            Rectangle::fill_with([386.0, 983.0], color::TRANSPARENT)
                .middle_of(self.ids.creation_bg)
                .scroll_kids()
                .scroll_kids_vertically()
                .set(self.ids.creation_alignment, ui_widgets);
            Scrollbar::y_axis(self.ids.creation_alignment)
                .thickness(5.0)
                .auto_hide(true)
                .rgba(0.33, 0.33, 0.33, 1.0)
                .set(self.ids.selection_scrollbar, ui_widgets);            
                           

                // Male/Female/Race Icons
                
                Text::new("Body & Weapon")
                .mid_top_with_margin_on(self.ids.creation_alignment, 10.0)
                .font_size(24)
                .font_id(self.fonts.metamorph)
                .color(TEXT_COLOR)
                .set(self.ids.bodyrace_text, ui_widgets);
                // Alignment
                Rectangle::fill_with([136.0, 68.0], color::TRANSPARENT)
                    .mid_top_with_margin_on(self.ids.creation_alignment, 60.0)
                    .set(self.ids.creation_buttons_alignment_1, ui_widgets);
                // Male
                Image::new(self.imgs.male)
                    .w_h(68.0, 68.0)
                    .top_left_with_margins_on(self.ids.creation_buttons_alignment_1, 0.0, 0.0)
                    .set(self.ids.male, ui_widgets);
                if Button::image(if let BodyType::Male = self.character_body.body_type {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.male)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)                
                .set(self.ids.body_type_1, ui_widgets)
                .was_clicked()
                {
                    self.character_body.body_type = BodyType::Male;
                }
                // Female
                Image::new(self.imgs.female)
                    .w_h(68.0, 68.0)
                    .top_right_with_margins_on(self.ids.creation_buttons_alignment_1, 0.0, 0.0)
                    .set(self.ids.female, ui_widgets);
                if Button::image(if let BodyType::Female = self.character_body.body_type {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.female)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.body_type_2, ui_widgets)
                .was_clicked()
                {
                    self.character_body.body_type = BodyType::Female;
                }     

                // Alignment for Races and Weapons
                 Rectangle::fill_with([260.0, 180.0], color::RED)
                    .down_from(self.ids.creation_alignment, 30.0)
                    .set(self.ids.creation_buttons_alignment_2, ui_widgets);         

                // Human
                Image::new(if let BodyType::Male = self.character_body.body_type {
                    self.imgs.human_m
                } else {
                    self.imgs.human_f
                })
                .w_h(60.0, 60.0)
                .top_left_with_margins_on(self.ids.creation_buttons_alignment_2, 0.0, 0.0)
                .set(self.ids.human, ui_widgets);
                if Button::image(if let Race::Human = self.character_body.race {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.human)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.race_1, ui_widgets)
                .was_clicked()
                {
                    self.character_body.race = Race::Human;
                }

                // Orc
                Image::new(if let BodyType::Male = self.character_body.body_type {
                    self.imgs.orc_m
                } else {
                    self.imgs.orc_f
                })
                .w_h(60.0, 60.0)
                .right_from(self.ids.human, 0.0)
                .set(self.ids.orc, ui_widgets);
                if Button::image(if let Race::Orc = self.character_body.race {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.orc)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.race_2, ui_widgets)
                .was_clicked()
                {
                    self.character_body.race = Race::Orc;
                }
                // Dwarf
                Image::new(if let BodyType::Male = self.character_body.body_type {
                    self.imgs.dwarf_m
                } else {
                    self.imgs.dwarf_f
                })
                .w_h(60.0, 60.0)
                .right_from(self.ids.orc, 0.0)
                .set(self.ids.dwarf, ui_widgets);
                if Button::image(if let Race::Dwarf = self.character_body.race {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.dwarf)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.race_3, ui_widgets)
                .was_clicked()
                {
                    self.character_body.race = Race::Dwarf;
                }
                // Elf
                Image::new(if let BodyType::Male = self.character_body.body_type {
                    self.imgs.elf_m
                } else {
                    self.imgs.elf_f
                })
                .w_h(60.0, 60.0)
                .down_from(self.ids.human, 1.0)
                .set(self.ids.elf, ui_widgets);
                if Button::image(if let Race::Elf = self.character_body.race {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.elf)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.race_4, ui_widgets)
                .was_clicked()
                {
                    self.character_body.race = Race::Elf;
                }
                // Undead
                Image::new(if let BodyType::Male = self.character_body.body_type {
                    self.imgs.undead_m
                } else {
                    self.imgs.undead_f
                })
                .w_h(60.0, 60.0)
                .right_from(self.ids.elf, 0.0)
                .set(self.ids.undead, ui_widgets);
                if Button::image(if let Race::Undead = self.character_body.race {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.undead)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.race_5, ui_widgets)
                .was_clicked()
                {
                    self.character_body.race = Race::Undead;
                }
                // Danari
                Image::new(if let BodyType::Male = self.character_body.body_type {
                    self.imgs.danari_m
                } else {
                    self.imgs.danari_f
                })
                .right_from(self.ids.undead, 0.0)
                .set(self.ids.danari, ui_widgets);
                if Button::image(if let Race::Danari = self.character_body.race {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .w_h(60.0, 60.0)
                .middle_of(self.ids.danari)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.race_6, ui_widgets)
                .was_clicked()
                {
                    self.character_body.race = Race::Danari;
                }

                // Hammer

                 Image::new(self.imgs.hammer)
                    .w_h(60.0, 60.0)
                    .bottom_left_with_margins_on(self.ids.creation_buttons_alignment_2, 0.0, 0.0)
                    .set(self.ids.hammer, ui_widgets);
                if Button::image(if let Weapon::Hammer = self.character_body.weapon {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.hammer)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.hammer_button, ui_widgets)
                .was_clicked()
                {
                    self.character_body.weapon = Weapon::Hammer;
                }

                // Bow

                 Image::new(self.imgs.bow)
                    .w_h(60.0, 60.0)
                    .right_from(self.ids.hammer, 0.0)
                    .set(self.ids.bow, ui_widgets);
                if Button::image(if let Weapon::Bow = self.character_body.weapon {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.bow)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.bow_button, ui_widgets)
                .was_clicked()
                {
                    self.character_body.weapon = Weapon::Bow;
                }
                // Staff

                 Image::new(self.imgs.staff)
                    .w_h(60.0, 60.0)
                    .right_from(self.ids.bow, 0.0)
                    .set(self.ids.staff, ui_widgets);
                if Button::image(if let Weapon::Staff = self.character_body.weapon {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.staff)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.staff_button, ui_widgets)
                .was_clicked()
                {
                    self.character_body.weapon = Weapon::Staff;
                }
                // Sword

                Image::new(self.imgs.sword)
                    .w_h(60.0, 60.0)
                    .up_from(self.ids.axe, 0.0)
                    .set(self.ids.sword, ui_widgets);
                if Button::image(if let Weapon::Sword = self.character_body.weapon {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.sword)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.sword_button, ui_widgets)
                .was_clicked()
                {
                    self.character_body.weapon = Weapon::Sword;
                }

                // Daggers
                Image::new(self.imgs.daggers)
                    .w_h(60.0, 60.0)
                    .right_from(self.ids.sword, 0.0)
                    .set(self.ids.daggers, ui_widgets);
                if Button::image(if let Weapon::Daggers = self.character_body.weapon {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.daggers)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.daggers_button, ui_widgets)
                .was_clicked()
                {
                    self.character_body.weapon = Weapon::Daggers;
                }

                // Axe
                Image::new(self.imgs.axe)
                    .w_h(60.0, 60.0)
                    .right_from(self.ids.daggers, 0.0)
                    .set(self.ids.axe, ui_widgets);
                if Button::image(if let Weapon::Axe = self.character_body.weapon {
                    self.imgs.icon_border_pressed
                } else {
                    self.imgs.icon_border
                })
                .middle_of(self.ids.axe)
                .hover_image(self.imgs.icon_border_mo)
                .press_image(self.imgs.icon_border_press)
                .set(self.ids.axe_button, ui_widgets)
                .was_clicked()
                {
                    self.character_body.weapon = Weapon::Axe;
                }       
                //Divider
                Image::new(self.imgs.divider)
                    .w_h(300.0, 6.0)
                    .mid_bottom_with_margin_on(self.ids.creation_buttons_alignment_2, -36.0)
                    .set(self.ids.divider, ui_widgets);                
                
                // Facial Features
                Text::new("Facial Features")
                    .mid_top_with_margin_on(self.ids.divider, 26.0)
                    .font_size(24)
                    .font_id(self.fonts.metamorph)
                    .color(TEXT_COLOR)
                    .set(self.ids.facialfeatures_text, ui_widgets);
        
                // Sliders

                // Hair Style

                // Skin

                // Eye Brows

                // Eye Color

                // Acessories




            

               
        } // Char Creation fin

        events
    }

    pub fn handle_event(&mut self, event: ui::Event) {
        self.ui.handle_event(event);
    }

    pub fn maintain(&mut self, renderer: &mut Renderer) -> Vec<Event> {
        let events = self.update_layout();
        self.ui.maintain(renderer);
        events
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.ui.render(renderer);
    }
}
