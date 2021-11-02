// On an infinite plane, a robot initially stands at (0, 0) and faces north. The robot can receive one of three instructions:

//     "G": go straight 1 unit;
//     "L": turn 90 degrees to the left;
//     "R": turn 90 degrees to the right.

// The robot performs the instructions given in order, and repeats them forever.

// Return true if and only if there exists a circle in the plane such that the robot never leaves the circle.

// Example 1:

// Input: instructions = "GGLLGG"
// Output: true
// Explanation: The robot moves from (0,0) to (0,2), turns 180 degrees, and then returns to (0,0).
// When repeating these instructions, the robot remains in the circle of radius 2 centered at the origin.

// Example 2:

// Input: instructions = "GG"
// Output: false
// Explanation: The robot moves north indefinitely.

// Example 3:

// Input: instructions = "GL"
// Output: true
// Explanation: The robot moves from (0, 0) -> (0, 1) -> (-1, 1) -> (-1, 0) -> (0, 0) -> ...

// Constraints:

//     1 <= instructions.length <= 100
//     instructions[i] is 'G', 'L' or, 'R'.
struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut position = (0, 0);
        let mut direction = (0, 1);

        for i in instructions.chars() {
            if i == 'G' {
                position = (position.0 + direction.0, position.1 + direction.1)
            } else if i == 'L' {
                direction = (-direction.1, direction.0);
            } else if i == 'R' {
                direction = (direction.1, -direction.0);
            }
        }

        return position == (0, 0) || direction != (0, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let instructions = "GGLLGG".to_string();
        let result = Solution::is_robot_bounded(instructions);

        assert_eq!(result, true);
    }
}
