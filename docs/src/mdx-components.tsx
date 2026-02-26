import defaultMdxComponents from 'fumadocs-ui/mdx';
import type { MDXComponents } from 'mdx/types';
import { withBasePath } from './lib/base-path';
import { cn } from './lib/cn';

export function getMDXComponents(components?: MDXComponents): MDXComponents {
  const defaultImg = defaultMdxComponents.img as
    | ((props: Record<string, unknown>) => React.JSX.Element)
    | undefined;

  return {
    ...defaultMdxComponents,
    img: (props) => {
      const src = typeof props.src === 'string' ? props.src : '';
      const resolvedSrc = withBasePath(src);
      const hasWidth = props.width !== undefined && props.width !== null;
      const hasHeight = props.height !== undefined && props.height !== null;
      const isPlantumlLightSvg =
        src.startsWith('/kubos/diagrams/light/') && src.endsWith('.svg');

      if (isPlantumlLightSvg) {
        const darkSrc = withBasePath(
          src.replace('/kubos/diagrams/light/', '/kubos/diagrams/dark/'),
        );
        const sharedClassName = typeof props.className === 'string' ? props.className : undefined;
        const alt = typeof props.alt === 'string' ? props.alt : '';

        return (
          <span className="block">
            <img
              {...props}
              src={resolvedSrc}
              alt={alt}
              className={cn(sharedClassName, 'dark:hidden')}
            />
            <img
              {...props}
              src={darkSrc}
              alt=""
              aria-hidden="true"
              className={cn(sharedClassName, 'hidden dark:block')}
            />
          </span>
        );
      }

      // Migrated KubOS docs and generated PlantUML SVGs often don't include explicit dimensions.
      // Fallback to a plain img tag in that case to avoid Next Image runtime errors.
      if (!hasWidth || !hasHeight) {
        return <img {...props} src={resolvedSrc} alt={props.alt ?? ''} />;
      }

      if (defaultImg) {
        return defaultImg(props as Record<string, unknown>);
      }

      return <img {...props} src={resolvedSrc} alt={props.alt ?? ''} />;
    },
    ...components,
  };
}
