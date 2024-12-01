import scala.io.Source
@main 
def part1() = 

    var list1: Vector[Int] = Vector();
    var list2: Vector[Int] = Vector();

    val filename = "/home/axel/Documents/games/aoc24/src/day1in.txt"
    val source = Source.fromFile(filename)
    for (line <- source.getLines()) do
       val lines = line.split(' ')
       list1 = list1.appended(lines(0).toInt);
       list2 = list2.appended(lines(3).toInt);
    
    source.close()

    list1 = list1.sorted()
    list2 = list2.sorted()
    var sum = 0
    // for(i <- list1) do 
    //     sum = sum + i*list2.count( _ == i)
    for(i <- list1.indices) do 
        if list1(i) > list2(i) then
            sum = sum + list1(i) -list2(i)
        else
            sum = sum + list2(i) -list1(i)
    println(sum)


