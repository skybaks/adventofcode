using System;
using System.Collections.Generic;
using System.Text;

namespace day_12;

public record Coordinate2D
{
    required public int X { get; set; }
    required public int Y { get; set; }

    public override string ToString() => $"{X},{Y}";
}
