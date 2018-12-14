use imgui::StyleVar::*;
use imgui::*;
use std::time::Duration;

use crate::command::CommandBuffer;
use crate::sheet::{Animation, AnimationFrame};
use crate::state::{self, Document, Selection, State};
use crate::ui::Rect;

fn draw_timeline_ticks<'a>(
    ui: &Ui<'a>,
    state: &State,
    commands: &mut CommandBuffer,
    document: &Document,
) {
    if let Ok(zoom) = state.get_timeline_zoom_factor() {
        let h = 8.0; // TODO DPI?
        let padding = 4.0; // TODO DPI?

        let draw_list = ui.get_window_draw_list();
        let cursor_start = ui.get_cursor_screen_pos();
        let max_draw_x = cursor_start.0 + ui.get_content_region_avail().0
            - ui.get_window_content_region_min().0
            + 2.0 * ui.get_cursor_pos().0;

        let mut x = cursor_start.0;
        let mut delta_t = 0;
        while x < max_draw_x {
            let (color, tick_height) = if delta_t % 100 == 0 {
                ([70.0 / 255.0, 70.0 / 255.0, 70.0 / 255.0], h) // TODO.style
            } else {
                ([20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0], h / 2.0) // TODO.style
            };

            draw_list.add_rect_filled_multicolor(
                (x, cursor_start.1),
                (x + 1.0, cursor_start.1 + tick_height),
                color,
                color,
                color,
                color,
            );

            delta_t += 10;
            x = cursor_start.0 + delta_t as f32 * zoom;
        }

        let clicked = ui.invisible_button(
            im_str!("timeline_ticks"),
            (max_draw_x - cursor_start.0, h + padding),
        );
        if ui.is_item_hovered()
            && ui.imgui().is_mouse_down(ImMouseButton::Left)
            && !ui.imgui().is_mouse_dragging(ImMouseButton::Left)
        {
            commands.begin_scrub();
        }
        let is_scrubbing = document.is_scrubbing();
        if clicked || is_scrubbing {
            let mouse_pos = ui.imgui().mouse_pos();
            let delta = mouse_pos.0 - cursor_start.0;
            let new_t = delta / zoom;
            commands.update_scrub(Duration::from_millis(std::cmp::max(0, new_t as i64) as u64));
        }

        ui.set_cursor_screen_pos((cursor_start.0, cursor_start.1 + h + padding));
    }
}

fn draw_insert_marker<'a>(ui: &Ui<'a>, draw_list: &WindowDrawList, height: f32) {
    let position = ui.get_cursor_screen_pos();
    let insert_marker_size = 8.0; // TODO DPI?
    let insert_marker_color = [249.0 / 255.0, 40.0 / 255.0, 50.0 / 255.0];
    let marker_top_left = (position.0 - insert_marker_size / 2.0, position.1);
    let marker_bottom_right = (position.0 + insert_marker_size / 2.0, position.1 + height);
    draw_list.add_rect_filled_multicolor(
        marker_top_left,
        marker_bottom_right,
        insert_marker_color,
        insert_marker_color,
        insert_marker_color,
        insert_marker_color,
    );
}

