# NODES

```
A detailed explanation of the nodes follows this list of a node's
structure in the wad file.
  Each node is 28 bytes in 14 <short> fields:

(1)  X coordinate of partition line's start
(2)  Y coordinate of partition line's start
(3)  DX, change in X to end of partition line
(4)  DY, change in Y to end of partition line

  If (1) to (4) equaled 64, 128, -64, -64, the partition line would
go from (64,128) to (0,64).

(5)  Y upper bound for right bounding-box.\
(6)  Y lower bound                         All SEGS in right child of node
(7)  X lower bound                         must be within this box.
(8)  X upper bound                        /
(9)  Y upper bound for left bounding box. \
(10) Y lower bound                         All SEGS in left child of node
(11) X lower bound                         must be within this box.
(12) X upper bound                        /
(13) a NODE or SSECTOR number for the right child. If bit 15 of this
       <short> is set, then the rest of the number represents the
       child SSECTOR. If not, the child is a recursed node.
(14) a NODE or SSECTOR number for the left child.

  The NODES lump is by far the most difficult to understand of all the
data structures in DOOM. A new level won't display right without a valid
set of precalculated nodes, ssectors, and segs.
  Here I will explain what the nodes are for, and how they can be
generated automatically from the set of linedefs, sidedefs, and
vertices. I am NOT including any code or a pseudo-code algorithm, like
I do for the BLOCKMAP (appendix [A-3]). This is for reasons of space,
and more importantly, the fact that I haven't written any such
algorithm myself. If there's to be some "node code" published here, it
will have to be donated by someone, well-commented, well-organized, in
pseudo-code, and 100% effective! So the odds are long against it.

  The NODES are branches in a binary space partition (BSP) that divides
up the level and is used to determine which walls are in front of others,
a process know as hidden-surface removal. The SSECTORS (sub-sectors) and
SEGS (segments) lumps are necessary parts of the structure.
  A BSP tree is normally used in 3d space, but DOOM uses a simplified
2d version of the scheme. Basically, the idea is to keep dividing the
map into smaller spaces until each of the smallest spaces contains only
wall segments which cannot possibly occlude (block from view) other
walls in its own space. The smallest, undivided spaces will become
SSECTORS. Each wall segment is part or all of a linedef (and thus a
straight line), and becomes a SEG. All of the divisions are kept track
of in a binary tree structure, which is used to greatly speed the
rendering process (drawing what is seen). How does this binary tree
lead to faster rendering? I have no idea.
  Only the SECTORS need to be divided. The parts of the levels that are
"outside" sectors are ignored. Also, only the walls need to be kept
track of. The sides of any created ssectors which are not parts of
linedefs do not become segs.
  Some sectors do not require any dividing. Consider a square sector.
All the walls are orthogonal to the floor (the walls are all straight
up and down), so from any viewpoint inside the square, none of its
four walls can possibly block the view of any of the others. Now
imagine a sector shaped like this drawing:

+--------------.------+   The * is the viewpoint, looking ->, east. The
|               .     |   diagonal wall marked @ @ can't be seen at all,
|               /\    |@  and the vertical wall marked @@@ is partially
|  *->        /   @\  |@  occluded by the other diagonal wall. This sector
|           /       @\|@  needs to be divided. Suppose the diagonal wall
+---------/               is extended, as shown by the two dots (..):

now each of the two resulting sub-sectors are sufficient, because while
in either one, no wall that is part of that sub-sector blocks any other.
  In general, being a convex polygon is the goal of a ssector. Convex
means a line connecting any two points that are inside the polygon will
be completely contained in the polygon. All triangles and rectangles are
convex, but not all quadrilaterals. In doom's simple Euclidean space,
convex also means that all the interior angles of the polygon are less
than or equal to 180 degrees.
  Now, an additional complication arises because of the two-sided
linedef. Its two sides are in different sectors, so they will end up
in different ssectors too. Thus every two-sided linedef becomes two segs
(or more), or you could say that every sidedef becomes a seg. Creating
segs from sidedefs is a good idea, because the seg can then be associated
with a sector. Two segs that aren't part of the same sector cannot
possibly be in the same ssector, so further division is required of any
set of segs that aren't all from the same sector.
  Whenever a division needs to be made, a SEG is picked, somewhat
arbitrarily, which along with its imaginary extensions, forms a "knife"
that divides the remaining space in two (thus binary). This seg is the
partition line of a node, and the remaining spaces on either side of
the partition line become the right and left CHILDREN of the node. All
partition lines have a direction, and the space on the "right" side of
the partition is the right child of the node; the space on the "left"
is the left child (there's a cute sketch in [4-3]: LINEDEFS that shows
how right and left relate to the start and end of a line). Note that if
there does not exist a seg in the remaining space which can serve as a
partition line, then there is no need for a further partition, i.e.
it's a ssector and a "leaf" on the node tree.
  If the "knife" passes through any lines/segs (but not at vertices),
they are split at the intersection, with one part going to each child.
A two-sided linedef, which is two segs, when split results in four segs.
A two sider that lies along an extension of the partition line has each
of its two segs go to opposite sides of the partition line. This is the
eventual fate of ALL segs on two-sided linedefs.
  As the partition lines are picked and the nodes created, a strict
ordering must be maintained. The node tree is created from the "top"
down. After the first division is made, then the left child is divided,
then its left child, and so on, until a node's child is a ssector. The
n you move back up the tree one branch, and divide the right child, then
its left, etc. ALWAYS left first, on the way down.
  Since there will be splits along the way, there is no way to know
ahead of time how many nodes and ssectors there will be at the end.
And the top of the tree, the node that is created first, is given the
highest number. So as nodes and ssectors are created, they are simply
numbered in order from 0 on up, and when it's all done (nothing's left
but ssectors), then ALL the numbers, for nodes and ssectors, are
reversed. If there's 485 nodes, then 485 becomes 0 and 0 becomes 485.
  Here is another fabulous drawing which will explain everything.
+ is a vertex, - and | indicate linedefs, the . . indicates an
extension of a partition line. The <, >, and ^ symbols indicate the
directions of partition lines. All the space within the drawing is
actual level space, i.e. sectors.

      +-----+-------+-------+            0                     (5)
      |     |       |       |         /     \      ==>       /     \
      |  e  |^  f   |^  g   |       1         4           (4)       (1)
      |     |4      |5      |     /   \      / \         /   \      / \
+---- + . . +-------+-------+    2     3    e   5      (3)   (2)   2  (0)
|     |           < 0       |   / \   / \      / \     / \   / \      / \
|  a  |       b             |  a   b c   d    f   g   6   5 4   3    1   0
|     |^                    |
| . . |2. . . . . +---------+ The order in which      How the elements are
|     |           |1 >        the node tree's         numbered when it's
|  c  |^    d     |           elements get made.      finished.
|     |3          |           0 = node                (5) = node
+-----+-----------+           a = ssector             6 = ssector

  1. Make segs from all the linedefs. There are 5 two-sided lines here.
  2. Pick the vertex at 0 and go west (left). This is the first
       partition line. Note the . . extension line.
  3. Pick the vertex at 1, going east. The backwards extension . . cuts
       the line 3>2>, and the unlabeled left edge line. The left edge
       was one seg, it becomes two. The 3>2> line was two segs, it
       becomes four. New vertices are created at the intersection
       points to do this.
  4. Pick the (newly created) vertex at 2. Now the REMAINING spaces on
       both sides of the partition line are suitable for ssectors. The
       left one is first, it becomes a, the right b. Note that ssector
       a has 3 segs, and ssector b has 5 segs. The . . imaginary lines
       are NOT segs.
  5. Back up the tree, and take 1's right branch. Pick 3. Once again, we
       can make 2 ssectors, c and d, 3 segs each. Back up the tree to 0.
  6. Pick 4. Now the left side is a ssector, it becomes e. But the right
       side is not, it needs one more node. Pick 5, make f and g.
  7. All done, so reverse all the ordination of the nodes and the
       ssectors. Ssector 0's segs become segs 0-2, ssector 1's segs
       become segs 3-7, etc. The segs are written sequentially according
       to the ssector numbering.

  If we want to create an algorithm to do the nodes automatically, it
needs to be able to pick partition lines automatically. From studying
the original maps, it appears that they usually chose a linedef which
divides the child's space roughly in "half". This is restricted by the
availability of a seg in a good location, with a good angle. Also, the
"half" refers to the total number of ssectors in any particular child,
which we have no way of knowing when we start! Optimization methods are
probably used, or maybe brute force, trying every candidate seg until
the "best" one is found.
  What is the best possible choice for a partition line? Well, there
are apparently two goals when creating a BSP tree, which are partially
exclusive. One is to have a balanced tree, i.e. for any node, there are
about the same total number of sub-nodes on either side of it. The other
goal is to minimize the number of "splits" necessary, in this case, the
number of seg splits needed, along with the accompanying new vertices
and extra segs. Only a very primitive and specially constructed set of
linedefs could avoid having any splits, so they are inevitable. It's
just that with some choices of partition lines, there end up being
fewer splits. For example,

+--------------+       If a and b are chosen as partition lines,
|              |       there will be four extra vertices needed,
+---+      +---+ < A   and this shape becomes five ssectors and
    |^    ^|           16 segs. If A and B are chosen, however,
+---+a    b+---+ < B   there are no extra vertices, and only three
|              |       ssectors and 12 segs.
+--------------+

  I've read that for a "small" number of polygons (less than 1000?),
which is what we're dealing with in a doom level, one should definitely
try to minimize splits, and not worry about balancing the tree. I can't
say for sure, but it does appear that the original levels strive for
this. Their trees are not totally unbalanced, but there are some parts
where many successive nodes each have a node and a ssector as children
(this is unbalanced). And there are a lot of examples to prove that the
number of extra segs and vertices they create is very low compared to
what it could be. I think the algorithm that id Software used tried to
optimize both, but with fewer splits being more important.
```
