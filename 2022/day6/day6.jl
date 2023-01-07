cd(@__DIR__)
using Test
using Combinatorics

function diffchars(segment::String)::Bool
    combos = combinations(segment, 2)
    combo1 = [x[1] for x in combos]
    combo2 = [x[2] for x in combos]
    if mapreduce(!=, &, combo1, combo2)
        return(true)
    else
        return(false)
    end
end

function indexstring(line::String, numchar::Int)::Int
    for i in 1:length(line)
        segment = line[i:i+numchar-1]
        if diffchars(segment)
            return(i+numchar-1)
        end
    end
end

function day6(infile::String, numchar::Int)
    open(infile, "r") do f
        line = readline(f)
        indexstring(line, numchar)
    end
end

@test indexstring("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)==7
@test indexstring("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)==5
@test indexstring("nppdvjthqldpwncqszvftbrmjlhg", 4)==6
@test indexstring("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)==10
println("Day 6 Part 1: ", day6("input.txt", 4))

@test indexstring("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)==19
@test indexstring("bvwbjplbgvbhsrlpgdmjqwftvncz", 14)==23
@test indexstring("nppdvjthqldpwncqszvftbrmjlhg", 14)==23
@test indexstring("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)==29
@test indexstring("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)==26
println("Day 6 Part 2: ", day6("input.txt", 14))