# Leetcode implementation Option&lt;Box&lt;ListNode&gt;&gt;

Implementation:

```rust,mdbook-runnable
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None, }
    }

    pub fn from_list(values: &[i32]) -> Option<Box<Self>> {
        if values.is_empty() {
            return None;
        }
        let mut head = Box::new(Self::new(values[0]));
        let mut last_node = &mut *head;
        for &val in &values[1..] {
            let new_node = Box::new(Self::new(val));
            last_node.next = Some(new_node);
            last_node = last_node.next.as_mut().unwrap();
        }
        Some(head)
    }

    pub fn to_list(&self) -> Vec<i32> {
        let mut result = vec![];
        let mut first = Some(self);
        while first.is_some() {
            result.push(first.as_ref().unwrap().val);
            first = first.unwrap().next.as_deref();
        }
        result
    }
}
```

