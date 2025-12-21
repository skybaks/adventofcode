
using day_12;

Shape? currShape = null;
List<Shape> shapes = [];
List<TestRegion> regions = [];
foreach (string line in File.ReadLines("D:\\work\\adventofcode\\2025\\day-12\\input.txt"))
{
    if (line.Contains(':'))
    {
        var splitLine = line.Split(':');
        if (int.TryParse(splitLine[0], out int shapeIndex))
        {
            currShape = new Shape(shapeIndex);
        }
        else
        {
            var splitRegion = splitLine[0].Split('x');
            int regionWide = int.Parse(splitRegion[0]);
            int regionLong = int.Parse(splitRegion[1]);
            var counts = splitLine[1].Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToList();
            regions.Add(new TestRegion { Wide = regionWide, Long = regionLong, Counts = counts });
        }
    }
    else if (string.IsNullOrWhiteSpace(line))
    {
        shapes.Add(currShape!);
        currShape = null;
    }
    else
    {
        currShape!.AddRow(line);
    }
}

int canFit = 0;
foreach (TestRegion region in regions)
{
    // Easy check #1:
    // If the number of coords needed to draw all the shapes is less than
    // available area we can shortcut all further logic... There's no way it
    // could work
    int coordsNeeded = region.Counts.Zip(shapes).Select(x => x.First * x.Second.PointsCount).Sum();
    if (region.Area < coordsNeeded)
    {
        Console.WriteLine($"{region}: shapes need more than available space");
        continue;
    }

    // Easy check #2:
    // If we assume each shape is a rectangle, could they all still fit within
    // the area?
    int rectArea = region.Counts.Zip(shapes).Select(x => x.First * x.Second.RectArea).Sum();
    if (region.Area < rectArea)
    {
        Console.WriteLine($"{region}: RectArea is more than avail");
        continue;
    }

    ++canFit;
}
Console.WriteLine($"canFit:{canFit}");

return;
