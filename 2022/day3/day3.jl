cd(@__DIR__)

# Int64('a') = 97
# lwoercase priorities 1-26
# Int64('A') = 65
# uppercase priorities 27-52
open("input.txt", "r") do f
    score = 0
    for ln in eachline(f)
        spl = Int64(length(ln)/2) # should always be even based on problem phrasing
        ln = split(ln, "")
        firsthalf = ln[1:spl]
        secondhalf = ln[spl+1:end]
        overlap = intersect(firsthalf, secondhalf)
        for c in overlap
            c = first(c)
            if isuppercase(c)
                score += Int64(c)-38
            else
                score += Int64(c)-96
            end
        end
    end
    println("score part 1: ", score)
end

# Part 2
# reading lines from https://stackoverflow.com/questions/58169711/how-to-read-a-file-line-by-line-in-julia
open("input.txt", "r") do f
    score = 0
    while !eof(f)
        elf1 = readline(f)
        elf2 = readline(f)
        elf3 = readline(f)
        badge = intersect(elf1, elf2, elf3) |> first
        if isuppercase(badge)
            score += Int64(badge)-38
        else
            score += Int64(badge)-96
        end
    end
    println("score part 2: ", score)
end