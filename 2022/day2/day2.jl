cd(@__DIR__)

# Part 1 solution
# A = rock, B = paper, C = scissor
# X = rock, Y = paper, Z = scissor
open("input.txt", "r") do f
    score = 0
    for ln in eachline(f)
        if ln == "A X"
            # rock-rock tie, 1+3=4
            score += 4
        elseif ln == "A Y"
            # rock-paper win, 2+6=8
            score += 8
        elseif ln == "A Z"
            # rock-scissor lose, 3+0=3
            score += 3
        elseif ln == "B X"
            # paper-rock lose, 1+0 = 1
            score += 1
        elseif ln == "B Y"
            # paper-paper draw, 2+3=5
            score += 5
        elseif ln == "B Z"
            # paper-scissor win, 3+6=9
            score += 9
        elseif ln == "C X"
            # scissor-rock win, 1+6 = 7
            score += 7
        elseif ln == "C Y"
            # scissor-paper lose, 2+0 = 2
            score += 2
        elseif ln == "C Z"
            # scissor-scissor draw, 3+3=6
            score += 6
        end
    end
    println("score part 1: ", score)
end

# Part 2 solution
# A = rock, B = paper, C = scissor
# X = lose, Y = draw, Z = win
open("input.txt", "r") do f
    score = 0
    for ln in eachline(f)
        if ln == "A X"
            # rock, lose with scissor, 3 + 0 = 3
            score += 3
        elseif ln == "A Y"
            # rock draw rock, 1 + 3 = 4
            score += 4
        elseif ln == "A Z"
            # rock, win with paper, 2 + 6 = 8
            score += 8
        elseif ln == "B X"
            # paper, lose with rock, 1 + 0 = 1
            score += 1
        elseif ln == "B Y"
            # paper-paper draw, 2+3=5
            score += 5
        elseif ln == "B Z"
            # paper, win with scissor, 3+6=9
            score += 9
        elseif ln == "C X"
            # scissor, lose with paper, 2 + 0 = 2
            score += 2
        elseif ln == "C Y"
            # scissor, draw with scissor, 3 + 3 = 6
            score += 6
        elseif ln == "C Z"
            # scissor, win with rock, 1+6 = 7
            score += 7
        end
    end
    println("score part 2: ", score)
end