For getting the surface area of the 3D shape, we can add the surface area contributed by each block. A block can be in a corner, side or in the middle ( surrounded by 4 blocks ).

We can get the surface area contributed by a single bar from its height and the height of neighbouring blocks. For blocks in the corner or side, the height of the missing neighbouring block can be considered as 0.

Surface area of one block would be an addition of areas of 6 sides: top, bottom, up, down, left, right.

The top and bottom portion is occupied by every block which is 1 units² each. For neighbouring blocks, consider the height of the current bar as ‘x’ units and height of the neighboring block as ‘y’ units, the surface area would be exposed only if x > y. Otherwise the surface area of that side would be hidden.

We add surface area contributed by each block to get surface area occupied by the 3D figure.