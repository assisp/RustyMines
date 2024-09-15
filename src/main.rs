use std::time::{SystemTime, Duration};

use eframe::egui;
use egui::{Color32, FontFamily, FontId, TextStyle};
use FontFamily::Proportional;
use TextStyle::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

const COLS: u16 = 20;
const ROWS: u16 = 20;
const MINES: u16 = 75;
const CELL_SIZE: f32 = 30.0;

const UTF8_SMILE: [u8; 4] = [0xF0, 0x9F, 0x98, 0x90];
const UTF8_WINNER: [u8; 4] = [0xF0, 0x9F, 0x98, 0x8E];
const UTF8_LOOSER: [u8; 4] = [0xF0, 0x9F, 0x98, 0x9E];
const UTF8_BOMB: [u8; 4] = [0xF0, 0x9F, 0x92, 0xA3];
const UTF8_FLAG: [u8; 4] = [0xF0,0x9F,0x9A,0xA9];
const UTF8_CROSS: [u8; 4] = [0xF0, 0x9F, 0x8E, 0x8C];
const UTF8_DEAD: [u8; 4] = [0xF0, 0x9F, 0x95, 0xB1];
//const UTF8_CHECK: [u8; 4] = [0x00, 0xE2, 0x9C, 0x85];

fn main() {

    // set the window frame viewport size
    let mut options = eframe::NativeOptions::default();
    options.viewport.inner_size = Some(egui::Vec2 { 
        x: (CELL_SIZE + 1.0) * f32::from(COLS) + 14.,
        y: (CELL_SIZE + 1.0) * f32::from(ROWS) + 104.
    });
    options.viewport.min_inner_size = Some(egui::Vec2 { 
        x: (CELL_SIZE + 1.0) * 4. + 14., 
        y: (CELL_SIZE + 1.0) * 4. + 54.
    });

    let _ = eframe::run_native("RustyMines", options, 
        Box::new(|cc| Ok(Box::new(AppGui::new(
            cc, 
            ROWS.into(), 
            COLS.into(), 
            MINES.into())))));
}

/// The colors for a catppuccin theme variant.
/// from https://github.com/catppuccin/egui
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MyTheme {
    pub rosewater: Color32,
    pub flamingo: Color32,
    pub pink: Color32,
    pub mauve: Color32,
    pub red: Color32,
    pub maroon: Color32,
    pub peach: Color32,
    pub yellow: Color32,
    pub green: Color32,
    pub teal: Color32,
    pub sky: Color32,
    pub sapphire: Color32,
    pub blue: Color32,
    pub lavender: Color32,
    pub text: Color32,
    pub subtext1: Color32,
    pub subtext0: Color32,
    pub overlay2: Color32,
    pub overlay1: Color32,
    pub overlay0: Color32,
    pub surface2: Color32,
    pub surface1: Color32,
    pub surface0: Color32,
    pub base: Color32,
    pub mantle: Color32,
    pub crust: Color32,
}
//Latte colors from catppuccin
pub const LATTE: MyTheme = MyTheme {
    rosewater: Color32::from_rgb(220, 138, 120),
    flamingo: Color32::from_rgb(221, 120, 120),
    pink: Color32::from_rgb(234, 118, 203),
    mauve: Color32::from_rgb(136, 57, 239),
    red: Color32::from_rgb(210, 15, 57),
    maroon: Color32::from_rgb(230, 69, 83),
    peach: Color32::from_rgb(254, 100, 11),
    yellow: Color32::from_rgb(223, 142, 29),
    green: Color32::from_rgb(64, 160, 43),
    teal: Color32::from_rgb(23, 146, 153),
    sky: Color32::from_rgb(4, 165, 229),
    sapphire: Color32::from_rgb(32, 159, 181),
    blue: Color32::from_rgb(30, 102, 245),
    lavender: Color32::from_rgb(114, 135, 253),
    text: Color32::from_rgb(76, 79, 105),
    subtext1: Color32::from_rgb(92, 95, 119),
    subtext0: Color32::from_rgb(108, 111, 133),
    overlay2: Color32::from_rgb(124, 127, 147),
    overlay1: Color32::from_rgb(140, 143, 161),
    overlay0: Color32::from_rgb(156, 160, 176),
    surface2: Color32::from_rgb(172, 176, 190),
    surface1: Color32::from_rgb(188, 192, 204),
    surface0: Color32::from_rgb(204, 208, 218),
    base: Color32::from_rgb(239, 241, 245),
    mantle: Color32::from_rgb(230, 233, 239),
    crust: Color32::from_rgb(220, 224, 232),
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellTag {
    UNSELECTED, //unselected cell
    FLAGGED,    //flagged cell
    SELECTED,   //selected cell
    DEAD,       //we got killed in this cell
}

// cell data
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    val: u8, // number of neighbour mines(0..8) or 9 for mined
    flag: CellTag,
} 

