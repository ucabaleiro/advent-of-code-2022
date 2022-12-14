use std::vec;

use utils::input::get_input;

trait FileSystemElement {
    fn get_size(&self) -> usize;
    fn from_line(line: &String) -> Self;
}

#[derive(Debug)]
struct SizedFile {
    _name: String,
    size: usize,
}

impl FileSystemElement for SizedFile {
    fn get_size(&self) -> usize {
        self.size
    }

    fn from_line(line: &String) -> Self {
        let (size, name) = line.split_once(' ').expect(&("malformed input line corresponding to a file:  ".to_string() + line));
        let size = size.parse::<usize>().expect(&("malformed input line corresponding to a file:  ".to_string() + line));
        Self { _name: name.to_string(), size: size }
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<SizedFile>,
    dirs: Vec<Dir>,
}

impl FileSystemElement for Dir {
    fn get_size(&self) -> usize {
        self.files.iter().map(SizedFile::get_size).sum::<usize>() 
            + self.dirs.iter().map(Self::get_size).sum::<usize>()
    }

    fn from_line(line: &String) -> Self {
        let (_, name) = line.split_once(' ').expect(&("malformed input line corresponding to a dir:  ".to_string() + line));
        Self { name: name.to_string(), files: Vec::new(), dirs: Vec::new() }
    }
}

impl Dir {
    fn load(mut self, lines: &mut impl Iterator<Item = String>) -> Dir {
        while let Some(line) = lines.next() {
            if !line.starts_with("$") {
                if line.starts_with("dir") {
                     self.add_dir(Dir::from_line(&line))
                } else { 
                    self.add_file(SizedFile::from_line(&line));
                }
            }
            if line.starts_with("$ cd ..") { return self}
            else if line.starts_with("$ cd ") {
                let loaded = self.get_subdir_with_name(&Dir::get_name_from_cd_line(&line)).load(lines);
                self.add_dir(loaded);
            }
        }
        self
    }

    fn get_subdir_with_name(&mut self, name: &String) -> Dir{
        let index = self.dirs.iter().position(|dir| dir.name == *name)
            .expect(&format!("cant find dir of name '{}' in dir named '{}'", self.name, name));
        self.dirs.swap_remove(index)
            
    }

    fn get_name_from_cd_line(line: &String) -> String{
        let name = line.split(' ').last().expect(&format!("malformed cd line: {line}"));
        name.to_string()
    }

    fn add_file(&mut self, file: SizedFile) {
        self.files.push(file);
    }

    fn add_dir(&mut self, dir: Dir) {
        self.dirs.push(dir);
    }

    fn get_sizes_by_dir(&self) -> Vec<(String, usize)> {
        let mut ret = Vec::new();

        ret.push((self.name.to_string(), self.get_size()));

        let mut inner = self.dirs.iter().flat_map(Dir::get_sizes_by_dir).collect();
        ret.append(&mut inner);
        ret
    }

    fn get_sizes(&self) -> Vec<usize> {
        self.get_sizes_by_dir().into_iter().map(|(_, size)| size).collect()
    }

    fn _test_data() -> Dir {
        Dir {
            name: "d1".to_string(),
            files: vec![
                SizedFile { _name: "f1".to_string(), size: 100},
                SizedFile { _name: "f2".to_string(), size: 100},
            ],
            dirs: vec![
                Dir {
                    name: "d2".to_string(),
                    files: vec![
                        SizedFile { _name: "f3".to_string(), size: 100},
                        SizedFile { _name: "f4".to_string(), size: 100},
                    ],
                    dirs: vec![]
                },
                Dir {
                    name: "d3".to_string(),
                    files: vec![
                        SizedFile { _name: "f5".to_string(), size: 100},
                        SizedFile { _name: "f6".to_string(), size: 100},
                    ],
                    dirs: vec![
                        Dir {
                            name: "d4".to_string(),
                            files: vec![
                                SizedFile { _name: "f7".to_string(), size: 100},
                                SizedFile { _name: "f8".to_string(), size: 100},
                            ],
                            dirs: vec![]
                        },
                        Dir {
                            name: "d5".to_string(),
                            files: vec![
                                SizedFile { _name: "f9".to_string(), size: 100},
                                SizedFile { _name: "f10".to_string(), size: 50},
                            ],
                            dirs: vec![
                                Dir {
                                    name: "d6".to_string(),
                                    files: vec![],
                                    dirs: vec![
                                        Dir {
                                            name: "d7".to_string(),
                                            files: vec![],
                                            dirs: vec![]
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    }

}



fn main() {
    let mut input_lines = get_input().into_iter();
    input_lines.next();

    let root = Dir::from_line(&"dir /".to_string());

    let root = root.load(&mut input_lines);

    let total_size_of_dirs_smaller_than_100_000 = root.get_sizes().into_iter()
        .filter(|size| *size <= 100_000).sum::<usize>();
    
    println!("[AOC07] Part A - sum of sizes of dirs smaller than 100_000:   {total_size_of_dirs_smaller_than_100_000}");

    // part B
    const TOTAL_SPACE: usize = 70_000_000;
    let necessary_space: usize= 30_000_000;
    let space_to_free = root.get_size() - (TOTAL_SPACE - necessary_space);

    let samllest_dir_to_delete = root.get_sizes().into_iter()
        .filter(|size| *size >= space_to_free).min().expect("more than 1 dir must be deleted");
    
    println!("[AOC07] Part B - total size of directory to delete:    {samllest_dir_to_delete}");

}
