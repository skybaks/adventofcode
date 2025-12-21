using System;
using System.Collections.Generic;
using System.Text;

namespace day_12;

public class Shape
{
    public int Index { get; }
    public int PointsCount => m_points.Count;
    public int RectArea => m_rowNum * m_colMax;
    private List<Coordinate2D> m_points;
    private int m_rowNum;
    private int m_colMax;

    public Shape(int index)
    {
        Index = index;
        m_points = [];
        m_rowNum = 0;
    }

    public void AddRow(string shapeRow)
    {
        int colNum = 0;
        foreach (var c in shapeRow.ToCharArray())
        {
            if (c == '#')
            {
                m_points.Add(new Coordinate2D { X = colNum, Y = (m_rowNum * -1) });
            }
            ++colNum;
            m_colMax = Math.Max(m_colMax, colNum);
        }
        ++m_rowNum;
    }
}
