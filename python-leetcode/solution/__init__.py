"""
所有专题的预备代码
"""


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
