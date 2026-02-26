import { createHash } from 'node:crypto';

type MdastNode = {
  type: string;
  children?: MdastNode[];
  lang?: string | null;
  value?: string;
  url?: string;
  alt?: string;
  title?: string | null;
};

function isPlantUmlBlock(node: MdastNode): node is MdastNode & { value: string } {
  if (node.type !== 'code' || typeof node.value !== 'string') return false;
  const value = node.value;
  return value.includes('@startuml') && value.includes('@enduml');
}

function plantUmlHash(source: string): string {
  return createHash('sha256').update(source.replace(/\r\n/g, '\n')).digest('hex').slice(0, 16);
}

function diagramUrl(source: string): string {
  return `/kubos/diagrams/${plantUmlHash(source)}.svg`;
}

function visit(node: MdastNode) {
  if (!node.children) return;

  for (let i = 0; i < node.children.length; i += 1) {
    const child = node.children[i];

    if (isPlantUmlBlock(child)) {
      node.children[i] = {
        type: 'image',
        url: diagramUrl(child.value),
        alt: 'PlantUML diagram',
        title: null,
      };
      continue;
    }

    visit(child);
  }
}

export function remarkPlantumlStaticImages() {
  return (tree: MdastNode) => {
    visit(tree);
  };
}
