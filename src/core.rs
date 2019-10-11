
struct Tag {
    name: String,
    value: Value
}

struct Value {
    text: String
}

pub fn generate_headers<'a>(s: &'a String, col_lin: usize) -> Vec<&str> {
    let mut vec: Vec<&str> = s.split('\n').collect();
    vec = vec[col_lin].split(',').collect();

    vec
}

pub fn generate_rows<'a>(s: &'a String) -> Vec<&str> {
    let rows: Vec<&str> = s.split('\n').collect();

    let headers: Vec<&str> = generate_headers(s,0);

    println!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>");
    println!("<document>");
    for i in 1..rows.len() - 1 {
        let rowsf: Vec<&str> = rows[i].split(',').collect();
        println!("\t<row>");
        for x in 0..rowsf.len() {
            let tags: Vec<String> = tagify(headers[x]);
            println!("\t\t{0} {1} {2}", tags[0],rowsf[x], tags[1]);
        }
        println!("\t</row>");
    }
    println!("</document>");

    rows
}

pub fn print_version() -> (){
    println!("<!-- \n\nXEngine {} v0.1.0\n\n-->", '\u{1F682}');
}

pub fn tagify<'a>(t: &'a str) -> Vec<String> {
    let mut tags: Vec<String> = vec![];

    let mut tag: String = String::from("<");
    tag.push_str(&t.to_string()); tag.push_str(">");
    tags.push(tag.to_lowercase().replace(" ", "_"));

    tag = String::from("</");
    tag.push_str(&t.to_string());tag.push_str(">");
    tags.push(tag.to_lowercase().replace(" ", "_"));


    tags
}
