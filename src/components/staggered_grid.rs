use makepad_widgets::{scroll_bar::ScrollBarAction, *};
use std::collections::{HashMap, VecDeque};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub ElementStaggeredGrid = {{ElementStaggeredGrid}} {
        width: Fill
        height: Fill
        capture_overload: true
        scroll_bar: <ScrollBar> {}
        flow: Down
    }
}

#[derive(Live, Widget)]
pub struct ElementStaggeredGrid {
    #[redraw]
    #[rust]
    area: Area,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[rust]
    range_start: usize,
    #[rust(usize::MAX)]
    range_end: usize,

    #[live(0.2)]
    flick_scroll_minimum: f64,
    #[live(80.0)]
    flick_scroll_maximum: f64,
    #[live(0.005)]
    flick_scroll_scaling: f64,
    #[live(0.98)]
    flick_scroll_decay: f64,

    #[live(100.0)]
    max_pull_down: f64,

    #[live(true)]
    align_top_when_empty: bool,
    #[live(false)]
    grab_key_focus: bool,
    #[live(true)]
    drag_scrolling: bool,
    #[live(false)]
    auto_tail: bool,
    #[rust(false)]
    tail_range: bool,
    #[live]
    capture_overload: bool,
    #[live(false)]
    keep_invisible: bool,

    #[live]
    scroll_bar: ScrollBar,

    #[rust(Vec2Index::X)]
    vec_index: Vec2Index,

    #[rust]
    first_visible_item: usize,
    #[rust]
    scrolled_offset: f64,

    #[rust]
    draw_state: DrawStateWrap<ListDrawState>,
    #[rust]
    draw_align_list: Vec<AlignItem>,
    #[rust]
    detect_tail_in_draw: bool,

    #[rust]
    templates: ComponentMap<LiveId, LivePtr>,
    #[rust]
    items: ComponentMap<(usize, LiveId), WidgetRef>,
    #[rust(ScrollState::Stopped)]
    scroll_state: ScrollState,
    #[live]
    columns_number: usize,
    #[live]
    column_spacing: f64,
    #[rust(1usize)]
    last_drawn_column: usize,

    #[rust]
    columns: Vec<Column>,

    #[rust]
    last_drawn_item_index: usize,
    #[rust]
    item_columns: HashMap<usize, usize>,

    #[rust]
    most_recent_viewport: Rect,

    #[rust]
    currently_visible_items: Vec<usize>,

    #[rust]
    items_usage_order: VecDeque<(usize, LiveId)>,

    #[rust]
    repurpose_inactive_widgets: bool,

    #[rust]
    max_active_widgets: usize,
}

#[derive(Default, Clone, Debug)]
struct Column {
    pub last_item_index: usize,
    pub height: f64,
    pub exceeds_viewport: bool,
    pub items: Vec<ColumnItem>,
    pub is_dirty: bool,
    pub first_visible_item: usize,
    pub first_item_offset: f64,
    pub has_drawn_first_item: bool,
}

#[derive(Default, Clone, Debug)]
struct ColumnItem {
    pub index: usize,
    pub size: DVec2,
    pub is_visible: bool,
}

struct AlignItem {
    size: DVec2,
    index: usize,
}

#[derive(Clone, Copy)]
struct ScrollSample {
    abs: f64,
    time: f64,
}

enum ScrollState {
    Stopped,
    Drag { samples: Vec<ScrollSample> },
    Flick { delta: f64, next_frame: NextFrame },
    Pulldown { next_frame: NextFrame },
}

