use std::path::Path;

use lopdf::Document;
use std::str::pattern::Pattern;

#[cfg(test)]
mod test {
    #[test]
    fn test_something() {
        assert_eq!(3, 3);
    }
}

fn is_found_in<'a, P: Pattern<'a>>(txt: &'a str, pattern: P) -> bool {
    txt.contains(pattern)
}

pub fn search_in_document(path: &Path, pat_vec: &Vec<String>) -> bool {

    let doc = Document::load(path).unwrap();

    // Assumption:
    // Text and abstract don't go past first two pages

    // Get text from the first two pages
    let page_nums = vec![1, 2];
    let txt = doc.extract_text(&page_nums[..]).unwrap();

    // Assumption:
    // Document starts with a title
    // Title doesn't span more than 200 characters
    let title = &txt[..200];


    let mut in_title = false;

    for pat in pat_vec {
        in_title = in_title || is_found_in(title, pat);
    }
    // let in_title =  is_found_in(title, &pat);

    if in_title {
        println!(
            "Pattern: {:?} FOUND in title of: {}",
            pat_vec,
            path.to_str().unwrap()
        );
    } else {
        println!(
            "Pattern: {:?} NOT FOUND in title of: {}",
            pat_vec,
            path.to_str().unwrap()
        );
    }

    let start = txt.find("Abstract").unwrap();
    let stop = txt.find("Introduction").unwrap();

    let maybe_abstract = &txt[start..stop];

    let mut in_abstract = false;

    for pat in pat_vec {
        in_abstract = in_abstract || is_found_in(maybe_abstract, pat);
    }

    if in_abstract {
        println!(
            "Pattern: {:?} FOUND in abstract of: {}",
            pat_vec,
            path.to_str().unwrap()
        );
    } else {
        println!(
            "Pattern: {:?} NOT FOUND in abstract of: {}",
            pat_vec,
            path.to_str().unwrap()
        );
    }

    in_title && in_abstract
}