//get a new map: Vec<Cell> with len = rows * columns
fn get_map_vec(rows: usize, columns: usize, n_mines: usize) -> Vec<Cell> {
    let len = rows * columns;
    //set the cells value to 0 and flagged with UNSELECTED
    let mut m_vec: Vec<Cell> = vec![Cell{val: 0, flag: CellTag::UNSELECTED}; len];

    //set mines in the first n_mines positions of the vector
    for v in m_vec.iter_mut().take(n_mines) {
        v.val = 9;
    }
    
    //shuffle the vector
    m_vec.shuffle(&mut thread_rng());
    //shuffle again
    m_vec.shuffle(&mut thread_rng());

    // for each cell count the neighbouring mines
    // at NW, W, SW, N, S, NE, E and SE postions
    for r in 0..rows {
        for c in 0..columns {
       
            let ind = (r * columns) + c;

            //this cell is mined jump to the next one
            if m_vec[ind].val == 9 {
                continue;
            }
           
            let mut count = 0;
            //NW
            if r > 0 && c > 0 && 
                m_vec[((r-1)*columns) + (c-1)].val == 9 {
                    count += 1;
            }
            //W
            if c > 0 && 
                m_vec[(r*columns) + (c-1)].val == 9 {
                    count += 1;
            }
            //SW
            if r < rows - 1 && c > 0 &&
                m_vec[((r+1)*columns) + (c-1)].val == 9 {
                    count += 1;
            }
            //N
            if r > 0 &&
                m_vec[((r-1)*columns) + c].val == 9 {
                    count += 1;
            }
            //S
            if r < rows - 1 &&
                m_vec[((r+1)*columns) + c].val == 9 {
                    count += 1;
            }
            //NE
            if r > 0 && c < columns - 1 &&
                m_vec[((r-1)*columns) + (c+1)].val == 9 {
                    count += 1;
            }
            //E
            if c < columns - 1 &&
                m_vec[(r*columns) + (c+1)].val == 9 {
                    count += 1;
            }
            //SE
            if r < rows - 1 && c < columns - 1 &&
                m_vec[((r+1)*columns) + (c+1)].val == 9 {
                    count += 1;
            } 

            m_vec[ind].val = count;
        }
    }
       
    m_vec
}

//App Data
pub struct AppGui {
    welcome: bool, //display welcome menu
    state: u8, //0-playing 1-winner 2-looser
    s_time: SystemTime, //game start time
    f_time: f64, //game time (elapsed seconds)
    selected: usize, //total cells selected
    s_rows: usize, //rows in settings (welcome menu)
    s_columns: usize, //columns in settings (welcome menu)
    n_mines: usize, //total number of mines
    f_mines: usize, //flagged mines
    rows: usize, //map rows
    columns: usize, //map columns
    map: Vec<Cell>, //game map    
}

//eframe::egui stuff
impl AppGui {
    //App creation
    pub fn new(cc: &eframe::CreationContext<'_>,
        rows: usize, 
        columns: usize, 
        n_mines: usize) -> Self {

        let mut this = Self {
            welcome: true,
            state: 0,
            s_time: SystemTime::now(),
            f_time: 0.0,
            selected: 0,
            s_rows: rows,
            s_columns: columns,
            n_mines,
            f_mines: 0,
            rows,
            columns,
            map: get_map_vec(rows, columns, n_mines),
        };
        //set the default visuals and style
        this.set_visuals(&cc.egui_ctx);

        this
    }