fn draw_animation_frame<'a>(
    ui: &Ui<'a>,
    state: &State,
    commands: &mut CommandBuffer,
    document: &Document,
    animation: &Animation,
    animation_frame_index: usize,
    animation_frame: &AnimationFrame,
    frame_starts_at: Duration,
    hovered: &mut bool,
) {
    let zoom = state.get_timeline_zoom_factor().unwrap_or(1.0);
    let w = animation_frame.get_duration() as f32 * zoom;
    let h = 20.0; // TODO DPI?
    let outline_size = 1.0; // TODO DPI?
    let text_padding = 4.0; // TODO DPI?
    let resize_handle_size = 16.0; // TODO DPI?
    let is_selected = document.get_selection()
        == &Some(Selection::AnimationFrame(
            animation.get_name().to_string(),
            animation_frame_index,
        ));

    // TODO what happens when things get tiny?

    let draw_list = ui.get_window_draw_list();
    let mut cursor_pos = ui.get_cursor_screen_pos();
    cursor_pos.0 += frame_starts_at.as_millis() as f32 * zoom;

    // Draw outline
    let top_left = cursor_pos;
    let bottom_right = (top_left.0 + w, top_left.1 + h);
    let outline_color = [25.0 / 255.0, 15.0 / 255.0, 0.0 / 255.0]; // TODO.style
    draw_list.add_rect_filled_multicolor(
        top_left,
        bottom_right,
        outline_color,
        outline_color,
        outline_color,
        outline_color,
    );

    // Draw fill
    let mut fill_top_left = top_left;
    let mut fill_bottom_right = bottom_right;
    fill_top_left.0 += outline_size;
    fill_top_left.1 += outline_size;
    fill_bottom_right.0 -= outline_size;
    fill_bottom_right.1 -= outline_size;
    let fill_color = if is_selected {
        [249.0 / 255.0, 212.0 / 255.0, 200.0 / 255.0] // TODO.style
    } else {
        [249.0 / 255.0, 212.0 / 255.0, 35.0 / 255.0] // TODO.style
    };
    draw_list.add_rect_filled_multicolor(
        fill_top_left,
        fill_bottom_right,
        fill_color,
        fill_color,
        fill_color,
        fill_color,
    );

    // Draw name
    if let Some(name) = animation_frame.get_frame().file_name() {
        let text_color = outline_color; // TODO.style
        let text_position = (fill_top_left.0 + text_padding, fill_top_left.1);
        draw_list.add_text(text_position, text_color, name.to_string_lossy());
    }

    // Click interactions
    {
        let id = format!("frame_button_{}", top_left.0);
        ui.set_cursor_screen_pos((top_left.0 + resize_handle_size / 2.0, top_left.1));
        if ui.invisible_button(
            &ImString::new(id),
            (
                bottom_right.0 - top_left.0 - resize_handle_size,
                bottom_right.1 - top_left.1,
            ),
        ) {
            commands.select_animation_frame(animation_frame_index);
        }
    }

    // Drag and drop interactions
    {
        let mouse_pos = ui.imgui().mouse_pos();
        let is_hovering_frame = mouse_pos.0 >= top_left.0 && mouse_pos.0 <= bottom_right.0;
        let is_window_hovered =
            ui.is_window_hovered_with_flags(ImGuiHoveredFlags::AllowWhenBlockedByActiveItem);
        if is_hovering_frame && is_window_hovered {
            *hovered = true;

            let is_mouse_down = ui.imgui().is_mouse_down(ImMouseButton::Left);
            let is_mouse_dragging = ui.imgui().is_mouse_dragging(ImMouseButton::Left);
            let dragging_frame = document.get_content_frame_being_dragged().is_some();
            let dragging_animation_frame = document.get_timeline_frame_being_dragged().is_some();

            if dragging_frame || dragging_animation_frame {
                if is_mouse_dragging {
                    ui.set_cursor_screen_pos(top_left);
                    draw_insert_marker(ui, &draw_list, h);
                }
                if !is_mouse_down {
                    if let Some(dragged_frame) = document.get_content_frame_being_dragged() {
                        commands.insert_animation_frame_before(
                            dragged_frame,
                            animation_frame_index,
                        );
                    } else if let Some(dragged_animation_frame) =
                        document.get_timeline_frame_being_dragged()
                    {
                        commands.reorder_animation_frame(
                            *dragged_animation_frame,
                            animation_frame_index,
                        );
                    }
                }
            } else if is_mouse_down && !is_mouse_dragging {
                commands.begin_animation_frame_drag(animation_frame_index);
            }
        }
    }

    // Drag to resize interaction
    {
        let id = format!("frame_handle_{}", top_left.0);
        ui.set_cursor_screen_pos((bottom_right.0 - resize_handle_size / 2.0, top_left.1));
        ui.invisible_button(&ImString::new(id), (resize_handle_size, h));

        let is_mouse_dragging = ui.imgui().is_mouse_dragging(ImMouseButton::Left);
        let is_mouse_down = ui.imgui().is_mouse_down(ImMouseButton::Left);
        match document.get_timeline_frame_being_scaled() {
            None => {
                if ui.is_item_hovered() {
                    ui.imgui().set_mouse_cursor(ImGuiMouseCursor::ResizeEW);
                    if is_mouse_down && !is_mouse_dragging {
                        commands.begin_animation_frame_duration_drag(animation_frame_index);
                    }
                }
            }
            Some(i) if *i == animation_frame_index => {
                ui.imgui().set_mouse_cursor(ImGuiMouseCursor::ResizeEW);
                if is_mouse_dragging {
                    let mouse_pos = ui.imgui().mouse_pos();
                    let new_width = mouse_pos.0 - top_left.0;
                    let new_duration = std::cmp::max((new_width / zoom).ceil() as i32, 1) as u32;
                    commands.update_animation_frame_duration_drag(new_duration);
                }
            }
            _ => (),
        };
    }

    ui.set_cursor_screen_pos(bottom_right);
}

