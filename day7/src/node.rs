use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Node {
    Dir(BTreeMap<String, Node>),
    File(usize),
}

impl Node {
    pub fn traverse<I: Iterator<Item = String>>(
        &mut self,
        iter: &mut std::iter::Peekable<I>,
    ) -> Result<(), String> {
        let dirs = match self {
            Node::Dir(d) => d,
            Node::File(_) => return Err("Unable to traverse folder.".to_string()),
        };

        loop {
            let line = match iter.next() {
                Some(line) => line,
                None => return Ok(()),
            };

            let cmd = line
                .strip_prefix("$ ")
                .ok_or_else::<String, _>(|| format!("Unable to read command on {}.", line))?;

            // println!("Command: {cmd}");

            if cmd == "ls" {
                while iter.peek().map(|s| !s.starts_with("$ ")).unwrap_or(false) {
                    let ls_result = iter.next().unwrap();
                    let (name, node) = if let Some(dir) = ls_result.strip_prefix("dir ") {
                        (dir.to_owned(), Node::Dir(Default::default()))
                    } else {
                        let mut words = ls_result.split(" ");
                        let size = words
                            .next()
                            .ok_or_else(|| format!("Unable to read size {}.", ls_result))?
                            .parse::<usize>()
                            .unwrap();
                        let name = words
                            .next()
                            .ok_or_else(|| format!("Unable to read file name"))?;
                        (name.to_owned(), Node::File(size))
                    };

                    if dirs.insert(name.clone(), node).is_some() {
                        return Err(format!("Added node {name} more than once."));
                    }
                }
            } else if let Some(target_dir) = cmd.strip_prefix("cd ") {
                if target_dir == ".." {
                    break;
                }
                dirs.get_mut(target_dir)
                    .ok_or_else(|| format!("Unknown directory {target_dir}."))?
                    .traverse(iter)?;
            }
        }

        Ok(())
    }

    pub fn calculate_size(&self, sizes: &mut Vec<usize>) -> usize
    {
        match self {
            Node::Dir(d) => {
                let size = d.values().map(|v| v.calculate_size(sizes)).sum();
                sizes.push(size);
                size
            },
            Node::File(s) => *s,
        }
    }

}
