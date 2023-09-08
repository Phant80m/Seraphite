use std::io::{stdout, Write};
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::*;
fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(90);
    area
}
use super::reader::Pages;
fn view_area2() -> Area {
    let mut area = Area::new(0, 0, 25, 100);
    let scroll: usize = 2;
    let content: usize = 200;
    area.scrollbar(scroll, content);
    area.pad_for_max_width(30);
    area
}
fn clear_and_redraw<W: Write>(w: &mut W, view: &MadView) -> Result<(), Error> {
    queue!(w, Clear(ClearType::All))?;
    view.write_on(w)?;
    w.flush()?;
    Ok(())
}

fn run_app(skin: MadSkin) -> Result<(), Error> {
    let mut pages = Pages::construct();
    let mut w = stdout();
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?;

    let mut view = MadView::from(pages.current_page_content(), view_area(), skin.clone());

    loop {
        let selected_page_index = pages.current_page;
        let pages_list: String = pages
            .pages
            .iter()
            .enumerate()
            .map(|(index, page)| {
                let mut page_line = if index == selected_page_index {
                    format!(
                        "{}> {}\n",
                        "\x1b[35m",
                        page.lines().next().unwrap_or_default()
                    )
                } else {
                    format!("{}\n", page.lines().next().unwrap_or_default())
                };
                // Reset the text color after each line
                page_line.push_str("\x1b[0m");
                page_line
            })
            .collect();
        let view2 = MadView::from(pages_list, view_area2(), skin.clone());
        view.write_on(&mut w)?;
        view2.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                Left => {
                    if pages.current_page > 0 {
                        pages.current_page -= 1;
                        view =
                            MadView::from(pages.current_page_content(), view_area(), skin.clone());
                    }
                }
                Right => {
                    if pages.current_page < pages.pages.len() - 1 {
                        pages.current_page += 1;
                        view =
                            MadView::from(pages.current_page_content(), view_area(), skin.clone());
                    }
                }
                Char('R') => {
                    clear_and_redraw(&mut w, &view)?;
                }
                Char('x') | Char('q') => {
                    break;
                }
                _ => {}
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    queue!(w, Show)?;
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.paragraph.align = Alignment::Left;
    skin.code_block.align = Alignment::Center;
    skin.code_block.set_fg(Blue);
    skin.inline_code.set_fg(Blue);
    skin.inline_code.set_bg(Black);
    skin.code_block.set_bg(Black);
    // skin.set_bg(rgb(36, 36, 36));
    // skin.set_headers_fg(rgb(127, 255, 212));
    // skin.scrollbar.thumb.set_fg(rgb(127, 255, 212));
    skin.set_headers_fg(Magenta);
    skin.bold.set_fg(White);
    skin.paragraph.set_fg(White);
    skin.italic.set_fg(rgb(205, 210, 215));
    skin.scrollbar.thumb.set_fg(Magenta);
    skin
}

pub fn doc() -> Result<(), Error> {
    let skin = make_skin();
    run_app(skin)
}
