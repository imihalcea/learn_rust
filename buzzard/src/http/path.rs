pub const SEPARATOR:char = '/';

#[derive(Debug)]
pub struct Path<'buf>
{
    fragments: Vec<&'buf str>,
    value:&'buf str
}


impl<'buf> From<&'buf str> for Path<'buf> {
    fn from(value: &'buf str) -> Self {
        let mut fragments = vec![];
        for fragment in value.split(SEPARATOR){
            if fragment != ""{
                fragments.push(fragment);
            }
        }
        Self{
            fragments,
            value
        }
    }
}

impl<'buf> Path<'buf> {
    pub fn contains_fragment(&self, fragment:&str) -> bool{
        self.fragments.contains(&fragment)
    }

    pub fn matches_exactly(&self, fragment:&str) -> bool{
        self.value.eq(fragment)
    }

    pub fn matches(&self, fragment:&str) -> bool{
        false
    }

    pub fn matches_path(&self, other:&Path) -> bool{
        false
    }
}


#[cfg(test)]
mod tests{
    use super::Path;

    #[test]
    fn create_from_str_nominal(){
        let path = Path::from("/a/b/c");
        assert_eq!(path.fragments, vec!["a","b","c"]);
    }

    #[test]
    fn create_from_empty(){
        let path = Path::from("");
        assert!(path.fragments.is_empty());
    }
}