#[derive(Clone)]
enum ListDrawState {
    Begin,
    Down { index: usize, pos: f64, viewport: Rect },
    Up { index: usize, pos: f64, hit_bottom: bool, viewport: Rect },
    DownAgain { index: usize, pos: f64, viewport: Rect },
    End,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ElementStaggeredGridAction {
    Scroll,
    None,
}

impl ListDrawState {
    fn is_down_again(&self) -> bool {
        matches!(self, Self::DownAgain { .. })
    }
}

impl LiveHook for ElementStaggeredGrid {
    fn before_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            self.templates.clear();
        }
    }

    fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::NewFromDoc { file_id } | ApplyFrom::UpdateFromDoc { file_id } => {
                self.max_active_widgets = usize::MAX;
                if nodes[index].origin.has_prop_type(LivePropType::Instance) {
                    let live_ptr = cx.live_registry.borrow().file_id_index_to_live_ptr(file_id, index);
                    self.templates.insert(id, live_ptr);
                    for ((_, templ_id), node) in self.items.iter_mut() {
                        if *templ_id == id {
                            node.apply(cx, apply, index, nodes);
                        }
                    }
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                }
            }
            _ => (),
        }
        nodes.skip_node(index)
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if let Flow::Down = self.layout.flow {
            self.vec_index = Vec2Index::Y
        } else {
            self.vec_index = Vec2Index::X
        }
        if self.auto_tail {
            self.tail_range = true;
        }
    }
}

#[derive(PartialEq)]
pub enum WidgetAllocationStatus {
    Created,
    Repurposed,
    Retained,
}

impl ElementStaggeredGrid {
    fn begin(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, self.layout);

        self.currently_visible_items.clear();
        if self.columns.len() != self.columns_number {
            self.item_columns.clear();
            self.columns = vec![Column::default(); self.columns_number];
        } else {
            for column in &mut self.columns {
                column.last_item_index = 0;
                column.exceeds_viewport = false;
                column.has_drawn_first_item = false;
                column.is_dirty = true;
            }
        }

