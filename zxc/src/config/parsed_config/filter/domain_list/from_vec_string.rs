use tracing::trace;

use super::*;

/* Steps:
 *      1. Separate wildcard list and string list
 *      2. Convert wildcard list to Wildcard
 */

impl From<Vec<String>> for DomainList {
    fn from(v: Vec<String>) -> Self {
        // 1. Separate string list and wildcard list
        let (wildcard, mut str): (Vec<_>, Vec<_>) = v
            .into_iter()
            .partition(|val| val.contains('*'));

        trace!("wildcard| {:?}", wildcard);

        // 2. Build wildcard list
        let wildcard = wildcard
            .into_iter()
            .filter_map(|val| match Wildcard::from_owned(val.into()) {
                Ok(r) => Some(r),
                Err(e) => {
                    error!("Error converting to wildcard| {}", e);
                    None
                }
            })
            .collect::<Vec<Wildcard<'static>>>();

        DomainList {
            str: if str.is_empty() {
                None
            } else {
                str.sort_unstable();
                Some(str)
            },
            wildcard: if wildcard.is_empty() {
                None
            } else {
                Some(wildcard)
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_vec_to_domain_list() {
        let vec = vec![
            "*.reddit.com".to_string(),
            "www.google.com".to_string(),
            "www.youtube.com".to_string(),
        ];
        let domain_list = DomainList::from(vec);
        assert_eq!(
            domain_list.str,
            Some(vec![
                "www.google.com".to_string(),
                "www.youtube.com".to_string()
            ])
        );

        //assert_eq!(
        //    domain_list.wildcard,
        //    Some(vec![Wildcard::from_owned("*.reddit.com").unwrap()])
        //);

        assert!(domain_list.contains(&"www.reddit.com".to_string()));
        assert!(domain_list.contains(&"chat.reddit.com".to_string()));
        assert!(domain_list.contains(&"www.google.com".to_string()));
        assert!(domain_list.contains(&"www.youtube.com".to_string()));
        assert!(!domain_list.contains(&"www.bing.com".to_string()));

        let wild = domain_list.wildcard.unwrap();
        assert!(wild[0].is_match(b"www.reddit.com"));
    }
}
