#[derive(Copy, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub const sys_colors: [RGB; 64] = [
    RGB{r: 84, g: 84, b: 84}, RGB{r: 0, g: 30, b: 116}, RGB{r: 8, g: 16, b: 144}, RGB{r: 48, g: 0, b: 136}, RGB{r: 68, g: 0, b: 100}, RGB{r: 92, g: 0, b: 48}, RGB{r: 84, g: 4, b: 0}, RGB{r: 60, g: 24, b: 0}, 
    RGB{r: 32, g: 42, b: 0}, RGB{r: 8, g: 58, b: 0}, RGB{r: 0, g: 64, b: 0}, RGB{r: 0, g: 60, b: 0}, RGB{r: 0, g: 50, b: 60}, RGB{r: 0, g: 0, b: 0}, RGB{r: 152, g: 150, b: 152}, RGB{r: 8, g: 76, b: 196}, 
    RGB{r: 48, g: 50, b: 236}, RGB{r: 92, g: 30, b: 228}, RGB{r: 136, g: 20, b: 176}, RGB{r: 160, g: 20, b: 100}, RGB{r: 152, g: 34, b: 32}, RGB{r: 120, g: 60, b: 0}, RGB{r: 84, g: 90, b: 0}, RGB{r: 40, g: 114, b: 0}, 
    RGB{r: 8, g: 124, b: 0}, RGB{r: 0, g: 118, b: 40}, RGB{r: 0, g: 102, b: 120}, RGB{r: 0, g: 0, b: 0}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, 
    RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, 
    RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, 
    RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, 
    RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, RGB{r: 1, g: 2, b: 3}, 
];