# 1488_Avoid_Flood_in_The_City
# https://leetcode.cn/problems/avoid-flood-in-the-city/description/?envType=daily-question&envId=2023-10-13

from typing import List

# 超时
class Solution:
    def avoidFlood(self, rains: List[int]) -> List[int]:
        # Create a mapping of lake numbers to an incremental index, ignoring 0 (no rain).
        lake_to_index = {}
        index = 1
        for lake in set(rains):
            if lake != 0:
                lake_to_index[lake] = index
                index += 1
        lake_to_index[0] = 0

        # Convert rains to use the new index mapping.
        indexed_rains = [lake_to_index[lake] for lake in rains]

        # Initialize lakes' status and the result list.
        lake_status = [-1] * len(rains)
        result = [-1] * len(indexed_rains)

        for i in range(len(indexed_rains)):
            if indexed_rains[i] != 0:
                # Increase the water level for the lake.
                lake_status[indexed_rains[i]-1] += 1

                # If the lake is overflowing, return an empty list.
                if lake_status[indexed_rains[i]-1] > 0:
                    return []

            else:  # Dry day, we can dry any of the filled lakes.
                dried = False
                for j in range(i, len(rains)):
                    if indexed_rains[j] != 0 and lake_status[indexed_rains[j]-1] != -1:
                        # Dry the lake and update the result.
                        lake_status[indexed_rains[j]-1] -= 1
                        result[i] = indexed_rains[j]
                        dried = True
                        break

                # If no lakes were dried, set the result to an unused lake number.
                if not dried:
                    result[i] = len(rains) + 2

        # Convert the indexed result back to the original lake numbers.
        index_to_lake = {index: lake for lake, index in lake_to_index.items()}
        index_to_lake[-1] = -1
        index_to_lake[len(rains) + 2] = 1

        result = [index_to_lake[index] for index in result]

        return result

s = Solution()
# ans = s.avoidFlood([626831530,87909071,38027148,947809724,693239597,958225392,996593617,427652522,599501396,414395871,508191421,226113378,865756097,589595023,46916859,48528667,390198343,413125086,708525826,750541122,217528164,76670483,164144822,675292501,507879850,84595214,140504117,934217736])
ans = s.avoidFlood([1,2,3,4])
print(ans)

ans = s.avoidFlood([1,2,0,0,2,1])
print(ans)
                    