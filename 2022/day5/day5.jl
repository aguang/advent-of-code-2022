cd(@__DIR__)
using Test

function cratebuild!(numcrates::Int, ln::String, stacks::Vector{Vector{String}})
    for i in 1:numcrates
        crate = ln[i*4-3:i*4-1]
        if isnothing(findfirst(isspace,crate))
            crate = strip(crate, ['[',']'])
            try
                pushfirst!(stacks[i], crate)
            catch
                stacks[i] = [crate]
            end
        end
    end
end

function day5_part1(infile::String)
    open(infile, "r") do f
        lines = eachline(f)

        # determine number of crates for construction
        initial = first(lines)
        l = length(initial)
        numcrates = convert(Int, (l+1)/4)
        stacks = Vector{Vector{String}}(undef,numcrates)
        instructions_flag = 0
        cratebuild!(numcrates, initial, stacks)

        for ln in eachline(f)
            if occursin(" 1   2", ln) || ln == ""
                instructions_flag = 1
                continue
            end

            # crates
            if instructions_flag == 0
                cratebuild!(numcrates, ln, stacks)
            # instructions
            else
                sections = split(ln)
                number = parse(Int,sections[2])
                from = parse(Int,sections[4])
                to = parse(Int, sections[6])
                for i in 1:number
                    crate = pop!(stacks[from])
                    push!(stacks[to],crate)
                end
            end
        end
        
        # get crate tops
        s = join([last(stacks[i]) for i in 1:numcrates])
        return(s)
    end
end


@test day5_part1("test_input.txt") == "CMZ"
println(day5_part1("input.txt"))

# part 2
function day5_part2(infile::String)
    open(infile, "r") do f
        lines = eachline(f)

        # determine number of crates for construction
        initial = first(lines)
        l = length(initial)
        numcrates = convert(Int, (l+1)/4)
        stacks = Vector{Vector{String}}(undef,numcrates)
        instructions_flag = 0
        cratebuild!(numcrates, initial, stacks)

        for ln in eachline(f)
            if occursin(" 1   2", ln) || ln == ""
                instructions_flag = 1
                continue
            end

            # crates
            if instructions_flag == 0
                cratebuild!(numcrates, ln, stacks)
            # instructions
            else
                sections = split(ln)
                number = parse(Int,sections[2])
                from = parse(Int,sections[4])
                to = parse(Int, sections[6])
                e = lastindex(stacks[from])
                crates = splice!(stacks[from],e-number+1:e)
                append!(stacks[to],crates)
            end
        end
        
        # get crate tops
        s = join([last(stacks[i]) for i in 1:numcrates])
        return(s)
    end
end


@test day5_part2("test_input.txt") == "MCD"
println("Part 2: ", day5_part2("input.txt"))