    // set the default visuals - LATTE from catppuccin
    // and style
    pub fn set_visuals(&mut self, ctx: &egui::Context) {

        let mut visuals = egui::Visuals::light();

        visuals.override_text_color = Some(LATTE.text);
        visuals.hyperlink_color = LATTE.rosewater;
        visuals.faint_bg_color = LATTE.surface0;
        visuals.extreme_bg_color = LATTE.crust;
        visuals.code_bg_color = LATTE.mantle;
        visuals.warn_fg_color = LATTE.peach;
        visuals.error_fg_color = LATTE.maroon;
        visuals.window_fill = LATTE.base;
        visuals.panel_fill = LATTE.base;
        visuals.window_stroke.color = LATTE.overlay1;
        visuals.widgets.noninteractive.bg_fill = LATTE.base;
        visuals.widgets.noninteractive.weak_bg_fill = LATTE.base;
        visuals.widgets.noninteractive.bg_stroke.color = LATTE.overlay1;
        visuals.widgets.noninteractive.fg_stroke.color = LATTE.text;
        visuals.widgets.inactive.bg_fill = LATTE.surface0;
        visuals.widgets.inactive.weak_bg_fill = LATTE.surface0;
        visuals.widgets.inactive.bg_stroke.color = LATTE.overlay1;
        visuals.widgets.inactive.fg_stroke.color = LATTE.text;
        visuals.widgets.active.bg_fill = LATTE.surface1;
        visuals.widgets.active.weak_bg_fill = LATTE.surface1;
        visuals.widgets.active.bg_stroke.color = LATTE.overlay1;
        visuals.widgets.active.fg_stroke.color = LATTE.text;
        visuals.widgets.hovered.bg_fill = LATTE.surface2;
        visuals.widgets.hovered.weak_bg_fill = LATTE.surface2;
        visuals.widgets.hovered.bg_stroke.color = LATTE.overlay1;
        visuals.widgets.hovered.fg_stroke.color = LATTE.text;
        visuals.widgets.open.bg_fill = LATTE.surface0;
        visuals.widgets.open.weak_bg_fill = LATTE.surface0;
        visuals.widgets.open.bg_stroke.color = LATTE.overlay1;
        visuals.widgets.open.fg_stroke.color = LATTE.text;
        visuals.selection.bg_fill = LATTE.blue.linear_multiply(0.4);
        visuals.selection.stroke.color = LATTE.overlay1;
        visuals.window_shadow.color = LATTE.base;
        visuals.popup_shadow.color = LATTE.base;
        visuals.dark_mode = false;
        
        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing = egui::Vec2::new(1.0, 1.0);
        
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(18.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ].into();
        
        ctx.set_style(style);
        ctx.set_visuals(visuals);
    }

    // set the map cell with index ind to SELECTED 
    // and increment the App selected value
    pub fn set_selected(&mut self, ind: usize) {
        self.map[ind].flag = CellTag::SELECTED;
        self.selected += 1;
    }
    
    //if a 0 cell is selected clean all neighbouring cells with 0 
    //up to the first non zero cell
    pub fn clean_neighbour_cells(&mut self, row: usize, column: usize) {
        
        // create a vector to push zero value cells to check neighbours
        let mut cells: Vec<(usize, usize)> = Vec::new();
        cells.push((row,column));

        // pop the cells until the vector is empty
        while let Some((r, c)) = cells.pop() {

            let ind = (r * self.columns) + c;
            
            //if it's not a zero value cell set it to selected 
            //and jump to the next one in the vector
            if self.map[ind].val > 0 {
                if self.map[ind].flag == CellTag::UNSELECTED {
                    self.set_selected(ind);
                }
                continue;
            }

            //if the neighbour cell it's not a mine flag it selected 
            //and push it to the vector to have it's neighbours checked

            //NW
            if r > 0 && c > 0 {
                let i = ((r - 1) * self.columns) + (c - 1);  
                if self.map[i].val < 9 && self.map[i].flag == CellTag::UNSELECTED {
                    self.set_selected(i);
                    cells.push((r - 1, c - 1));
                }
            }
            //W
            if c > 0 {
                let i = (r * self.columns) + (c - 1);  
                if self.map[i].val < 9 && self.map[i].flag == CellTag::UNSELECTED {
                    self.set_selected(i);
                    cells.push((r, c - 1));
                }
            }
            //SW
            if r < self.rows - 1 && c > 0 {
                let i = ((r + 1) * self.columns) + (c - 1);  
                if self.map[i].val < 9 && self.map[i].flag == CellTag::UNSELECTED {
                    self.set_selected(i);
                    cells.push((r + 1, c - 1));
                }
            }
            //N
            if r > 0 {
                let i = ((r - 1) * self.columns) + c;
                if self.map[i].val < 9 && self.map[i].flag == CellTag::UNSELECTED {
                    self.set_selected(i);
                    cells.push((r - 1, c));
                }
            }
            //S
            if r < self.rows - 1 {
                let i = ((r + 1) * self.columns) + c;
                if self.map[i].val < 9 && self.map[i].flag == CellTag::UNSELECTED {
                    self.set_selected(i);
                    cells.push((r + 1, c));
                }
            }
            //E
            if c < self.columns - 1 {
                let i = (r * self.columns) + (c + 1);
                if self.map[i].val < 9 && self.map[i].flag == CellTag::UNSELECTED {
                    self.set_selected(i);
                    cells.push((r, c + 1));
                }
            }
            //NE
            if r > 0 && c < self.columns - 1 {
                let i = ((r - 1) * self.columns) + (c + 1);  
                if self.map[i].val < 9 && self.map[i].flag == CellTag::UNSELECTED {
                    self.set_selected(i);
                    cells.push((r - 1, c + 1));
                }
            }
            //SE
            if r < self.rows - 1 && c < self.columns - 1 {
                let i = ((r + 1) * self.columns) + (c + 1);  
                if self.map[i].val < 9 && self.map[i].flag == CellTag::UNSELECTED {
                    self.set_selected(i);
                    cells.push((r + 1, c + 1));
                }
            }
        }    
        
    }

