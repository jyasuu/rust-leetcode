struct Solution;
impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        use std::cmp;
        meetings.sort_by(|a, b| a[0].cmp(&b[0]));
        // println!("{:?}", meetings);
        let mut room_meeting_count = vec![0; n as usize];
        let mut available_rooms = std::collections::BinaryHeap::new();
        for i in 0..n {
            available_rooms.push(cmp::Reverse(i));
        }

        let mut meeting_rooms = std::collections::BinaryHeap::new();
        for i in 0..meetings.len() {
            while let Some(cmp::Reverse((time, available_room))) = meeting_rooms.pop() {
                if time <= meetings[i][0] as i64 {
                    available_rooms.push(cmp::Reverse(available_room));
                } else {
                    meeting_rooms.push(cmp::Reverse((time, available_room)));
                    break;
                }
            }

            if let Some(cmp::Reverse(available_room)) = available_rooms.pop() {
                // println!("{i} {available_room}");
                room_meeting_count[available_room as usize] += 1;
                meeting_rooms.push(cmp::Reverse((meetings[i][1] as i64, available_room)));
                continue;
            }
            while let Some(cmp::Reverse((time, available_room))) = meeting_rooms.pop() {
                if time > meetings[i][0] as i64 {
                    let diff = time - meetings[i][0] as i64;
                    // println!("{i} {available_room}");
                    room_meeting_count[available_room as usize] += 1;
                    meeting_rooms
                        .push(cmp::Reverse((meetings[i][1] as i64 + diff, available_room)));
                } else {
                    // println!("{i} {available_room}");
                    room_meeting_count[available_room as usize] += 1;
                    meeting_rooms.push(cmp::Reverse((meetings[i][1] as i64, available_room)));
                }
                break;
            }
        }
        let mut max_room = 0;
        for i in 0..n as usize {
            if room_meeting_count[max_room] < room_meeting_count[i] {
                max_room = i;
            }
        }

        max_room as i32
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn case1() {
        let actual =
            Solution::most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn case2() {
        let actual = Solution::most_booked(
            3,
            vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]],
        );
        assert_eq!(actual, 1);
    }

    #[test]
    fn case3() {
        let actual =
            Solution::most_booked(2, vec![vec![0, 10], vec![2, 7], vec![1, 5], vec![3, 4]]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn case4() {
        let actual = Solution::most_booked(
            4,
            vec![
                vec![18, 19],
                vec![3, 12],
                vec![17, 19],
                vec![2, 13],
                vec![7, 10],
            ],
        );
        assert_eq!(actual, 0);
    }

}
