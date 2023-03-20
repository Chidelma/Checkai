from Board import Board
import random
from dotmap import DotMap
import numpy as np
import json
from tensorflow.keras.models import Sequential, load_model
from tensorflow.keras.layers import Dense
from tensorflow.keras.optimizers import Adam
import tensorflowjs as tfjs
import multiprocessing as mp
import time
import os

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '3'

boardStates = []
labelMoves = []

epochs = 100
num_games = 10000


def initData():

    allData = []

    print("Collecting Data")

    with open("./src/dataPointsD3.json", "r") as file:

        allData = json.load(file)

    print(len(allData))

    game = Board()

    for data in allData:

        boardStates.append(game.normalizeBoard(data["board"]))

        for i in range(len(data["moves"])):

            index = data["moves"][i]

            data["moves"][i] = np.zeros((8,), dtype=int)

            data["moves"][i][index] = 1

        labelMoves.append(np.array(data["moves"]).flatten())


def build_model(input_size, output_size):

    print("\nBuilding Network")

    model = Sequential()

    model.add(Dense(input_size / 2, input_dim=input_size, activation="relu"))
    model.add(Dense(output_size * 2, activation="relu"))
    model.add(Dense(((output_size * 2) + (input_size / 2)) / 2, activation="relu"))
    model.add(Dense(output_size, activation="linear"))
    model.compile(loss="mse", optimizer=Adam())

    return model


def train_model():

    print("\nTraining Started")

    xs = np.array(boardStates).reshape(-1, len(boardStates[0]))
    ys = np.array(labelMoves).reshape(-1, len(labelMoves[0]))

    gen = 1

    while True:

        print("\nGeneration", gen)

        try:
            model = load_model("modelD3")
        except:
            model = build_model(len(boardStates[0]), len(labelMoves[0]))

        model.fit(xs, ys, epochs=epochs, shuffle=True)

        predict_model(model)

        model.save("modelD3")
        tfjs.converters.save_keras_model(model, "web_modelD3")

        gen += 1


def predict_model(model):

    correct = 0

    for pred in range(epochs):

        index = random.randrange(0, len(boardStates))

        state = boardStates[index]

        move = np.array(labelMoves[index]).reshape(4, 8)

        xs = np.array(state).reshape(-1, len(state))

        ys = np.array(model.predict(xs)).reshape(4, 8)

        moveArr = []

        ysArr = []

        for i in range(len(ys)):
            ysArr.append(np.where(ys[i] == np.amax(ys[i]))[0][0])

        for i in range(len(move)):
            moveArr.append(np.where(move[i] == np.amax(move[i]))[0][0])

        print()
        print(moveArr, "<==>", ysArr)

        ysArr.sort()
        moveArr.sort()

        if ysArr == moveArr:
            correct += 1

    rate = correct / epochs

    print("\ncorrect rate:", rate, "percent")


def normalize(state):

    state = state.flatten()

    boardState = []

    for i in range(5):

        if i == 0:

            for j in range(len(state)):

                if state[j] == -2:
                    boardState.append(1)
                else:
                    boardState.append(0)

        elif i == 1:

            for j in range(len(state)):

                if state[j] == -1:
                    boardState.append(1)
                else:
                    boardState.append(0)

        elif i == 2:

            for j in range(len(state)):

                if state[j] == 0:
                    boardState.append(1)
                else:
                    boardState.append(0)

        elif i == 3:

            for j in range(len(state)):

                if state[j] == 1:
                    boardState.append(1)
                else:
                    boardState.append(0)

        elif i == 4:

            for j in range(len(state)):

                if state[j] == 2:
                    boardState.append(1)
                else:
                    boardState.append(0)

    return boardState[::-1]


def predict_board(state):

    model = load_model("modelV1")

    xs = np.array(state).reshape(-1, len(state))

    ys = np.array(model.predict(xs)).reshape(4, 8)

    ysArr = []

    for i in range(len(ys)):
        ysArr.append(7 - np.where(ys[i] == np.amax(ys[i]))[0][0])

    return ysArr


