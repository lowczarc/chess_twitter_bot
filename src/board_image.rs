use std::{collections::HashMap, path::Path};

use chess::{Board, Color, File, Piece, Rank, Square};
use image::{self, DynamicImage, ImageBuffer, Rgba};

pub type ImageBuff8 = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Hash, PartialEq, Eq)]
pub enum AssetKey {
    Board,
    Piece((Color, Piece)),
}

pub fn construct_assets() -> HashMap<AssetKey, ImageBuff8> {
    let mut assets = HashMap::new();

    assets.insert(
        AssetKey::Board,
        image::open(Path::new("assets/chess_board.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::Black, Piece::King)),
        image::open(Path::new("assets/black_king.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::Black, Piece::Queen)),
        image::open(Path::new("assets/black_queen.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::Black, Piece::Bishop)),
        image::open(Path::new("assets/black_bishop.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::Black, Piece::Knight)),
        image::open(Path::new("assets/black_knight.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::Black, Piece::Rook)),
        image::open(Path::new("assets/black_rook.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::Black, Piece::Pawn)),
        image::open(Path::new("assets/black_pawn.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::White, Piece::King)),
        image::open(Path::new("assets/white_king.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::White, Piece::Queen)),
        image::open(Path::new("assets/white_queen.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::White, Piece::Bishop)),
        image::open(Path::new("assets/white_bishop.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::White, Piece::Knight)),
        image::open(Path::new("assets/white_knight.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::White, Piece::Rook)),
        image::open(Path::new("assets/white_rook.png"))
            .unwrap()
            .to_rgba(),
    );
    assets.insert(
        AssetKey::Piece((Color::White, Piece::Pawn)),
        image::open(Path::new("assets/white_pawn.png"))
            .unwrap()
            .to_rgba(),
    );

    assets
}

fn copy_image_into_another(dest: &mut ImageBuff8, src: &ImageBuff8, x: u32, y: u32) {
    if y + src.height() > dest.height() || x + src.width() > dest.width() {
        panic!("dest overflow in copy_image_into_another");
    }
    for i in 0..src.height() {
        for j in 0..src.width() {
            let src_pixel = src.get_pixel(j, i);
            let dest_pixel = dest.get_pixel(j + x, i + y);
            let opacity = src_pixel[3] as f32 / 255.;
            let new_pixel = [
                (src_pixel[0] as f32 * opacity + dest_pixel[0] as f32 * (1. - opacity)) as u8,
                (src_pixel[1] as f32 * opacity + dest_pixel[1] as f32 * (1. - opacity)) as u8,
                (src_pixel[2] as f32 * opacity + dest_pixel[2] as f32 * (1. - opacity)) as u8,
                255,
            ];
            if src_pixel[3] != 0 {
                dest.put_pixel(j + x, i + y, Rgba { data: new_pixel });
            }
        }
    }
}

pub fn create_image(assets: &HashMap<AssetKey, ImageBuff8>, board: Board, name: &str) {
    let mut board_image = assets.get(&AssetKey::Board).unwrap().clone();

    for i in 0..8 {
        for j in 0..8 {
            let square = Square::make_square(Rank::from_index(7 - i), File::from_index(j));
            if let (Some(piece), Some(color)) = (board.piece_on(square), board.color_on(square)) {
                copy_image_into_another(
                    &mut board_image,
                    assets.get(&AssetKey::Piece((color, piece))).unwrap(),
                    56 + 114 * j as u32,
                    56 + 114 * i as u32,
                );
            } else {
            }
        }
    }
    DynamicImage::ImageRgba8(board_image)
        .save(name)
        .expect("Failed to save image");
}
