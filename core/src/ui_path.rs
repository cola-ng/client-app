use makepad_widgets::*;
use makepad_component::*;

fn view_ref_from_view(view: &View, path: &[LiveId]) -> ViewRef {
    let mut current = makepad_widgets::View::view(view, path[0]);
    for id in &path[1..] {
        current = makepad_widgets::ViewRef::view(&current, *id);
    }
    current
}

fn view_ref_from_view_ref(view: &ViewRef, path: &[LiveId]) -> ViewRef {
    let mut current = makepad_widgets::ViewRef::view(view, path[0]);
    for id in &path[1..] {
        current = makepad_widgets::ViewRef::view(&current, *id);
    }
    current
}

pub trait ViewPathExt {
    fn view(&self, path: &[LiveId]) -> ViewRef;
    fn button(&self, path: &[LiveId]) -> ButtonRef;
    fn label(&self, path: &[LiveId]) -> LabelRef;
    fn text_input(&self, path: &[LiveId]) -> TextInputRef;
    fn drop_down(&self, path: &[LiveId]) -> DropDownRef;
    fn link_label(&self, path: &[LiveId]) -> LinkLabelRef;
    fn markdown(&self, path: &[LiveId]) -> MarkdownRef;
    fn page_flip(&self, path: &[LiveId]) -> PageFlipRef;
    fn portal_list(&self, path: &[LiveId]) -> PortalListRef;
}

impl ViewPathExt for View {
    fn view(&self, path: &[LiveId]) -> ViewRef {
        view_ref_from_view(self, path)
    }

    fn button(&self, path: &[LiveId]) -> ButtonRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::ButtonWidgetExt::button(self, *last)
        } else {
            makepad_widgets::ButtonWidgetExt::button(&view_ref_from_view(self, parent), *last)
        }
    }

    fn label(&self, path: &[LiveId]) -> LabelRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::LabelWidgetExt::label(self, *last)
        } else {
            makepad_widgets::LabelWidgetExt::label(&view_ref_from_view(self, parent), *last)
        }
    }

    fn text_input(&self, path: &[LiveId]) -> TextInputRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::TextInputWidgetExt::text_input(self, *last)
        } else {
            makepad_widgets::TextInputWidgetExt::text_input(
                &view_ref_from_view(self, parent),
                *last,
            )
        }
    }

    fn drop_down(&self, path: &[LiveId]) -> DropDownRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::DropDownWidgetExt::drop_down(self, *last)
        } else {
            makepad_widgets::DropDownWidgetExt::drop_down(&view_ref_from_view(self, parent), *last)
        }
    }

    fn link_label(&self, path: &[LiveId]) -> LinkLabelRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::LinkLabelWidgetExt::link_label(self, *last)
        } else {
            makepad_widgets::LinkLabelWidgetExt::link_label(
                &view_ref_from_view(self, parent),
                *last,
            )
        }
    }

    fn markdown(&self, path: &[LiveId]) -> MarkdownRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::MarkdownWidgetExt::markdown(self, *last)
        } else {
            makepad_widgets::MarkdownWidgetExt::markdown(&view_ref_from_view(self, parent), *last)
        }
    }

    fn page_flip(&self, path: &[LiveId]) -> PageFlipRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::PageFlipWidgetExt::page_flip(self, *last)
        } else {
            makepad_widgets::PageFlipWidgetExt::page_flip(&view_ref_from_view(self, parent), *last)
        }
    }

    fn portal_list(&self, path: &[LiveId]) -> PortalListRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::PortalListWidgetExt::portal_list(self, *last)
        } else {
            makepad_widgets::PortalListWidgetExt::portal_list(
                &view_ref_from_view(self, parent),
                *last,
            )
        }
    }
}

impl ViewPathExt for ViewRef {
    fn view(&self, path: &[LiveId]) -> ViewRef {
        view_ref_from_view_ref(self, path)
    }

    fn button(&self, path: &[LiveId]) -> ButtonRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::ButtonWidgetExt::button(self, *last)
        } else {
            makepad_widgets::ButtonWidgetExt::button(&view_ref_from_view_ref(self, parent), *last)
        }
    }

    fn label(&self, path: &[LiveId]) -> LabelRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::LabelWidgetExt::label(self, *last)
        } else {
            makepad_widgets::LabelWidgetExt::label(&view_ref_from_view_ref(self, parent), *last)
        }
    }

    fn text_input(&self, path: &[LiveId]) -> TextInputRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::TextInputWidgetExt::text_input(self, *last)
        } else {
            makepad_widgets::TextInputWidgetExt::text_input(
                &view_ref_from_view_ref(self, parent),
                *last,
            )
        }
    }

    fn drop_down(&self, path: &[LiveId]) -> DropDownRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::DropDownWidgetExt::drop_down(self, *last)
        } else {
            makepad_widgets::DropDownWidgetExt::drop_down(
                &view_ref_from_view_ref(self, parent),
                *last,
            )
        }
    }

    fn link_label(&self, path: &[LiveId]) -> LinkLabelRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::LinkLabelWidgetExt::link_label(self, *last)
        } else {
            makepad_widgets::LinkLabelWidgetExt::link_label(
                &view_ref_from_view_ref(self, parent),
                *last,
            )
        }
    }

    fn markdown(&self, path: &[LiveId]) -> MarkdownRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::MarkdownWidgetExt::markdown(self, *last)
        } else {
            makepad_widgets::MarkdownWidgetExt::markdown(
                &view_ref_from_view_ref(self, parent),
                *last,
            )
        }
    }

    fn page_flip(&self, path: &[LiveId]) -> PageFlipRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::PageFlipWidgetExt::page_flip(self, *last)
        } else {
            makepad_widgets::PageFlipWidgetExt::page_flip(
                &view_ref_from_view_ref(self, parent),
                *last,
            )
        }
    }

    fn portal_list(&self, path: &[LiveId]) -> PortalListRef {
        let (last, parent) = path.split_last().unwrap();
        if parent.is_empty() {
            makepad_widgets::PortalListWidgetExt::portal_list(self, *last)
        } else {
            makepad_widgets::PortalListWidgetExt::portal_list(
                &view_ref_from_view_ref(self, parent),
                *last,
            )
        }
    }
}
