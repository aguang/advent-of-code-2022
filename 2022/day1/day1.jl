using BenchmarkTools
cd(@__DIR__)

# Part 1 solution
open("input.txt", "r") do f
    max = 0
    sum = 0
    for ln in eachline(f)
        if ln == ""
            if sum > max
                max = sum
            end
            sum = 0
        else
            sum += parse(Int64,ln)
        end
    end
    println("max: ", max)
end

# Part 2 solution
# Think doing it as we go will be faster than computing all sums and then sorting
# but we can also check
# It does appear slightly faster!
function day1_part2()
    open("input.txt", "r") do f
        max1 = 0
        max2 = 0
        max3 = 0
        sum = 0
        for ln in eachline(f)
            if ln == ""
                if sum > max1
                    if sum > max2
                        if sum > max3
                            max1 = max2
                            max2 = max3
                            max3 = sum
                        else
                            max1 = max2
                            max2 = sum
                        end
                    else
                        max1 = sum
                    end
                end
                sum = 0
            else
                sum += parse(Int64,ln)
            end
        end
        return(max1+max2+max3)
    end
end
@btime day1_part2()
println("top 3 from day1_part2: ", day1_part2())

function day1_part2_sort()
    calories = Int64[]
    open("input.txt", "r") do f
        sum = 0
        for ln in eachline(f)
            if ln == ""
                push!(calories, sum)
                sum = 0
            else
                sum += parse(Int64,ln)
            end
        end
    end
    sort!(calories, rev=true)
    top3 = calories[1]+calories[2]+calories[3]
    return(top3)
end
@btime day1_part2_sort()
println("top 3 from day1_part2_sort(): ", day1_part2_sort())