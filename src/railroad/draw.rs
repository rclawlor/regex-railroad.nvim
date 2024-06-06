///  Methods for locating a node relative to others, alongside
///  rendering the node
pub trait Draw {
    /// The number of lines from this element's top to where the entering,
    /// connecting path is drawn.
    fn entry_height(&self) -> usize;

    /// This primitives's total height.
    fn height(&self) -> usize;

    /// This primitive's total width.
    fn width(&self) -> usize;

    /// Draw this element.
    fn draw(&self) -> Vec<String>;
}

impl std::fmt::Debug for dyn Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Draw")
            .field("entry_height", &self.entry_height())
            .field("height", &self.height())
            .field("width", &self.width())
            .finish()
    }
}

impl<'a, N> Draw for &'a N
where
    N: Draw + ?Sized,
{
    fn entry_height(&self) -> usize {
        (**self).entry_height()
    }

    fn height(&self) -> usize {
        (**self).height()
    }

    fn width(&self) -> usize {
        (**self).width()
    }

    fn draw(&self) -> Vec<String> {
        (**self).draw()
    }
}

impl<N> Draw for Box<N>
where
    N: Draw + ?Sized,
{
    fn entry_height(&self) -> usize {
        (**self).entry_height()
    }

    fn height(&self) -> usize {
        (**self).height()
    }

    fn width(&self) -> usize {
        (**self).width()
    }

    fn draw(&self) -> Vec<String> {
        (**self).draw()
    }
}

impl<'a, N> Draw for &'a mut N
where
    N: Draw + ?Sized,
{
    fn entry_height(&self) -> usize {
        (**self).entry_height()
    }

    fn height(&self) -> usize {
        (**self).height()
    }

    fn width(&self) -> usize {
        (**self).width()
    }

    fn draw(&self) -> Vec<String> {
        (**self).draw()
    }
}

pub trait DrawGroup {
    /// The maximum `entry_height()`-value.
    fn max_entry_height(self) -> usize;

    /// The maximum `height()`-value.
    fn max_height(self) -> usize;

    /// The maximum `width()`-value.
    fn max_width(self) -> usize;

    /// The sum of all `width()`-values.
    fn total_width(self) -> usize;

    /// The sum of all `height()`-values.
    fn total_height(self) -> usize;
}

impl<I, N> DrawGroup for I
where
    I: IntoIterator<Item = N>,
    N: Draw,
{
    fn max_entry_height(self) -> usize {
        self.into_iter()
            .map(|n| n.entry_height())
            .max()
            .unwrap_or_default()
    }

    fn max_height(self) -> usize {
        self.into_iter()
            .map(|n| n.height())
            .max()
            .unwrap_or_default()
    }

    fn max_width(self) -> usize {
        self.into_iter()
            .map(|n| n.width())
            .max()
            .unwrap_or_default()
    }

    fn total_width(self) -> usize {
        self.into_iter().map(|n| n.width()).sum()
    }

    fn total_height(self) -> usize {
        self.into_iter().map(|n| n.height()).sum()
    }
}