    fn check_looser_map(&mut self) {
        //set all unselected cells to selected
        for cell in self.map.iter_mut() {
            if cell.flag == CellTag::UNSELECTED {
                cell.flag = CellTag::SELECTED;
            }
        }
    }
}

impl eframe::App for AppGui {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        //display the welcome window
        if self.welcome {
            egui::Window::new("RustyMines")
                .show(ctx, |ui| {                    

                    ui.label("Settings:");
                    
                    ui.add_sized(
                        egui::Vec2::new(250., 30.),
                        egui::Slider::new(&mut self.s_rows, 4..=40)
                            .text("number of rows"));
                    
                    ui.add_sized(
                        egui::Vec2::new(250., 30.),
                        egui::Slider::new(&mut self.s_columns, 4..=40)
                            .text("number of columns"));
                    
                    ui.add_sized(
                        egui::Vec2::new(250., 30.),
                        egui::Slider::new(&mut self.n_mines, 1..=400)
                            .text("number of mines"));
                    
                    ui.horizontal(|ui| { 
                        if ui.button("OK").clicked() {
                            self.welcome = false;
                            //make sure m_mines isn't bigger than the map size
                            let len = self.s_rows * self.s_columns;
                            if self.n_mines >= len {
                                self.n_mines= len - 1;
                            }
                            self.rows = self.s_rows;
                            self.columns = self.s_columns;
                            self.selected = 0;
                            self.f_mines = 0;
                            self.f_time = 0.0;
                            self.state = 0;
                            self.map = get_map_vec(self.rows, self.columns, self.n_mines);
                            self.s_time = SystemTime::now();
                        }

                        if ui.button("Defaults").clicked() {
                            self.n_mines = MINES.into();
                            self.s_rows = ROWS.into();
                            self.s_columns = COLS.into();
                        }
                    });
                });
        } //end if self.welcome

