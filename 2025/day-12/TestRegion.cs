using System;
using System.Collections.Generic;
using System.Text;

namespace day_12;

public class TestRegion
{
    required public int Wide { get; init; }
    required public int Long { get; init; }
    required public List<int> Counts { get; init; }

    public int Area => Wide * Long;

    public override string ToString() => $"{Wide}x{Long}: {string.Join(' ', Counts)}";
}