def playGame(game_index, start_time, wins, training_data, cacheTable, minaxCache):

    game_memory = []
    prev_game_state = []

    game = Board()
    game.cacheTable = cacheTable
    game.minaxCache = minaxCache
    winner = 0
    done = False

    num_moves = 0

    last_time = start_time

    while not done:
        '''
        num_moves += 1

        if game.curr_player == game.opSide:

            print()

            elapsed_time = time.time() - start_time

            diff_time = elapsed_time - last_time

            last_day = diff_time // (24 * 3600)
            diff_time = diff_time % (24 * 3600)
            last_hour = diff_time // 3600
            diff_time %= 3600
            last_minutes = diff_time // 60
            diff_time %= 60
            last_seconds = diff_time

            last_time = elapsed_time

            day = elapsed_time // (24 * 3600)
            elapsed_time = elapsed_time % (24 * 3600)
            hour = elapsed_time // 3600
            elapsed_time %= 3600
            minutes = elapsed_time // 60
            elapsed_time %= 60
            seconds = elapsed_time

            if game_index > 0:
                print("Current Win Rate:", int((wins/(game_index)) * 100), "percent")

            print("Playing Game:", game_index + 1, "|| Playing Move:", num_moves)

            print()

            print("Move Time:", int(last_day), "Days", int(last_hour), "Hours", int(
                last_minutes), "Minutes", int(last_seconds), "seconds")

            print("Elapsed Time:", int(day), "Days", int(hour), "Hours",
                    int(minutes), "Minutes", int(seconds), "seconds")

            print()

            piece, nextPos = game.getCacheState()

            if piece == None and nextPos == None:

                piece, nextPos = game.ultimateMove()

        elif game.curr_player == game.mySide:

            piece, nextPos = game.bestMove()
        '''

        allPieces = game.allMovablePieces(game.curr_player)

        piece = random.choice(allPieces)

        nextPos = random.choice(game.possibleMoves(piece, None))

        if len(prev_game_state) > 0:

            game_memory.append([game.curr_player, prev_game_state, piece, nextPos])
        
        game.doMove(piece, nextPos)

        prev_game_state = game.state.flatten().tolist()

        done, winner = game.finishState()

        if done:
            cacheTable = game.cacheTable
            minaxCache = game.minaxCache

    if winner == game.opSide:
        wins += 1

    for mem in game_memory:

        if mem[0] == game.opSide and (winner == 0 or winner == game.opSide):

            output = [mem[2].x, mem[2].y, mem[3].x, mem[3].y]

            output = np.array(output).astype(int).tolist()

            training_data.append({"board": mem[1], "move": output})
        
        elif mem[0] == game.mySide and (winner == 0 or winner == game.mySide):

            output = [7 - mem[2].x, 7 - mem[2].y, 7 - mem[3].x, 7 - mem[3].y]

            output = np.array(output).astype(int).tolist()

            training_data.append({"board": mem[1][::-1], "move": output})
        
    #print("Saving Data")

    with open("dataPointsV2.json", 'w') as file:
        json.dump(training_data, file, indent=4)
        file.close()
    '''
    with open("cacheTableD6.json", 'w') as file:
        json.dump(cacheTable, file, indent=4)

    with open("minaxCache.json", 'w') as file:
        json.dump(minaxCache, file, indent=4)
    '''
    return wins, training_data, cacheTable, minaxCache


def self_play(num):

    start_time = time.time()

    wins = 0

    if os.stat("dataPointsV2.json").st_size > 0:
        with open("dataPointsV2.json", 'r') as file:
            training_data = json.load(file)
            file.close
    else:
        training_data = []

    if os.stat("cacheTableD6.json").st_size > 0:
        with open("cacheTableD6.json", "r") as file:
            cacheTable = json.load(file)
            file.close()
    else:
        cacheTable = {}

    if os.stat("minaxCache.json").st_size > 0:
        with open("minaxCache.json", "r") as file:
            minaxCache = json.load(file)
            file.close()
    else:
        minaxCache = {}

    game_index = 0

    while len(training_data) < 1000000:

        if game_index % 1000 == 0:
            print("Games Played:", game_index)
            print("Data Size:", len(training_data))
    
        dataSize = os.stat("dataPointsV2.json").st_size
        time.sleep(1)

        while os.stat("dataPointsV2.json").st_size > dataSize:
            time.sleep(1)
            dataSize = os.stat("dataPointsV2.json").st_size

        wins, training_data, cacheTable, minaxCache = playGame(game_index, start_time, wins, training_data, cacheTable, minaxCache)
        game_index += 1

    print("Win Rate:", (wins/num), "percent")


def testGame():

    game = Board()

    winner = 0
    done = False

    while not done:

        if game.curr_player == game.opSide:

            piece, nextPos = game.ultimateMove()

        elif game.curr_player == game.mySide:

            piece, nextPos = game.bestMove()

        print("\nComputer:", game.opPieces)
        print(game.state)
        print("Player", game.myPieces)

        print()
        print(piece, "==>", nextPos)

        game.doMove(piece, nextPos)

        done, winner = game.finishState()

    if winner == game.opSide:
        print("Computer Wins")
    elif winner == game.mySide:
        print("Player Wins")
    else:
        print("Draw")


# testGame()
#self_play(num_games)
initData()
train_model()
#model = load_model("modelV1")
