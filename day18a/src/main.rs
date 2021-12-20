use serde::{Deserialize, Serialize};
use std::fs;
use std::ops::Add;
use std::str::FromStr;

const EXPLODE_DEPTH: usize = 4;

type Value = u8;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TreeNode {
    Leaf(Value),
    Branch(Box<TreeNode>, Box<TreeNode>),
}

impl TreeNode {
    fn decode(s: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }

    fn encode(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    fn as_leaf_value(&self) -> Option<Value> {
        match self {
            TreeNode::Leaf(value) => Some(*value),
            _ => None,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            TreeNode::Leaf(value) => {
                if *value >= 10 {
                    let div = *value as f64 / 2_f64;
                    *self = TreeNode::Branch(
                        Box::from(TreeNode::Leaf(div.floor() as Value)),
                        Box::from(TreeNode::Leaf(div.ceil() as Value)),
                    );
                    true
                } else {
                    false
                }
            }
            TreeNode::Branch(left_box, right_box) => left_box.split() || right_box.split(),
        }
    }

    fn add_left(&mut self, carry: Option<Value>) {
        if carry.is_none() {
            return;
        }

        match self {
            TreeNode::Leaf(value) => *value += carry.unwrap(),
            TreeNode::Branch(left_box, _) => left_box.add_left(carry),
        }
    }

    fn add_right(&mut self, carry: Option<Value>) {
        if carry.is_none() {
            return;
        }

        match self {
            TreeNode::Leaf(value) => *value += carry.unwrap(),
            TreeNode::Branch(_, right_box) => right_box.add_right(carry),
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<Value>, Option<Value>) {
        match self {
            TreeNode::Leaf(_) => (false, None, None),
            TreeNode::Branch(left_box, right_box) => {
                if depth == 0 {
                    let left_value = left_box.as_leaf_value();
                    let right_value = right_box.as_leaf_value();
                    *self = TreeNode::Leaf(0);
                    (true, left_value, right_value)
                } else {
                    let (changed, left_value, right_value) = left_box.explode(depth - 1);
                    if changed {
                        right_box.add_left(right_value);
                        return (true, left_value, None);
                    }
                    let (changed, left_value, right_value) = right_box.explode(depth - 1);
                    if changed {
                        left_box.add_right(left_value);
                        return (true, None, right_value);
                    }
                    (false, None, None)
                }
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            TreeNode::Leaf(value) => *value as usize,
            TreeNode::Branch(left_box, right_box) => {
                3 * left_box.magnitude() + 2 * right_box.magnitude()
            }
        }
    }
}

impl FromStr for TreeNode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<TreeNode> {
        Self::decode(s)
    }
}

impl Add<Self> for TreeNode {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut root = TreeNode::Branch(Box::from(self), Box::from(other));

        loop {
            if root.explode(EXPLODE_DEPTH).0 {
                continue;
            }

            if !root.split() {
                break;
            }
        }

        root
    }
}

fn sum_trees(roots: Vec<TreeNode>) -> TreeNode {
    roots.into_iter().reduce(|prev, cur| prev + cur).unwrap()
}

fn parse_trees(s: &str) -> Vec<TreeNode> {
    s.lines()
        .map(|line| TreeNode::decode(line).unwrap())
        .collect()
}

fn main() {
    let s = fs::read_to_string("input").unwrap();
    let root = sum_trees(parse_trees(&s));
    dbg!(root.encode().unwrap());
    dbg!(root.magnitude());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_explode(before: &str, after: &str) {
        let mut root = TreeNode::decode(before).unwrap();
        root.explode(EXPLODE_DEPTH);
        assert_eq!(root.encode().unwrap(), after);
    }

    #[test]
    fn test_explode() {
        assert_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        assert_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        assert_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        assert_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        assert_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    fn assert_split(before: &str, after: &str) {
        let mut root = TreeNode::decode(before).unwrap();
        root.split();
        assert_eq!(root.encode().unwrap(), after);
    }

    #[test]
    fn test_split() {
        assert_split(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        assert_split(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
    }

    fn assert_addition(a: &str, b: &str, expected: &str) {
        assert_eq!(
            (TreeNode::decode(a).unwrap() + TreeNode::decode(b).unwrap())
                .encode()
                .unwrap(),
            expected
        );
    }

    #[test]
    fn test_addition() {
        assert_addition(
            "[[[[4,3],4],4],[7,[[8,4],9]]]",
            "[1,1]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
        assert_addition(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );
        assert_addition(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
        );
        assert_addition(
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
        );
        assert_addition(
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
        );
        assert_addition(
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
        );
        assert_addition(
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            "[2,9]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
        );
        assert_addition(
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
        );
        assert_addition(
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
        );
        assert_addition(
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            "[[[[4,2],2],6],[8,7]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }

    fn assert_magnitude(encoded: &str, expected: usize) {
        assert_eq!(TreeNode::decode(encoded).unwrap().magnitude(), expected);
    }

    #[test]
    fn test_magnitude() {
        assert_magnitude("[[1,2],[[3,4],5]]", 143);
        assert_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        assert_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        assert_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        assert_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        assert_magnitude(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
    }

    fn assert_sum(s: &str, combined: &str) {
        assert_eq!(sum_trees(parse_trees(s)).encode().unwrap(), combined);
    }

    #[test]
    fn test_sum() {
        assert_sum(
            "[1,1]\n[2,2]\n[3,3]\n[4,4]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
        assert_sum(
            "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );
        assert_sum(
            "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );

        let slightly_larger_example = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ]
        .join("\n");

        assert_sum(
            &slightly_larger_example,
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }

    #[test]
    fn test_example() {
        let s = fs::read_to_string("input-test").unwrap();
        let root = sum_trees(parse_trees(&s));
        assert_eq!(
            root.encode().unwrap(),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
        assert_eq!(root.magnitude(), 4140);
    }
}