        //the main window
        egui::CentralPanel::default()
            .show(ctx, |ui| {
 
                //top widgets
                ui.columns(3, |columns| {
                    
                    columns[0].vertical_centered(|ui| {
                        ui.label(format!("Mines: {} of {}\nSelected {} of {}", 
                                self.f_mines, self.n_mines,
                                self.selected, self.map.len()));
                    });

                    columns[1].vertical_centered( |ui| {
                        ui.style_mut().text_styles.insert(
                            egui::TextStyle::Button,
                            egui::FontId::new(30.0, eframe::epaint::FontFamily::Proportional),
                        );
                        
                        ui.visuals_mut().override_text_color = Some(Color32::BROWN);   
                        let smile = match self.state {
                            1 => UTF8_WINNER,
                            2 => UTF8_LOOSER,
                            _ => UTF8_SMILE,
                        };

                        let lbl = String::from_utf8(smile.to_vec()).unwrap();

                        if ui.add_sized((40.,40.), 
                            egui::Button::new(lbl)).clicked() {
                            self.welcome = true;
                        }
                    });
                    
                    columns[2].vertical_centered( |ui| {
                        
                        ui.ctx().request_repaint_after(Duration::new(1, 0));
                        
                        if !self.welcome && self.state == 0 {
                            self.f_time = self.s_time.elapsed().unwrap().as_secs_f64();
                        }
                        ui.label(format!("Time: {:.0}", self.f_time));
                    });
                
                });
                
                //map widgets
                egui::ScrollArea::vertical().hscroll(true).show(ui, |ui| {

                    //disable if not playing
                    if self.welcome || self.state != 0 {
                        ui.disable();
                    }

                    for r in 0..self.rows {
                        ui.horizontal(|ui| {
                            for c in 0..self.columns {
                                
                                let ind = (r * self.columns) + c;
                                let val = self.map[ind].val;
                                let mut lbl = " ".to_string();
                                let mut enable = true;

                                if self.map[ind].flag == CellTag::FLAGGED {
                                    ui.visuals_mut().override_text_color = 
                                        Some(LATTE.red);   
                                                                                       
                                    //if cell is flagged display a utf8 flag
                                    let mut flag = UTF8_FLAG;
                                    if self.state > 0 {
                                        //if not playing
                                        if self.map[ind].val < 9 {
                                            ui.visuals_mut().override_text_color = 
                                                Some(LATTE.red);
                                            //display a utf8 cross if the flag
                                            // is not on a mine
                                            flag = UTF8_CROSS;
                                        
                                        } else {
                                            ui.visuals_mut().override_text_color = 
                                                Some(LATTE.green);
                                            
                                            //or a utf8 check if it is
                                            //flag = UTF8_CROSS;
                                        }
                                    }

                                    lbl = String::from_utf8(flag.to_vec()).unwrap();

                                } else if self.map[ind].flag == CellTag::SELECTED {
                                    ui.visuals_mut().override_text_color = 
                                            match val {
                                                1 => Some(LATTE.blue),
                                                2 => Some(LATTE.green),
                                                3 => Some(LATTE.mauve),
                                                4 => Some(LATTE.maroon),
                                                5 => Some(LATTE.sapphire),
                                                6 => Some(LATTE.flamingo),
                                                7 => Some(LATTE.lavender),
                                                _ => Some(LATTE.text),
                                            };
                                
                                    //if it is selected
                                    //disable the widget (button)
                                    enable = false;

                                    if val > 0 && val < 9 {
                                        //if it's not a bomb and it has a value
                                        //show it
                                        lbl = format!("{}", val);
                                    } else if val == 9 {
                                        //it's a bomb
                                        lbl = String::from_utf8(UTF8_BOMB.to_vec()).unwrap();
                                    }
                                } else if self.map[ind].flag == CellTag::DEAD {
                                   
                                    ui.visuals_mut().override_text_color = 
                                        Some(LATTE.red);

                                    lbl = String::from_utf8(UTF8_DEAD.to_vec()).unwrap();
                                }

                                let button = egui::Button::new(lbl);
                                                                  
                                ui.add_enabled_ui(enable, |ui| {

                                    let response = ui.add_sized(
                                        [CELL_SIZE, CELL_SIZE], 
                                        button);
                                    
                                    if response.clicked() {
                                        if self.map[ind].flag == CellTag::FLAGGED {
                                            self.f_mines -= 1;
                                        }
                                        
                                        self.set_selected(ind);
                                        
                                        if self.map[ind].val == 0 {
                                            self.clean_neighbour_cells(r, c);
                                        }
                                        
                                        if self.map[ind].val == 9 {
                                            self.state = 2; //looser
                                            self.map[ind].flag = CellTag::DEAD;
                                            self.check_looser_map();
                                        } else if (self.selected + self.f_mines) >= self.map.len() {
                                            self.state = 1; //winner
                                        }
                                    };

                                    if response.secondary_clicked() {
                                        
                                        if self.map[ind].flag == CellTag::FLAGGED {
                                            self.map[ind].flag = CellTag::UNSELECTED;
                                            self.f_mines -= 1;
                                        } else if self.map[ind].flag == CellTag::UNSELECTED
                                                && self.f_mines < self.n_mines {
                                            self.map[ind].flag = CellTag::FLAGGED;
                                            self.f_mines += 1;
                                        }

                                        if (self.selected + self.f_mines) >= self.map.len() {
                                            self.state = 1; //winner
                                        }
                                    }
                                });
                            }//for columns
                        });//horizontal
                    }//for rows
                });//ScrollArea
            });//CentralPanel
    }
}
