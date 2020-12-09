/*
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?

Your puzzle answer was 41979.
--- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?

Your puzzle answer was 193416912.
*/

fn main() {
    let data = [
        1977, 1802, 1856, 1309, 2003, 1854, 1898, 1862, 1857, 542, 1616, 1599, 1628, 1511, 1848,
        1623, 1959, 1693, 1444, 1211, 1551, 1399, 1855, 1538, 1869, 1664, 1719, 1241, 1875, 1733,
        1547, 1813, 1531, 1773, 624, 1336, 1897, 1179, 1258, 1205, 1727, 1364, 1957, 540, 1970,
        1273, 1621, 1964, 1723, 1699, 1847, 1249, 1254, 1644, 1449, 1794, 1797, 1713, 1534, 1202,
        1951, 1598, 1926, 1865, 1294, 1893, 1641, 1325, 1432, 1960, 413, 1517, 1724, 1715, 1458,
        1775, 1317, 1694, 1484, 1840, 1999, 1811, 1578, 1658, 1906, 1481, 1313, 1997, 1339, 1592,
        1971, 1453, 1706, 1884, 1956, 1384, 1579, 1689, 1726, 1217, 1796, 1536, 1213, 1867, 1304,
        2010, 1503, 1665, 1361, 814, 2007, 1430, 1625, 1958, 860, 1799, 1942, 1876, 1772, 1198,
        1221, 1814, 1826, 1667, 1334, 1504, 1420, 1164, 1414, 1934, 1823, 1507, 1195, 21, 1752,
        1472, 1196, 1558, 1322, 1927, 1556, 1922, 277, 1828, 1883, 1280, 1947, 1231, 1915, 1235,
        1961, 1494, 1324, 2009, 1367, 1545, 1736, 1575, 1214, 1704, 1833, 1663, 1474, 1894, 1754,
        1564, 1321, 1119, 1975, 1987, 1873, 1834, 1686, 1574, 1505, 1656, 1688, 1896, 1982, 1554,
        1990, 1902, 1859, 1293, 1739, 1282, 1889, 1981, 1283, 1687, 1220, 1443, 1409, 1252, 1506,
        1742, 1319, 1882, 951, 1849,
    ];
    'first: for x in &data {
        for y in &data {
            if x + y == 2020 {
                println!("{}+{}={}\t{}*{}={}", x, y, x + y, x, y, x * y);
                break 'first;
            }
        }
    }

    'second: for x in &data {
        for y in &data {
            for z in &data {
                if x + y + z == 2020 {
                    println!(
                        "{}+{}+{}={}\t{}*{}*{}={}",
                        x,
                        y,
                        z,
                        x + y + z,
                        x,
                        y,
                        z,
                        x * y * z
                    );
                    break 'second;
                }
            }
        }
    }
}
