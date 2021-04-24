use crate::model::open_close_time::OpenCloseTime;
use crate::services::technicals::top_bottom::TopBottom;
use crate::services::technicals::top_bottom::TopBottomType;

pub fn top_bottom_triangle(top_bottoms: &[&TopBottom], minutes: i32) -> Vec<OpenCloseTime> {
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
            triangles.push(OpenCloseTime::from_date_close(&p[5].close_time, minutes));
        };
    }
    triangles
}

#[cfg(test)]
mod test {}
