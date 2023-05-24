import networkx as nx
import matplotlib.pyplot as plt

def backtrack(node, depth, max_depth, max_value, graph):
    if depth == max_depth:
        return
    for value in range(node[-1]+1, max_value+1):
        child = node + (value,)
        graph.add_node(child)
        graph.add_edge(node, child)
        backtrack(child, depth+1, max_depth, max_value, graph)

def plot_tree(tree):
    pos = nx.drawing.nx_pydot.graphviz_layout(tree, prog='dot')
    nx.draw(tree, pos, with_labels=True, node_color='lightblue')
    plt.show()

max_depth = 3
max_value = 7
root = (0,)
tree = nx.DiGraph()
tree.add_node(root)
backtrack(root, 0, max_depth, max_value, tree)
plot_tree(tree)