fn draw_playback_head<'a>(ui: &Ui<'a>, state: &State, document: &Document, animation: &Animation) {
    let duration = animation.get_duration().unwrap_or(0);

    let now_ms = {
        let now = document.get_timeline_clock();
        let ms = now.as_millis();
        std::cmp::min(ms, duration.into()) as u32
    };

    let zoom = state.get_timeline_zoom_factor().unwrap_or(1.0);
    let draw_list = ui.get_window_draw_list();

    let mut cursor_pos = ui.get_cursor_screen_pos();
    cursor_pos.0 += now_ms as f32 * zoom;
    let space = ui.get_content_region_avail();

    let fill_color = [255.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0]; // TODO constants

    draw_list.add_rect_filled_multicolor(
        (cursor_pos.0, cursor_pos.1),
        (cursor_pos.0 + 1.0, cursor_pos.1 + space.1),
        fill_color,
        fill_color,
        fill_color,
        fill_color,
    );
}

pub fn draw<'a>(ui: &Ui<'a>, rect: &Rect, state: &State, commands: &mut CommandBuffer) {
    ui.with_style_vars(&vec![WindowRounding(0.0), WindowBorderSize(0.0)], || {
        ui.window(im_str!("Timeline"))
            .position(rect.position, ImGuiCond::Always)
            .size(rect.size, ImGuiCond::Always)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .always_horizontal_scrollbar(true)
            .build(|| {
                if let Some(document) = state.get_current_document() {
                    if let Some(state::WorkbenchItem::Animation(animation_name)) =
                        document.get_workbench_item()
                    {
                        if let Some(animation) = document.get_sheet().get_animation(animation_name)
                        {
                            if ui.small_button(im_str!("Play/Pause")) {
                                commands.toggle_playback();
                            }
                            ui.same_line(0.0);
                            let mut looping = animation.is_looping();
                            if ui.checkbox(im_str!("Loop"), &mut looping) {
                                commands.toggle_looping();
                            }

                            // TODO autoscroll during playback

                            let ticks_cursor_position = ui.get_cursor_pos();
                            draw_timeline_ticks(ui, state, commands, document);

                            let frames_start_cursor_position = ui.get_cursor_pos();
                            let mut frames_end_cursor_position = frames_start_cursor_position;
                            let mut cursor = Duration::new(0, 0);
                            let mut any_frame_hovered = false;
                            for (frame_index, animation_frame) in
                                animation.frames_iter().enumerate()
                            {
                                ui.set_cursor_pos(frames_start_cursor_position);
                                draw_animation_frame(
                                    ui,
                                    state,
                                    commands,
                                    document,
                                    animation,
                                    frame_index,
                                    animation_frame,
                                    cursor,
                                    &mut any_frame_hovered,
                                );
                                frames_end_cursor_position = ui.get_cursor_pos();
                                cursor +=
                                    Duration::from_millis(animation_frame.get_duration() as u64);
                            }

                            ui.set_cursor_pos(ticks_cursor_position);
                            draw_playback_head(ui, state, document, animation);

                            let is_window_hovered = ui.is_window_hovered_with_flags(
                                ImGuiHoveredFlags::AllowWhenBlockedByActiveItem,
                            );
                            let is_mouse_down = ui.imgui().is_mouse_down(ImMouseButton::Left);
                            let is_dragging = document.get_content_frame_being_dragged().is_some()
                                || document.get_timeline_frame_being_dragged().is_some();
                            if is_window_hovered && is_dragging && !any_frame_hovered {
                                ui.set_cursor_pos((
                                    frames_end_cursor_position.0,
                                    frames_start_cursor_position.1,
                                ));
                                draw_insert_marker(
                                    ui,
                                    &ui.get_window_draw_list(),
                                    frames_end_cursor_position.1 - frames_start_cursor_position.1,
                                );
                                if !is_mouse_down {
                                    if let Some(frame) = document.get_content_frame_being_dragged()
                                    {
                                        // TODO allow dropping frame on workbench
                                        commands.create_animation_frame(frame);
                                    } else if let Some(dragged_animation_frame) =
                                        document.get_timeline_frame_being_dragged()
                                    {
                                        commands.reorder_animation_frame(
                                            *dragged_animation_frame,
                                            animation.get_num_frames(),
                                        );
                                    }
                                }
                            }

                            if ui.is_window_hovered() {
                                if ui.imgui().key_ctrl() {
                                    let mouse_wheel = ui.imgui().mouse_wheel();
                                    if mouse_wheel > 0.0 {
                                        commands.timeline_zoom_in();
                                    } else if mouse_wheel < 0.0 {
                                        commands.timeline_zoom_out();
                                    }
                                }
                            }
                        }
                    }
                }
            });
    });
}
