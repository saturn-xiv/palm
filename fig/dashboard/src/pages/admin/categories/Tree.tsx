import { Tree } from "antd";
import type { TreeDataNode } from "antd";

import { ICategory } from "../../../api/daffodil";

export const ROOT = "root";

interface IProps {
  nodes: ICategory[];
  item?: ICategory;
}

const fetch_tree_node = (start: number, items: ICategory[]): TreeDataNode => {
  const cur = items.filter((x) => x.left === start)[0];

  const node: TreeDataNode = {
    key: cur.id,
    title: cur.code,
  };
  if (cur.right === cur.left + 1) {
    return node;
  }

  node.children = [];
  for (let i = cur.left + 1; i < items.length * 2; ) {
    const tmp = items.filter((x) => x.left === i);
    if (tmp.length === 1) {
      const it = tmp[0];
      if (it.right == it.left + 1) {
        node.children.push({ key: it.id, title: it.code });
      } else {
        node.children.push(fetch_tree_node(it.left, items));
      }

      i = it.right + 1;
    } else {
      break;
    }
  }

  return node;
};

// https://gist.github.com/tmilos/f2f999b5839e2d42d751
const Widget = ({ item, nodes }: IProps) => {
  return (
    <Tree
      showLine
      showIcon
      autoExpandParent
      treeData={
        nodes.length > 0 ? [fetch_tree_node(item?.left || 1, nodes)] : []
      }
    />
  );
};

export default Widget;
