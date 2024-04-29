pub fn list_up(list_cur: &mut usize) {
    if (*list_cur > 0) {
        *list_cur -= 1
    }
    println!("{}", &list_cur);
}
pub fn list_down(list: &mut Vec<String>, list_cur: &mut usize) {
    if (*list_cur + 1 < list.len()) {
        *list_cur += 1;
        *list_cur = *list_cur.clamp(&mut 0, &mut (list.len() - 1));
    }
    println!("{}", &list_cur);
}

pub fn list_remove(list: &mut Vec<String>, list_cur: &mut usize, list_add: &mut Vec<String>) {
    if (list_cur < &mut list.len()) {
        let mut max_list: usize = list.len() - 1;
        list_add.push(list.remove(*list_cur));
        if (*list_cur > 0) {
            *list_cur = *list_cur.clamp(&mut 0, &mut max_list) - 1;
        } else {
            *list_cur = *list_cur.clamp(&mut 0, &mut max_list);
        }
    }
    println!("{}", *list_cur);
}
