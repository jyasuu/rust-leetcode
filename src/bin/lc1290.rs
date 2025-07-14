// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution;
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut ans = 0 ;
        let mut temp = head;
        while let Some(node) = temp
        {
            ans <<= 1;
            ans |= node.val;
            temp = node.next;
        }
        ans
    }
}

fn main()
{

}


#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn case1()
    {
        assert_eq!(Solution::get_decimal_value(Some(Box::new(ListNode::new(1)))),1);
    }
    
    #[test]
    fn case2()
    {
        let data = Some(Box::new(ListNode::new(1)));
        let data =  Some(Box::new(ListNode{next:data,val:0}));
        let data =  Some(Box::new(ListNode{next:data,val:1}));

        assert_eq!(Solution::get_decimal_value(data),5);
    }
}
