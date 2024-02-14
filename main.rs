use regex::Regex;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct LyricList {
    quantity: u32,
    lyric: Vec<Lyric>,
}

impl LyricList {
    fn new() -> Self {
        Self {
            quantity: 0,
            lyric: Vec::new(),
        }
    }

    fn add(&mut self, lyric: Lyric) {
        self.lyric.push(lyric);
        self.quantity += 1;
    }
}

#[derive(Debug)]
struct Lyric {
    lrcTranslateLyric: Vec<String>,
    lyric: Vec<String>,
}

impl Lyric {
    fn new(lyrics: Vec<Vec<String>>) -> Self {
        Self {
            lrcTranslateLyric: lyrics[0].clone(),
            lyric: lyrics[1].clone(),
        }
    }
}

impl fmt::Display for Lyric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lyric {{\n  lrc_translate_lyric: {:?},\n  lyric: {:?}\n}}",
            self.lrcTranslateLyric, self.lyric
        )
    }
}

fn parsing_section(statement: &str) -> String {
    //[00:00.36]Amore mio
    let rex = Regex::new(r#"\{"t":(.*),"c":\[\{"tx":"(.*) "\},\{"tx":"(.*)"\}\]\}"#)
        .expect("正则表达式创建失败");

    let mut result = String::new();
    if let Some(caps) = rex.captures(statement) {
        let mut time = caps.get(1).unwrap().as_str();
        let words = caps.get(2).unwrap().as_str();
        let author = caps.get(3).unwrap().as_str();
        let mut temp_str = String::new();

        //🙅🏻位
        if time == "0" {
            time = "[00:00:000]";
        } else if time.chars().next() != Some('0') || time.len() <= 3 {
            temp_str = format!("[00:00.{}]", time);
            time = temp_str.as_str();
        } else {
            time = time;
        }

        result = format!("{}{}{}", time, words, author);
    } else {
        result = statement.to_owned();
    }

    result
}

fn withdraw(lrc: &str, lrcTranslateLyric: &str) -> Vec<Vec<String>> {
    let lyrics_json = json::parse(&lrc).expect("歌词转换json出错");

    let binding_ = lyrics_json["lrcTranslateLyric"].to_string();
    let binding = lyrics_json["lrc"].to_string();

    let lrc_vec_: Vec<_> = binding_.split("\n").collect();
    let lrc_vec: Vec<_> = binding.split("\n").collect();

    vec![
        lrc_vec_
            .into_iter()
            .enumerate()
            .map(|(index, statement)| {
                //考虑大部分歌词所得到的最佳索引3，其他也不是没可能，概况小
                if index < 3 {
                    //println!("{}", statement);
                    parsing_section(statement)
                } else {
                    //println!("{}", statement);
                    statement.to_string()
                }
            })
            .collect(),
        lrc_vec
            .into_iter()
            .enumerate()
            .map(|(index, statement)| {
                
                if index < 3 {
                    //println!("{}", statement);
                    parsing_section(statement)
                } else {
                    //println!("{}", statement);
                    statement.to_string()
                }
            })
            .collect(),
    ]
}

fn batch_processing(folder_path: &str) -> LyricList {
    let mut lyric_list = LyricList::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();

                if file_path.is_file() {
                    if let Ok(file_content) = fs::read_to_string(&file_path) {
                        lyric_list.add(Lyric::new(withdraw(&file_content, &file_content)));
                    } else {
                        println!("读取歌词文件失败");
                    }
                }
            }
        }
    }

    lyric_list
}

fn main() {
    let a = batch_processing("./Lyric");//存放歌词文件夹路径
    for i in 0..a.lyric.len() {
        println!("--------------------翻译----------------------");
        for k in a.lyric[i].lrcTranslateLyric.iter() {
            println!("{}", k);
        }

        println!("--------------------原词----------------------");
        for k in a.lyric[i].lyric.iter() {
            println!("{}", k);
        }

        println!("——————————————————————————————————————————————下一首——————————————————————————————————————————————\n\n\n\n");
    }
}
