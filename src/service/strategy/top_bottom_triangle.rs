use crate::service::technicals::top_bottom_tac::TopBottom;
use crate::{model::open_close::OpenClose, service::technicals::top_bottom_tac::TopBottomType};

pub fn top_bottom_triangle(top_bottoms: &[&TopBottom], minutes: &u32) -> Vec<OpenClose> {
    let mut triangles = Vec::new();
    for i in 0..top_bottoms.len() - 6 {
        let p = [
            top_bottoms.get(i).unwrap(),
            top_bottoms.get(i + 1).unwrap(),
            top_bottoms.get(i + 2).unwrap(),
            top_bottoms.get(i + 3).unwrap(),
            top_bottoms.get(i + 4).unwrap(),
            top_bottoms.get(i + 5).unwrap(),
        ];
        if p[0].type_p == TopBottomType::Bottom
            && p[0].price > p[2].price
            && p[2].price > p[4].price
            && p[1].price < p[3].price
            && p[3].price < p[5].price
        {
            println!("{}", p[5].close_time);
            triangles.push(OpenClose::from_date_close(&p[5].close_time, minutes));
        };
    }
    triangles
}

#[cfg(test)]
mod test {}
