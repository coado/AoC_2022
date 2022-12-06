


def part_one():
    # 0 index = win
    # 1st index = draw
     # 2sd index = lose
    roundPoints = [6, 3, 0]
    results = {
        'X': ['C', 'A', 'B', 1],
        'Y': ['A', 'B', 'C', 2],
        'Z': ['B', 'C', 'A', 3]
    }

    totalPoints = 0

    with open('data.txt') as f:
        for line in f:
            currentRound = line.split()
            currentRoundResults = results[currentRound[1]]
            # 0 index = win
            # 1st index = draw
            # 2sd index = lose
            index = currentRoundResults.index(currentRound[0])
            totalPoints += currentRoundResults[3] + roundPoints[index]

    return totalPoints

print(part_one())



def part_two():

    # 0 index = win
    # 1st index = draw
    # 2sd index = lose
    results = ['Z', 'Y', 'X']
    roundPoints = [6, 3, 0]

    gameMoves = {
        'A': ['B', 'A', 'C'],
        'B': ['C', 'B', 'A'],
        'C': ['A', 'C', 'B']
    }

    pointsForMove = {
        'A': 1,
        'B': 2,
        'C': 3
    }

    totalPoints = 0

    with open('data.txt') as f:
        for line in f:
            currentRound = line.split()
            # 1) check if I need to win/lose/draw
            indexGameResult = results.index(currentRound[1])
            # 2)  get points for result
            points = roundPoints[indexGameResult]
            # 3) check which move I should use to get above result
            currentMove = gameMoves[currentRound[0]][indexGameResult]    
            # 4) getting points for this move
            points += pointsForMove[currentMove]
            totalPoints += points
    
    return totalPoints


print(part_two())