        self.clean_up_old_items();
        self.draw_align_list.clear();
    }

    fn clean_up_old_items(&mut self) {
        let range_end = self.range_end;
        for column in &mut self.columns {
            column.items.retain(|item| item.index < range_end);
        }
    }

    fn end(&mut self, cx: &mut Cx2d) {
        if !matches!(self.draw_state.get(), Some(ListDrawState::End)) {
            error!("Draw state not at end in ElementStaggeredGrid, please review your next_visible_item loop");
        }

        self.max_active_widgets = self.currently_visible_items.len() * 2;
        cx.end_turtle_with_area(&mut self.area);
    }

    pub fn next_visible_item(&mut self, cx: &mut Cx2d) -> Option<usize> {
        let vi = self.vec_index;
        if let Some(draw_state) = self.draw_state.get() {
            match draw_state {
                ListDrawState::Begin => {
                    let viewport = cx.turtle().rect();
                    self.most_recent_viewport = viewport;

                    let mut first_visible_column = 0;
                    for (i, column) in self.columns.iter_mut().enumerate() {
                        column.height = viewport.pos.y + column.first_item_offset;
                        if column.first_visible_item == self.first_visible_item
                            && self.first_visible_item != 0
                        {
                            first_visible_column = i;
                        }
                    }

                    self.draw_state.set(ListDrawState::Down {
                        index: self.first_visible_item,
                        pos: self.columns[first_visible_column].first_item_offset,
                        viewport,
                    });

                    let column_width = self.column_width(viewport);
                    let abs_pos = dvec2(
                        viewport.pos.x + (first_visible_column as f64 * column_width),
                        viewport.pos.y + self.columns[first_visible_column].first_item_offset,
                    );

                    self.last_drawn_item_index = self.first_visible_item;
                    self.last_drawn_column = first_visible_column;
                    self.columns[first_visible_column].has_drawn_first_item = true;

                    cx.begin_turtle(
                        Walk {
                            abs_pos: Some(abs_pos),
                            margin: Default::default(), metrics: Default::default(),
                            width: Size::Fixed(column_width),
                            height: Size::fit(),
                        },
                        self.layout_with_spacing(),
                    );

                    self.add_to_visibles_list(self.first_visible_item);
                    return Some(self.first_visible_item);
                }
                ListDrawState::Down { index, pos, viewport }
                | ListDrawState::DownAgain { index, pos, viewport } => {
                    let is_down_again = draw_state.is_down_again();
                    let did_draw = cx.turtle_has_align_items();
                    let prev_item_rect = cx.end_turtle();
                    self.draw_align_list.push(AlignItem {
                        size: prev_item_rect.size,
                        index,
                    });

                    self.record_previous_column(index, prev_item_rect, viewport, vi);
                    let next_index = index + 1;
                    let mut valid_next_index = next_index;

                    let mut current_column = self.find_column_for_item(next_index);

                    if !self.columns[current_column].has_drawn_first_item {
                        let current_col_already_has_this_item = self.columns[current_column]
                            .items
                            .iter()
                            .any(|item| item.index == next_index);
                        let current_col_first_visible_is_this_item =
                            self.columns[current_column].first_visible_item == next_index;
                        let current_col_first_visible_zero =
                            self.columns[current_column].first_visible_item == 0;

                        if current_col_already_has_this_item
                            && !current_col_first_visible_is_this_item
                            && !current_col_first_visible_zero
                        {
                            while valid_next_index < self.range_end {
                                valid_next_index += 1;
                                current_column = self.find_column_for_item(valid_next_index);
                                let item = self.columns[current_column]
                                    .items
                                    .iter()
                                    .find(|item| item.index == valid_next_index);
                                if let Some(item) = item {
                                    if item.is_visible {
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    let index_is_older = index < self.last_drawn_item_index;
                    let exceeds_viewport = self.columns[current_column].exceeds_viewport;
                    let index_exceeds_range = index >= self.range_end;
                    if !did_draw || exceeds_viewport || index_exceeds_range || index_is_older {
                        if self.first_visible_item > 0 && !is_down_again {
                            self.draw_state.set(ListDrawState::Up {
                                index: self.first_visible_item - 1,
                                pos: self.scrolled_offset,
                                hit_bottom: index >= self.range_end,
                                viewport,
                            });
                            cx.begin_turtle(
                                Walk {
                                    abs_pos: Some(dvec2(viewport.pos.x, viewport.pos.y)),
                                    margin: Default::default(), metrics: Default::default(),
                                    width: Size::Fixed(self.column_width(viewport)),
                                    height: Size::fit(),
                                },
                                self.layout_with_spacing(),
                            );

                            self.add_to_visibles_list(self.first_visible_item);
                            return Some(self.first_visible_item);
                        } else {
                            self.draw_state.set(ListDrawState::End);
                            return None;
                        }
                    }
                    if is_down_again {
                        self.draw_state.set(ListDrawState::DownAgain {
                            index: valid_next_index,
                            pos: pos + prev_item_rect.size.index(vi),
                            viewport,
                        });
                    } else {
                        self.draw_state.set(ListDrawState::Down {
                            index: valid_next_index,
                            pos: pos + prev_item_rect.size.index(vi),
                            viewport,
                        });
                    }

                    let column_width = self.column_width(viewport);
                    let new_item_abs_pos = dvec2(
                        viewport.pos.x + (current_column as f64 * column_width),
                        self.columns[current_column].height,
                    );

                    self.last_drawn_column = current_column;
                    self.item_columns.insert(valid_next_index, current_column);
                    self.last_drawn_item_index = index;
                    self.columns[current_column].has_drawn_first_item = true;

                    cx.begin_turtle(
                        Walk {
                            abs_pos: Some(new_item_abs_pos),
                            margin: Default::default(), metrics: Default::default(),
                            width: Size::Fixed(column_width),
                            height: Size::fit(),
                        },
                        self.layout_with_spacing(),
                    );

                    self.add_to_visibles_list(valid_next_index);
                    return Some(valid_next_index);
                }
                ListDrawState::Up { index, pos, hit_bottom, viewport } => {
                    let did_draw = cx.turtle_has_align_items();
                    let rect = cx.end_turtle();
                    self.draw_align_list.push(AlignItem {
                        size: rect.size,
                        index,
                    });

                    if index == self.range_start {
                        if pos - rect.size.index(vi) > 0.0 {
                            if let Some(last_index) =
                                self.draw_align_list.iter().map(|v| v.index).max()
                            {
                                let total_height: f64 =
                                    self.draw_align_list.iter().map(|v| v.size.index(vi)).sum();
                                self.draw_state.set(ListDrawState::DownAgain {
                                    index: last_index + 1,
                                    pos: total_height,
                                    viewport,
                                });
                                self.last_drawn_column = 0;
                                cx.begin_turtle(
                                    Walk {
                                        abs_pos: Some(dvec2(
                                            viewport.pos.x,
                                            viewport.pos.y + total_height,
                                        )),
                                        margin: Default::default(), metrics: Default::default(),
                                        width: Size::Fixed(self.column_width(viewport)),
                                        height: Size::fit(),
                                    },
                                    self.layout_with_spacing(),
                                );

                                self.add_to_visibles_list(last_index + 1);
                                return Some(last_index + 1);
                            }
                        }
                        self.draw_state.set(ListDrawState::End);
                        return None;
                    }

                    if !did_draw
                        || pos < if hit_bottom {
                            -viewport.size.index(vi)
                        } else {
                            0.0
                        }
                    {
                        self.draw_state.set(ListDrawState::End);
                        return None;
                    }

                    self.draw_state.set(ListDrawState::Up {
                        index: index - 1,
                        hit_bottom,
                        pos: pos - rect.size.index(vi),
                        viewport,
                    });
                    cx.begin_turtle(
                        Walk {
                            abs_pos: Some(dvec2(viewport.pos.x, viewport.pos.y)),
                            margin: Default::default(), metrics: Default::default(),
                            width: Size::Fixed(self.column_width(viewport)),
                            height: Size::fit(),
                        },
                        self.layout_with_spacing(),
                    );

                    self.add_to_visibles_list(index - 1);
                    return Some(index - 1);
                }
                _ => (),
            }
        }
        None
    }

    fn add_to_visibles_list(&mut self, index: usize) {
        if self.currently_visible_items.iter().all(|&i| i != index) {
            self.currently_visible_items.push(index);
        }
    }

    pub fn item(
        &mut self,
        cx: &mut Cx,
        entry_id: usize,
        template: LiveId,
    ) -> Option<(WidgetRef, WidgetAllocationStatus)> {
        if let Some(ptr) = self.templates.get(&template) {
            let allocation_status;

            if let Some(entry) = self.items.get(&(entry_id, template)) {
                allocation_status = WidgetAllocationStatus::Retained;
                self.items_usage_order
                    .retain(|&k| k != (entry_id, template));
                self.items_usage_order.push_front((entry_id, template));
                return Some((entry.clone(), allocation_status));
            }

            let widget =
                if self.repurpose_inactive_widgets && self.items.len() >= self.max_active_widgets {
                    if let Some(oldest_key) = self
                        .items_usage_order
                        .iter()
                        .rev()
                        .find(|&&(_, t)| t == template)
                        .cloned()
                    {
                        allocation_status = WidgetAllocationStatus::Repurposed;
                        self.items_usage_order.retain(|&k| k != oldest_key);
                        self.items.remove(&oldest_key).unwrap()
                    } else {
                        allocation_status = WidgetAllocationStatus::Created;
                        WidgetRef::new_from_ptr(cx, Some(*ptr))
                    }
                } else {
                    allocation_status = WidgetAllocationStatus::Created;
                    WidgetRef::new_from_ptr(cx, Some(*ptr))
                };

            self.items
                .insert((entry_id, template), widget.clone());
            self.items_usage_order.push_front((entry_id, template));

            Some((widget, allocation_status))
        } else {
            warning!("Template not found: {template}. Did you add it to the <ElementStaggeredGrid> instance?");
            None
        }
    }

    pub fn set_item_range(&mut self, cx: &mut Cx, range_start: usize, range_end: usize) {
        self.range_start = range_start;
        if self.range_end != range_end {
            self.range_end = range_end;
            if self.tail_range {
                self.first_visible_item = self.range_end.max(1) - 1;
                self.scrolled_offset = 0.0;
            }
            self.update_scroll_bar(cx);
        }
    }

    pub fn set_columns_number(&mut self, cx: &mut Cx, columns_number: usize) {
        if self.columns_number != columns_number {
            self.columns_number = columns_number;
            self.currently_visible_items.clear();
            self.item_columns.clear();
            self.columns = vec![Column::default(); self.columns_number];
            self.clean_up_old_items();
            self.draw_align_list.clear();
            self.scroll_state = ScrollState::Stopped;
            self.delta_top_scroll(cx, 0.0, true);
        }
    }

    pub fn reset_and_scroll_top(&mut self, cx: &mut Cx) {
        self.currently_visible_items.clear();
        self.item_columns.clear();
        self.columns = vec![Column::default(); self.columns_number];
        self.clean_up_old_items();
        self.draw_align_list.clear();
        self.scroll_state = ScrollState::Stopped;
        self.delta_top_scroll(cx, 0.0, true);
    }

    pub fn update_scroll_bar(&mut self, cx: &mut Cx) {
        let max_scroll = self.calculate_max_scroll();
        let scroll_fraction = if max_scroll > 0.0 {
            self.scrolled_offset / max_scroll
        } else {
            0.0
        };
        let scroll_pos = scroll_fraction * self.scroll_bar.get_scroll_view_total();
        self.scroll_bar.set_scroll_pos_no_action(cx, scroll_pos);
    }

    fn delta_top_scroll(&mut self, cx: &mut Cx, delta: f64, clip_top: bool) {
        self.scrolled_offset += delta;
        let max_scroll = self.calculate_max_scroll();
        self.scrolled_offset = self.scrolled_offset.max(max_scroll);

        if self.first_visible_item == self.range_start {
            self.scrolled_offset = self.scrolled_offset.min(self.max_pull_down);
            if clip_top && self.scrolled_offset > 0.0 {
                self.scrolled_offset = 0.0;
            }
        }

        for column in self.columns.iter_mut() {
            let mut acc_height = self.scrolled_offset;
            let mut acc_items_height = 0.;
            column.first_visible_item = column
                .items
                .first()
                .map(|item| item.index)
                .unwrap_or(self.range_start);
            let first_column_item;
            if let Some(item) = column.items.first() {
                first_column_item = item.index;
            } else {
                continue;
            }

            for item in column.items.iter_mut() {
                let item_size = item.size.index(self.vec_index);
                acc_height += item_size;
                acc_items_height += item_size;
                if acc_height >= 0. {
                    column.first_visible_item = item.index;

                    if item.index == first_column_item {
                        column.first_item_offset = self.scrolled_offset;
                    } else {
                        column.first_item_offset =
                            (-(item_size - (acc_items_height + self.scrolled_offset))).min(0.0);
                    }

                    item.is_visible = true;
                    break;
                } else {
                    item.is_visible = false;
                }
            }
        }

        self.first_visible_item = self
            .columns
            .iter()
            .map(|col| col.first_visible_item)
            .min()
            .unwrap_or(self.range_start);

        self.update_scroll_bar(cx);
    }

    fn calculate_max_scroll(&self) -> f64 {
        let max_column_height = self
            .columns
            .iter()
            .map(|col| {
                col.items
                    .iter()
                    .map(|item| item.size.index(self.vec_index))
                    .sum::<f64>()
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let buffer = self.most_recent_viewport.size.index(self.vec_index) * 0.8;
        -((max_column_height - buffer).max(0.0))
    }

    fn record_previous_column(
        &mut self,
        index: usize,
        prev_item_rect: Rect,
        viewport: Rect,
        vi: Vec2Index,
    ) {
        let column = &mut self.columns[self.last_drawn_column];

        if let Some(item) = column.items.iter_mut().find(|item| item.index == index) {
            item.size = prev_item_rect.size;
        } else {
            column.items.push(ColumnItem {
                index,
                size: prev_item_rect.size,
                is_visible: true,
            });
        }

        column.last_item_index = index + 1;
        column.height += prev_item_rect.size.index(vi);
        column.exceeds_viewport = column.height - viewport.pos.y > viewport.size.index(vi);
        column.is_dirty = false;
    }

    fn find_column_for_item(&mut self, index: usize) -> usize {
        match self.item_columns.get(&index) {
            Some(column) => *column,
            None => {
                if let Some(col) = self.find_next_available_column() {
                    col
                } else {
                    self.columns
                        .iter()
                        .enumerate()
                        .filter(|(_, col)| col.is_dirty)
                        .min_by_key(|(_, col)| col.items.len())
                        .map(|(i, _)| i)
                        .unwrap_or_else(|| (self.last_drawn_column + 1) % self.columns.len())
                }
            }
        }
    }

    fn find_next_available_column(&self) -> Option<usize> {
        let start_index = (self.last_drawn_column + 1) % self.columns.len();
        let column_order = (start_index..self.columns.len()).chain(0..start_index);
        column_order
            .into_iter()
            .find(|&index| !self.columns[index].exceeds_viewport)
    }

    fn layout_with_spacing(&mut self) -> Layout {
        Layout::flow_down().with_padding(Padding {
            top: self.column_spacing,
            right: self.column_spacing,
            bottom: self.column_spacing,
            left: self.column_spacing,
        })
    }

    fn column_width(&self, viewport: Rect) -> f64 {
        viewport.size.x / self.columns.len() as f64
    }
}

impl Widget for ElementStaggeredGrid {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        let mut scroll_to = None;
        self.scroll_bar.handle_event_with(cx, event, &mut |_cx, action| {
            if let ScrollBarAction::Scroll {
                scroll_pos,
                view_total,
                view_visible,
            } = action
            {
                scroll_to = Some((scroll_pos, scroll_pos + 0.5 >= view_total - view_visible))
            }
        });
        if let Some((_scroll_to, at_end)) = scroll_to {
            if at_end && self.auto_tail {
                self.first_visible_item = self.range_end.max(1) - 1;
                self.scrolled_offset = 0.0;
                self.tail_range = true;
            } else if self.tail_range {
                self.tail_range = false;
            }

            self.first_visible_item = 0;
            self.scrolled_offset = 0.0;
            cx.widget_action(uid, &scope.path, ElementStaggeredGridAction::Scroll);
            self.area.redraw(cx);
        }

        for item in self.items.values_mut() {
            let item_uid = item.widget_uid();
            cx.group_widget_actions(uid, item_uid, |cx| {
                item.handle_event(cx, event, scope)
            });
        }

        match &mut self.scroll_state {
            ScrollState::Flick { delta, next_frame } => {
                if let Some(_) = next_frame.is_event(event) {
                    *delta = *delta * self.flick_scroll_decay;
                    if delta.abs() > self.flick_scroll_minimum {
                        *next_frame = cx.new_next_frame();
                        let delta = *delta;
                        self.delta_top_scroll(cx, delta, true);
                        cx.widget_action(uid, &scope.path, ElementStaggeredGridAction::Scroll);
                        self.area.redraw(cx);
                    } else {
                        self.scroll_state = ScrollState::Stopped;
                    }
                }
            }
            ScrollState::Pulldown { next_frame } => {
                if let Some(_) = next_frame.is_event(event) {
                    if self.first_visible_item == self.range_start && self.scrolled_offset > 0.0 {
                        self.scrolled_offset *= 0.9;
                        if self.scrolled_offset < 1.0 {
                            self.scrolled_offset = 0.0;
                        } else {
                            *next_frame = cx.new_next_frame();
                            cx.widget_action(
                                uid,
                                &scope.path,
                                ElementStaggeredGridAction::Scroll,
                            );
                        }
                        self.area.redraw(cx);
                    } else {
                        self.scroll_state = ScrollState::Stopped
                    }
                }
            }
            _ => (),
        }
        let vi = self.vec_index;
        let is_scroll = matches!(event, Event::Scroll(_));
        if self.scroll_bar.is_area_captured(cx) {
            self.scroll_state = ScrollState::Stopped;
        }
        if !self.scroll_bar.is_area_captured(cx) || is_scroll {
            match event.hits_with_capture_overload(cx, self.area, self.capture_overload) {
                Hit::FingerScroll(e) => {
                    if self.tail_range {
                        self.tail_range = false;
                    }
                    self.detect_tail_in_draw = true;
                    self.scroll_state = ScrollState::Stopped;
                    self.delta_top_scroll(cx, -e.scroll.index(vi), true);
                    cx.widget_action(uid, &scope.path, ElementStaggeredGridAction::Scroll);
                    self.area.redraw(cx);
                }
                Hit::FingerDown(e) => {
                    if self.grab_key_focus {
                        cx.set_key_focus(self.area);
                    }
                    if self.tail_range {
                        self.tail_range = false;
                    }
                    if self.drag_scrolling {
                        self.scroll_state = ScrollState::Drag {
                            samples: vec![ScrollSample {
                                abs: e.abs.index(vi),
                                time: e.time,
                            }],
                        };
                    }
                }
                Hit::FingerMove(e) => {
                    cx.set_cursor(MouseCursor::Default);
                    match &mut self.scroll_state {
                        ScrollState::Drag { samples } => {
                            let new_abs = e.abs.index(vi);
                            let old_sample = *samples.last().unwrap();
                            samples.push(ScrollSample {
                                abs: new_abs,
                                time: e.time,
                            });
                            if samples.len() > 4 {
                                samples.remove(0);
                            }
                            self.delta_top_scroll(cx, new_abs - old_sample.abs, false);
                            self.area.redraw(cx);
                        }
                        _ => (),
                    }
                }
                Hit::FingerUp(_e) => {
                    match &mut self.scroll_state {
                        ScrollState::Drag { samples } => {
                            let mut last = None;
                            let mut scaled_delta = 0.0;
                            let mut total_delta = 0.0;
                            for sample in samples.iter().rev() {
                                if last.is_none() {
                                    last = Some(sample);
                                } else {
                                    total_delta += last.unwrap().abs - sample.abs;
                                    scaled_delta += (last.unwrap().abs - sample.abs)
                                        / (last.unwrap().time - sample.time)
                                }
                            }
                            scaled_delta *= self.flick_scroll_scaling;
                            if self.first_visible_item == self.range_start
                                && self.scrolled_offset > 0.0
                            {
                                self.scroll_state = ScrollState::Pulldown {
                                    next_frame: cx.new_next_frame(),
                                };
                            } else if total_delta.abs() > 10.0
                                && scaled_delta.abs() > self.flick_scroll_minimum
                            {
                                self.scroll_state = ScrollState::Flick {
                                    delta: scaled_delta
                                        .min(self.flick_scroll_maximum)
                                        .max(-self.flick_scroll_maximum),
                                    next_frame: cx.new_next_frame(),
                                };
                            } else {
                                self.scroll_state = ScrollState::Stopped;
                            }
                        }
                        _ => (),
                    }
                }
                Hit::KeyFocus(_) => {}
                Hit::KeyFocusLost(_) => {}
                _ => (),
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.draw_state.begin(cx, ListDrawState::Begin) {
            self.begin(cx, walk);
            return DrawStep::make_step();
        }

        if let Some(_) = self.draw_state.get() {
            self.end(cx);
            self.draw_state.end();
        }
        DrawStep::done()
    }
}
