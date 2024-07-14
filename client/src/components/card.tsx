type CardProps = {
  content: any;
  subContent?: any;
  externalLink?: string | undefined | null;
  imageUrl?: string | undefined | null;
  usePlaceholder?: boolean;
};

export default function Card({
  content,
  subContent = null,
  externalLink = null,
  imageUrl = null,
  usePlaceholder = false,
}: CardProps) {
  return (
    <div className="flex py-2 min-w-0">
      <img className="object-cover h-auto w-12" src={imageUrl || ""} alt="" />

      <div className="flex flex-col min-w-0 ml-2">
        <h3 className="overflow-hidden text-ellipsis whitespace-nowrap">
          {content}
        </h3>
        <p className="text-sm overflow-hidden text-ellipsis whitespace-nowrap">
          {subContent}
        </p>
      </div>
    </div>
  );
}
