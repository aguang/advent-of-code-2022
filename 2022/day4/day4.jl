cd(@__DIR__)

open("input.txt", "r") do f
    fullycontained = 0
    for ln in eachline(f)
        sections = split(ln, ('-',','))
        sections = parse.(Int64, sections)
        if sections[1] <= sections[3] && sections[2] >= sections[4]
            fullycontained += 1
        elseif sections[1] >= sections[3] && sections[2] <= sections[4]
            fullycontained += 1
        end
    end
    println("fullycontained part 1: ", fullycontained)
end

# part 2
open("input.txt", "r") do f
    overlaps = 0
    for ln in eachline(f)
        sections = split(ln, ('-',','))
        sections = parse.(Int64, sections)
        if sections[1] <= sections[3] && sections[2] >= sections[3]
            overlaps += 1
        elseif sections[1] >= sections[3] && sections[1] <= sections[4]
            overlaps += 1
        end
    end
    println("overlaps part 1: ", overlaps)
end