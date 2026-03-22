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
      const srcFromObject =
        typeof props.src === 'object' &&
        props.src !== null &&
        'src' in props.src &&
        typeof (props.src as { src?: unknown }).src === 'string'
          ? ((props.src as { src: string }).src ?? '')
          : '';
      const src =
        typeof props.src === 'string'
          ? props.src.trim()
          : props.src instanceof URL
            ? props.src.toString().trim()
            : srcFromObject.trim();

      if (!src) {
        return null;
      }

      const resolvedSrc = withBasePath(src);
      const imageMeta =
        typeof props.src === 'object' && props.src !== null ? (props.src as Record<string, unknown>) : null;
      const normalizedWidth =
        props.width ?? (typeof imageMeta?.width === 'number' ? imageMeta.width : undefined);
      const normalizedHeight =
        props.height ?? (typeof imageMeta?.height === 'number' ? imageMeta.height : undefined);
      const hasWidth = normalizedWidth !== undefined && normalizedWidth !== null;
      const hasHeight = normalizedHeight !== undefined && normalizedHeight !== null;
      const normalizedProps = {
        ...props,
        src: resolvedSrc,
        width: normalizedWidth,
        height: normalizedHeight,
      };
      const normalizedNextSrc =
        imageMeta && typeof imageMeta.src === 'string'
          ? {
              ...imageMeta,
              src: resolvedSrc,
            }
          : resolvedSrc;
      const normalizedNextProps = {
        ...props,
        src: normalizedNextSrc,
        width: normalizedWidth,
        height: normalizedHeight,
      };
      const isPlantumlLightSvg =
        src.startsWith('/kubos/diagrams/light/') && src.endsWith('.svg');
      const isDrawioLightSvg =
        src.startsWith('/kubos/diagrams/drawio/light/') && src.endsWith('.svg');

      if (isPlantumlLightSvg || isDrawioLightSvg) {
        const darkSrc = withBasePath(
          isPlantumlLightSvg
            ? src.replace('/kubos/diagrams/light/', '/kubos/diagrams/dark/')
            : src.replace('/kubos/diagrams/drawio/light/', '/kubos/diagrams/drawio/dark/'),
        );
        const sharedClassName = typeof props.className === 'string' ? props.className : undefined;
        const alt = typeof props.alt === 'string' ? props.alt : '';

        return (
          <span className="block">
            <img
              {...normalizedProps}
              src={resolvedSrc}
              alt={alt}
              className={cn(sharedClassName, 'dark:hidden')}
            />
            <img
              {...normalizedProps}
              src={darkSrc}
              alt=""
              aria-hidden="true"
              className={cn(sharedClassName, 'hidden dark:block')}
            />
          </span>
        );
      }

      // Migrated KubOS docs and generated diagram SVGs often don't include explicit dimensions.
      // Fallback to a plain img tag in that case to avoid Next Image runtime errors.
      if (!hasWidth || !hasHeight) {
        return <img {...normalizedProps} src={resolvedSrc} alt={props.alt ?? ''} />;
      }

      if (defaultImg) {
        const nextImgProps = { ...normalizedNextProps } as Record<string, unknown>;
        const nextImgSrc =
          typeof nextImgProps.src === 'object' && nextImgProps.src !== null
            ? (nextImgProps.src as Record<string, unknown>)
            : null;
        const hasBlurData =
          typeof nextImgProps.blurDataURL === 'string' ||
          typeof nextImgSrc?.blurDataURL === 'string';

        if (nextImgProps.placeholder === 'blur' && !hasBlurData) {
          delete nextImgProps.placeholder;
        }

        return defaultImg(nextImgProps);
      }

      return <img {...normalizedProps} src={resolvedSrc} alt={props.alt ?? ''} />;
    },
    ...components,
  };
}
